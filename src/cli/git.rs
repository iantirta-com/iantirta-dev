use std::{
    error::Error,
    path::{Path, PathBuf},
    process::Command,
    sync::mpsc,
    thread,
    io::Write
};

use clap::{Args, Subcommand};
use log::{debug, trace};
use owo_colors::{Style, OwoColorize};

use crate::{
    cli::{
        Action
    },
    config::{
        Config
    }
};

#[derive(Args, Debug)]
pub struct GitCommand {
    #[command(subcommand)]
    action: GitAction
}


#[derive(Subcommand, Debug)]
pub enum GitAction {
    /// Check for uncommitted changes and branch sync status across all projects
    CheckAll,
    // CommitAll,
    // Commit
}

// Actionable method for git command
impl GitCommand {
    fn check_all(&self, config: &Config) -> Result<i32, Box<dyn Error>> {
        let project_dir = config.project_dir();

        // Println! so that its outside log
        println!("Checking git status across workspaces...");
        println!("Workspace: '{}'\n", project_dir.display());

        let (tx, rx) = mpsc::channel();
        let mut checked_count = 0;

        for entry in project_dir.read_dir()? {
            if let Ok(entry) = entry {
                let path = entry.path();
                let file_name = entry.file_name().to_str().unwrap().to_string();

                if path.is_dir() {
                    // Check if it is a git repository
                    let git_dir = path.join(".git");
                    if git_dir.exists() && git_dir.is_dir() {
                        checked_count += 1;

                        let tx_clone = tx.clone();
                        thread::spawn(move || {
                            // Run the git command and format the string
                            let result = Self::check_repo_status(&path, &file_name);
                            // Send the tuple (repo_name, formatted_output) back
                            tx_clone.send(result).unwrap();
                        });
                    }
                }
            }
        }

        let mut results = Vec::with_capacity(checked_count);
        for _ in 0..checked_count {
            if let Ok(res) = rx.recv() {
                results.push(res);
            }
        }
        results.sort_by(|a, b| a.0.cmp(&b.0));

        let mut dirty_repos = Vec::new();
        let mut behind_repos = Vec::new();
        let mut ahead_repos = Vec::new();

        for (name, path, output, is_dirty, behind_count, ahead_count) in results {
            println!("{}", output);

            if behind_count > 0 {
                behind_repos.push((name.clone(), path.clone()));
            }
            if is_dirty {
                dirty_repos.push((name, path));
            } else if ahead_count > 0 {
                // If it's dirty, we will prompt to push during the commit phase instead.
                // Only add to ahead_repos if it is clean but has unpushed commits.
                ahead_repos.push((name.clone(), path.clone()));
            }
        }

        if checked_count == 0 {
            println!("No valid git projects found in '{}'", project_dir.display().bright_yellow());
        } else {
            println!("\nChecked {} repositories.", checked_count.bright_cyan());
        }

        if !behind_repos.is_empty() {
            print!("\nFound {} repository(s) behind remote. Pull changes? [y/N]: ", behind_repos.len().bright_yellow());
            std::io::stdout().flush()?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            
            if input.trim().eq_ignore_ascii_case("y") {
                for (name, path) in &behind_repos {
                    println!("  {} {}...", "⬇ Pulling".bright_blue(), name.bold());
                    Command::new("git").arg("pull").current_dir(path).status()?;
                }
            }
        }

        if !dirty_repos.is_empty() {
            print!("\nFound {} repository(s) with uncommitted changes. Review and commit? [y/N]: ", dirty_repos.len().bright_yellow());
            std::io::stdout().flush()?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            
            if input.trim().eq_ignore_ascii_case("y") {
                self.interactive_commit(dirty_repos)?;
            }
        }

        if !ahead_repos.is_empty() {
            print!("\nFound {} repository(s) with unpushed commits. Push changes? [y/N]: ", ahead_repos.len().bright_yellow());
            std::io::stdout().flush()?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            
            if input.trim().eq_ignore_ascii_case("y") {
                for (name, path) in &ahead_repos {
                    println!("  {} {}...", "⬆ Pushing".bright_green(), name.bold());
                    Command::new("git").arg("push").current_dir(path).status()?;
                }
            }
        }

        Ok(0)
    }

