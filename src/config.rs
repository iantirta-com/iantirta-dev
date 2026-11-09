use clap::{Error, Command, Args, Parser, Subcommand, CommandFactory, ArgAction, FromArgMatches};
use crate::cli::{Commands, Action};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::exit;
use crate::constants::{IDEV_CONFIG_PATH};


#[derive(Parser, Debug)]
#[command(name = "idev")]
#[command(
    about = "A fast environment development and project manager by iantirta.com",
    long_about = None
)]
pub struct Config {

    #[command(flatten)]
    config: BaseConfig,

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

    #[arg(
        long,
        global = true
    )]
    project_dir: Option<PathBuf>,

    #[arg(
        long,
        global = true
    )]
    venv_dir: Option<PathBuf>,

    #[arg(short, long, global=true, default_value=IDEV_CONFIG_PATH)]
    config_path: Option<PathBuf>
}


impl Config {
    pub fn action_run(&mut self) -> i32 {
        println!("Command Config {:#?}", self);
        let command = self.command.as_ref()
            .unwrap_or_else(|| exit(1));
        command.run(&mut self.config)
    }
}


/// File config impl
impl Config {
    fn load_config(&self) {
        
    }
}


fn main() {
    println!("Ok")
}