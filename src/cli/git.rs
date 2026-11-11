use std::{
    error::Error,
    process::Command,
    sync::mpsc,
    thread,
    io::Write
};

use clap::{Args, Subcommand};
use owo_colors::{OwoColorize};

use crate::{
    cli::Action, config::Config, service::Git
};


/// Git command for cli
#[derive(Args, Debug)]
pub struct GitCommand {
    #[command(subcommand)]
    action: GitAction
}


/// Git subcommmands for cli
#[derive(Subcommand, Debug)]
pub enum GitAction {

    /// Check for uncommitted changes and branch sync status across all projects
    CheckAll,
}

// Actionable method for git command
impl GitCommand {
    /// Helper for user input cli
    fn input(&self, msg: &str) -> Result<String, Box<dyn Error>> {
        print!("{}", msg);
        std::io::stdout().flush()?;

        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input)?;

        Ok(user_input)
    }

    fn yes_input(&self, msg: &str) -> Result<bool, Box<dyn Error>> {
        let user_input = self.input(msg)?;
        if user_input.trim().eq_ignore_ascii_case("y") ||
            user_input.trim().eq_ignore_ascii_case("yes")
        {
            return Ok(true)
        };
        Ok(false)
    }

    fn no_input(&self, msg: &str) -> Result<bool, Box<dyn Error>> {
        let user_input = self.input(msg)?;
        if user_input.trim().eq_ignore_ascii_case("n") ||
            user_input.trim().eq_ignore_ascii_case("no")
        {
            return Ok(true)
        };
        Ok(false)
    }

    fn check_all(&self, config: &Config) -> Result<i32, Box<dyn Error>> {
        let project_dir = config.project_dir();

        // Println! so that its outside log
        println!("Checking git status across workspaces...");
        println!("Workspace: '{}'\n", project_dir.display());

        let (tx, rx) = mpsc::channel();
        let mut checked_count = 0;

        for entry in project_dir.read_dir()? {
            let path = entry?.path();
            if !path.is_dir() { continue };

            let git_dir = path.join(".git");
            if !git_dir.exists() || !git_dir.is_dir() { continue }

            checked_count += 1;

            let tx_clone = tx.clone();
            thread::spawn(move || {
                let result = Git::fetch_and_status(&path);
                tx_clone.send((path.clone(), result)).unwrap();
            });
        }

        let mut results = Vec::with_capacity(checked_count);
        drop(tx);

        for res in rx {
            results.push(res);

            print!("\r~ Scanning repositories... [{}/{}]", results.len(), checked_count);
            std::io::stdout().flush()?;
        }

        // Clear the loading line
        print!("\r\x1b[K"); 
        std::io::stdout().flush()?;

        results.sort_by(|a, b| a.0.cmp(&b.0));

        let mut dirty_repos = Vec::new();
        let mut behind_repos = Vec::new();
        let mut ahead_repos = Vec::new();

        for (path, status) in results {
            let filename = match path.file_name().and_then(|os_str| os_str.to_str()) {
                Some(name) => name,
                None => "Unknown"
            };

            match status {
                Ok(status) => {
                    let status_data = Git::get_branch_ahead_behind_from_status(&status);

                    let mut remote_sync_str = String::new();
                    if let a = status_data.ahead_count && a > 0 {
                        remote_sync_str.push_str(&format!("+ Ahead ({}) ", a).bright_blue().to_string());
                    }
                    if let b = status_data.behind_count && b > 0 {
                        remote_sync_str.push_str(&format!("- Behind ({})", b).bright_yellow().to_string());
                        behind_repos.push((filename.to_string(), path.clone()));
                    }
                    if remote_sync_str.is_empty() {
                        remote_sync_str = "Synced".green().to_string();
                    }

                    let mut dirty_ui_str = String::new();
                    if status_data.is_dirty {
                        dirty_repos.push((filename.to_string(), path.clone()));
                        dirty_ui_str.push_str(&"Has changes".red().to_string());
                    } else if status_data.ahead_count > 0 {
                        // If it's dirty, we will prompt to push during the commit phase instead.
                        // Only add to ahead_repos if it is clean but has unpushed commits.
                        ahead_repos.push((filename.to_string(), path.clone()))
                    } else { 
                        dirty_ui_str.push_str(&"Clean".green().to_string());
                    }

                    println!(
                        "{}...## {}\n  Status: {}\n  Remote: {}",
                        filename.bold(),
                        status_data.branch_name,
                        dirty_ui_str,
                        remote_sync_str
                    )
                }
                Err(e) => {
                    let msg = format!(
                        "Failed to check {}\n Error: {e}",
                        filename.bold()
                    )
                    ;
                    eprintln!("{}", msg.red());
                }
            }
        }

        if checked_count == 0 {
            println!("No valid git projects found in '{}'", project_dir.display().bright_yellow());
        } else {
            println!("\nChecked {} repositories.", checked_count.bright_cyan());
        }

        if !behind_repos.is_empty() {
            let msg = format!(
                "\nFound {} repository(s) behind remote. Pull changes? [y/N]: ",
                behind_repos.len().bright_yellow()
            );
            if self.yes_input(&msg)? {
                for (name, path) in behind_repos {
                    let msg = format!(
                        "Would you like to pull {}? [Y/n]: ",
                        name.bold()
                    );
                    if !self.no_input(&msg)? {
                        println!("  {} {}...", "~ Pulling".bright_blue(), name.bold());
                        Git::pull(&path).status()?;
                    }
                }
            };
        }

        if !dirty_repos.is_empty() {
            let msg = format!(
                "\nFound {} repository(s) with uncommitted changes. Review and commit? [y/N]: ",
                dirty_repos.len().bright_yellow()
            );
            if self.yes_input(&msg)? {
                for (name, path) in dirty_repos {

                    let msg = format!(
                        "Would you like to review {}? [Y/n]: ",
                        name.bold()
                    );

                    if self.no_input(&msg)? { continue };

                    println!("\n{}", "==================================================".bright_cyan());
                    println!("📝 Reviewing: {}", name.bold().bright_yellow());
                    println!("{}", "==================================================".bright_cyan());

                    let output = Git::diff(&path).output()?;
                    println!("{}", String::from_utf8_lossy(&output.stdout));

                    println!("Repo: {}", name.bold().bright_yellow());
                    
                    let user_input = self.input("Enter commit message (Leave empty to skip, 'q' to abort all remaining): ")?;
                    if user_input.trim().eq_ignore_ascii_case("q") {
                        println!("{}", "Aborting remaining commits.".bright_red());
                        break;
                    }

                    if user_input.trim().is_empty() {
                        println!("{}", "Skipped.".bright_yellow());
                        continue;
                    };

                    Command::new("git")
                        .arg("add")
                        .arg(".")
                        .current_dir(&path)
                        .status()?;

                    let commit_status = Git::commit(&path, &user_input)
                        .status()?;

                    if commit_status.success() {
                        println!("{}", "[V] Committed successfully!".green());

                        if self.yes_input("Push changes to remote? [y/N]")? {
                            println!("  {} {}...", "+ Pushing".bright_green(), name.bold());
                            let push_status = Git::push(&path)
                                .status()?;

                            if push_status.success() {
                                println!("{}", "[V] Pushed successfully!".green());

                            } else {
                                println!("{}", "[X] Push failed.".red());
                            }

                        }

                    } else {
                        println!("{}", "[X] Commit failed.".red());
                    }
                }
            }
        }

        if !ahead_repos.is_empty() {
            let msg = format!(
                "\nFound {} repository(s) with unpushed commits. Push changes? [y/N]: ",
                ahead_repos.len().bright_yellow()
            );
            if self.yes_input(&msg)? {
                for (name, path) in ahead_repos {
                    let msg = format!(
                        "Would you like to push {}? [Y/n]: ",
                        name.bold()
                    );
                    if !self.no_input(&msg)? {
                        println!("  {} {}...", "+ Pushing".bright_green(), name.bold());
                        Git::push(&path).status()?;
                    }
                }
            }
        }

        Ok(0)
    }
}


impl Action for GitCommand {
    fn run(&self, config: &Config) -> Result<i32, Box<dyn Error>> {
        match self.action {
            GitAction::CheckAll => self.check_all(config),
        }
    }
}