    fn interactive_commit(&self, dirty_repos: Vec<(String, PathBuf)>) -> Result<(), Box<dyn Error>> {
        for (name, path) in dirty_repos {
            println!("\n{}", "==================================================".bright_cyan());
            println!("📝 Reviewing: {}", name.bold().bright_yellow());
            println!("{}", "==================================================".bright_cyan());

            // 1. Show the user what actually changed (Short Status)
            let status = Command::new("git")
                .arg("diff")
                .arg("--minimal")
                .arg("--color=always")
                .current_dir(&path)
                .output()?;

            println!("{}", String::from_utf8_lossy(&status.stdout));

            // 2. Prompt for commit message
            print!("Enter commit message (Leave empty to skip, 'q' to abort all remaining): ");
            std::io::stdout().flush()?;
            
            let mut msg = String::new();
            std::io::stdin().read_line(&mut msg)?;
            let msg = msg.trim();

            if msg.eq_ignore_ascii_case("q") {
                println!("{}", "Aborting remaining commits.".bright_red());
                break;
            }

            if !msg.is_empty() {
                // 3. git add .
                Command::new("git")
                    .arg("add")
                    .arg(".")
                    .current_dir(&path)
                    .status()?;

                // 4. git commit -m "..."
                let commit = Command::new("git")
                    .arg("commit")
                    .arg("-m")
                    .arg(msg)
                    .current_dir(&path)
                    .status()?;

                if commit.success() {
                    println!("{} {}", "✓".green(), "Committed successfully!".green());
                } else {
                    println!("{} {}", "✗".red(), "Commit failed.".red());
                }
            } else {
                println!("{}", "Skipped.".bright_yellow());
            }
        }
        Ok(())
    }
}


/// Git method wrapper
impl GitCommand {
    fn check_repo_status(path: &Path, name: &str) -> (String, PathBuf, String, bool, u32, u32) {
        // We use `--porcelain -b` to get machine-readable output including branch ahead/behind status
        let output = Command::new("git")
            .arg("status")
            .arg("--porcelain")
            .arg("-b")
            .current_dir(path)
            .output();

        let mut final_is_dirty = false;
        let mut final_behind_count = 0;
        let mut final_ahead_count = 0;

        let formatted_result = match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                
                let mut is_dirty: bool = false;
                let mut behind_count = 0;
                let mut ahead_count = 0;

                let mut lines = stdout.lines();

                // The first line always contains branch info because of `-b`
                // Example: "## main...origin/main [behind 1]"
                let mut branch_name = String::new();

                if let Some(branch_line) = lines.next() {
                    let content = branch_line.trim_start_matches("## ");
                    let end_idx = content.find("...").or_else(|| content.find(' ')).unwrap_or(content.len());
                    branch_name = content[..end_idx].to_string();

                    if let Some(idx) = branch_line.find("ahead ") {
                        let tail = &branch_line[idx + 6..];
                        let end = tail.find(|c: char| !c.is_ascii_digit()).unwrap_or(tail.len());
                        ahead_count = tail[..end].parse::<u32>().unwrap_or(0);
                    }

                    if let Some(idx) = branch_line.find("behind ") {
                        let tail = &branch_line[idx + 7..];
                        let end = tail.find(|c: char| !c.is_ascii_digit()).unwrap_or(tail.len());
                        behind_count = tail[..end].parse::<u32>().unwrap_or(0);
                    }
                }

                // If there are ANY lines after the branch line, the working tree is dirty
                if lines.next().is_some() {
                    is_dirty = true;
                    final_is_dirty = true;
                }

                let status_color: Style= Style::new();
                let dirty_ui = if is_dirty { "Has changes" } else { "Clean" };

                let mut sync_ui = String::new();
                if behind_count > 0 { 
                    let msg = format!("- Behind {}", behind_count);
                    sync_ui.push_str(&format!("{} ", msg.bright_yellow())); 
                }
                if ahead_count > 0 { 
                    let msg = format!("+ Ahead {}", ahead_count);
                    sync_ui.push_str(&format!("{} ", msg.bright_blue())); 
                }
                if behind_count == 0 && ahead_count == 0 {
                    sync_ui = format!("{}", "Synced".green());
                }
                final_behind_count = behind_count;
                final_ahead_count = ahead_count;

                format!(
                    "{}...## {}\n  Status: {}\n  Remote: {}",
                    name.bold(),
                    branch_name,
                    dirty_ui.style(if is_dirty { status_color.red() } else { status_color.green() }),
                    sync_ui,
                )
            }
            Ok(output) if !output.status.success() => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                format!("  {}\n    Failed to execute git\n    Error: {}", name.bold(), stderr.trim()).red().to_string()
            },
            _ => {
                format!("  {}\n    Failed to execute git\n    Unknown Error", name.bold()).red().to_string()
            }
        };

        (name.to_string(), path.to_path_buf(), formatted_result, final_is_dirty, final_behind_count, final_ahead_count)
    }
}


impl Action for GitCommand {
    fn run(&self, config: &Config) -> Result<i32, Box<dyn Error>> {
        match self.action {
            GitAction::CheckAll => self.check_all(config),
        }
    }
}
