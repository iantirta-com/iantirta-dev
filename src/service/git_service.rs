use std::{error::Error, path::Path, process::{Command, Stdio}};

use log::warn;


pub struct Git {

}

/// Git common services command
impl Git {
    pub fn push(path: &Path) -> Command {
        let mut cmd = Command::new("git");

        cmd.arg("push")
            .current_dir(path);

        cmd
    }

    pub fn pull(path: &Path) -> Command {
        let mut cmd = Command::new("git");

        cmd.arg("pull")
            .current_dir(path);

        cmd
    }

    pub fn fetch(path: &Path) -> Command {
        let mut cmd = Command::new("git");

        cmd.arg("fetch")
            .stdin(Stdio::null())
            .current_dir(path);

        cmd
    }

    pub fn status(path: &Path) -> Command {
        let mut cmd = Command::new("git");
        
        cmd.arg("status")
            .arg("--porcelain")
            .arg("-b")
            .current_dir(path);

        cmd
    }

    pub fn diff(path: &Path) -> Command {
        let mut cmd = Command::new("git");
        
        cmd.arg("diff")
            .arg("--minimal")
            .arg("--color=always")
            .current_dir(path);

        cmd
    }

    pub fn commit(path: &Path, msg: &str) -> Command {
        let mut cmd = Command::new("git");
        
        cmd.arg("commit")
            .arg("-m")
            .arg(msg)
            .current_dir(path);

        cmd
    }
}


#[derive(Default)]
pub struct GitStatusMetadata {
    pub branch_name: String,
    pub ahead_count: u32,
    pub behind_count: u32,
    pub is_dirty: bool,
}


/// Advanced use of Git
impl Git {
    /// method to fetch and check status
    pub fn fetch_and_status(path: &Path) -> Result<String, Box<dyn Error + Send + Sync>> {
        let fetch_status = Self::fetch(path).output()?;

        if !fetch_status.status.success() {
            warn!(
                "Fetching status return error: {}",
                String::from_utf8_lossy(&fetch_status.stderr)
            );
        }

        let status_output = Self::status(path).output()?;

        if status_output.status.success() {
            Ok(String::from_utf8_lossy(&status_output.stdout).into())
        } else {
            let stderr = String::from_utf8_lossy(&status_output.stderr);
            Ok(format!("'git-status' Error: {}", stderr.trim()))
        }
    }

    /// method to get branch_name, ahead and behind from status output
    pub fn get_branch_ahead_behind_from_status(status: &str) -> GitStatusMetadata {
        let mut status_data = GitStatusMetadata::default();

        let mut lines = status.lines();

        if let Some(branch_line) = lines.next() {
            let content = branch_line.trim_start_matches("## ");
            let end_idx = content.find("...").or_else(|| content.find(' '))
                .unwrap_or(content.len());
            status_data.branch_name = content[..end_idx].to_string();

            if let Some(idx) = branch_line.find("ahead ") {
                // ahead need push
                let tail = &branch_line[idx + 6..];
                let end = tail.find(|c: char| !c.is_ascii_digit()).unwrap_or(tail.len());
                status_data.ahead_count = tail[..end].parse::<u32>().unwrap_or(0);
            }

            if let Some(idx) = branch_line.find("behind ") {
                // behind need pull
                let tail = &branch_line[idx + 7..];
                let end = tail.find(|c: char| !c.is_ascii_digit()).unwrap_or(tail.len());
                status_data.behind_count = tail[..end].parse::<u32>().unwrap_or(0);
            }
        }

        if lines.next().is_some() {
            status_data.is_dirty = true
        }
        status_data
    }
}