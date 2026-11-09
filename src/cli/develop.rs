use clap::Args;
use crate::config::{BaseConfig, Config};
use crate::cli::{Action, ProjectArgs};
use std::fs::{self, ReadDir};
use std::process::{exit, Stdio, Command};
use std::path::PathBuf;
use std::error::Error;
use globset::{Glob, GlobSet, GlobSetBuilder};
use log::{info, debug, error, warn};


#[derive(Args, Debug, Default)]
pub struct DevelopCommand {
    #[command(flatten)]
    pub project: ProjectArgs,
}

/// Helper method for DevelopCommand
impl DevelopCommand {
    fn get_project_id(&self) -> String {
        if let Some(ref project_id) = self.project.id {
            project_id.to_string()
        } else if let Some(ref project_id) = self.project.id_flag {
             project_id.to_string()
        } else {
            error!("Error: A Project ID is required!");
            Config::command().print_long_help();
            exit(1);
        }
    }
}


impl Action for DevelopCommand {
    fn run(&self, config: &mut BaseConfig) -> Result<i32, Box<dyn Error>> {
        let project_id = self.get_project_id();

//         println!("Develop Command Run! Project ID: {:#?}", project_id);

//         let re = config.get_project_re();

//         let project_dir: PathBuf = config.get_project_dir();
//         if !project_dir.is_dir() {
//             return Err("project-dir should be a dir".into());
//         };

//         let mut project_path= None;
//         let mut is_python = false;

//         // get project_path
//         for entry in project_dir.read_dir()? {
//             if let Ok(entry) = entry {
//                 if let Some(caps) = re.captures(entry.file_name().to_str().unwrap()) {
//                     if let Some(project_match) = caps.get(1) {
//                         if project_match.as_str() == project_id.as_str() && entry.path().is_dir() {
//                             project_path = Some(entry.path());
//                             break;
//                         }
//                     }
//                 }
//             }
//         }
//         let final_path = match project_path {
//             Some(path) => path,
//             None => {
//                 return Err(format!("Could not find project directory for ID: {}", project_id).into());
//             }
//         };

//         println!("Successfully found path: {:?}", final_path);

//         for path in WalkDir::new(&final_path) {
//             if let Some(ext) = path.extension() {
//                 if ext == "py" {
//                     is_python = true;
//                     break;
//                 }
//             }
//         }

//         println!("Is Python: {:?}", is_python);

//         let prog_name = if cfg!(target_os = "windows") {
//             "powershell".to_string()
//         } else {
//             match std::env::var("SHELL") {
//                 Ok(cmd) => cmd,
//                 Err(e) => {
//                     println!("Error while getting SHELL Value: {}", e);
//                     "bash".to_string()
//                 }
//             }
//         };
//         let mut child = Command::new(prog_name) // Lowercase 'sh', 'bash', or std::env::var("SHELL")
//             .current_dir(final_path)
//             .stdin(Stdio::inherit())
//             .stdout(Stdio::inherit())
//             .stderr(Stdio::inherit())
//             .spawn()
//             .expect("Failed to spawn shell");

//         let _ = child.wait().expect("Failed to wait on child process");

        Ok(0)
    }
}


// // Helper
// struct WalkDir {
//     stack: Vec<ReadDir>,
//     ignore_matcher: GlobSet,
// }


// impl WalkDir {
//     fn new(root: &PathBuf) -> Self {
//         let mut stack = Vec::new();

//         let ignore_matcher = match Self::build_ignore_matcher(&root) {
//             Ok(set) => set,
//             Err(e) => {
//                 eprintln!("Error while setting up ignore matcher: {}", e);
//                 exit(1)
//             }
//         };
//         // println!("Set is: {:#?}", ignore_matcher);

//         let rd = fs::read_dir(root);
//         match rd {
//             Ok(rd) => {
//                 stack.push(rd);
//                 Self { stack, ignore_matcher }
//             },
//             Err(e) => {
//                 eprintln!("Error while walk_dir: {}", e);
//                 exit(1)
//             }
//         }
//     }

//     fn build_ignore_matcher(root: &PathBuf) -> Result<GlobSet, Box<dyn Error>> {
//         let gitignore_path = root.join(".gitignore");
//         let mut builder = GlobSetBuilder::new();

//         builder.add(Glob::new(".git")?);
//         builder.add(Glob::new("**/.git/**")?);

//         let raw_patterns = match fs::read_to_string(&gitignore_path) {
//             Ok(content) => {
//                 content
//                     .lines()
//                     .map(|l| l.trim().to_string())
//                     .filter(|l| !l.is_empty() && !l.starts_with('#'))
//                     .collect::<Vec<String>>()
//             }
//             Err(_) => {
//                 println!(".gitignore not found. Using default fallback patterns.");
//                 vec![
//                     "__pycache__/".to_string(),
//                     ".venv/".to_string(),
//                     "venv/".to_string(),
//                     "target/".to_string(),
//                     "*.so".to_string(),
//                 ]
//             }
//         };

//         for pattern in raw_patterns {
//             let mut pat = pattern;

//             if pat.ends_with('/') {
//                 let trimmed = pat.trim_end_matches("/");
//                 builder.add(Glob::new(&format!("**/{}/**", trimmed))?);
//                 pat = trimmed.to_string();
//             }
//             builder.add(Glob::new(&pat)?);
//             builder.add(Glob::new(&format!("**/{}", pat))?);
//         };

//         Ok(builder.build()?)
//     }
// }


// impl Iterator for WalkDir {
//     type Item = PathBuf;
    
//     fn next(&mut self) -> Option<Self::Item> {
//         while let Some(current_dir) = self.stack.last_mut() {
//             if let Some(Ok(entry)) = current_dir.next() {
//                 let path = entry.path();
//                 if self.ignore_matcher.is_match(&path) {
//                     continue;
//                 }
//                 if path.is_dir() {
//                     if let Ok(next_dir) = fs::read_dir(path) {
//                         self.stack.push(next_dir);
//                     }
//                 } else {
//                     return Some(path);
//                 }
//             } else {
//                 self.stack.pop();
//             }
//         }
//         None
//     }
// }
