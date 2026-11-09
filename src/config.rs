use clap::{Args, Parser, CommandFactory, FromArgMatches};
use crate::cli::Commands;
use std::path::PathBuf;
use crate::constants::{IDEV_CONFIG_PATH, IDEV_PROJECT_DIR};
use regex::Regex;


#[derive(Parser, Debug)]
#[command(name = "idev")]
#[command(
    about = "A fast environment development and project manager by iantirta.com",
    long_about = None
)]
pub struct Config {

    #[command(flatten)]
    pub config: BaseConfig,

    #[command(subcommand)]
    pub command: Option<Commands>,
}


#[derive(Debug, Args)]
pub struct BaseConfig {
    /// Set log verbosity:
    /// (
    ///     0: NONSET,
    ///     1: INFO,
    ///     2: DEBUG,
    ///     3: TRACE
    /// )
    #[arg(
        short,
        long = "verbose",
        value_name = "verbose",
        global = true,
        action = clap::ArgAction::Count
    )]
    pub verbosity: u8,

    #[arg(long, global=true, default_value=IDEV_PROJECT_DIR.as_os_str())]
    project_dir: Option<PathBuf>,

    #[arg(
        long,
        global = true
    )]
    venv_dir: Option<PathBuf>,

    #[arg(long, global=true, default_value="iantirta|i")]
    prefix: Option<String>,

    #[arg(short, long, global=true, default_value=IDEV_CONFIG_PATH.as_os_str())]
    config_path: Option<PathBuf>
}


// impl Config {
//     pub fn action_run(&mut self) -> i32 {
//         println!("Command Config {:#?}", self);
//         let command = self.command.as_ref()
//             .unwrap_or_else(|| exit(1));
//         command.run(&mut self.config)
//     }
// }


/// File BaseConfig impl
impl BaseConfig {
    fn load_config(&self) {
        
    }
}

/// Project Level BaseConfig API
impl BaseConfig {
    pub fn get_project_re(&self) -> Regex {
        let prefix = self.prefix.as_deref().unwrap();
        let pattern = format!(r"^(?:{})?[\W_]*(.+)$", prefix);
        Regex::new(&pattern)
            .expect("Invalid regex pattern generated from config prefix")
    }
}

/// Reflection BaseConfig
impl BaseConfig {
    pub fn get_project_dir(&self) -> PathBuf {
        self.project_dir.as_deref()
            .expect("No project dir was set, add it by --project_dir")
            .to_path_buf()
    }
}


fn main() {
    println!("Ok")
}