use clap::{Args, Parser, CommandFactory, FromArgMatches, Error as ClapError};

use regex::Regex;
use env_logger::Builder;
use log::{Level, LevelFilter, debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};

use std::{
    fs,
    path::PathBuf,
    env,
    ffi::OsString,
    error::Error
};

use crate::constants::{IDEV_CONFIG_PATH, IDEV_PROJECT_DIR};
use crate::cli::{self, Commands, DevelopCommand, ProjectArgs};


/// Cli Config Parser
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
    pub command: Commands,
}


fn format_error<I: CommandFactory>(err: ClapError) -> ClapError {
    let mut cmd = I::command();
    err.format(&mut cmd)
}


// Custom config parsing
impl Config {

    fn default() -> Self {
        Self {
            config: BaseConfig::default(),
            command: Commands::Develop(DevelopCommand::default()),
        }
    }

    fn setup_logging(&self) {
        let log_level = match self.config.verbosity {
            0 => LevelFilter::Warn,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        };

        Builder::new()
            .filter_level(log_level)
            .format_timestamp(None)
            .format_target(false)
            .init();

        debug!("Log level setup for: {}", log_level);
    }

    /// Parse config with default and file config present.
    /// and will setup logging if set to true.
    /// 
    /// all log before this, count as danger log.
    pub fn parse_config<I, T>(itr: I, setup_logging: bool) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let mut config = Self::default();
    
        {
            let ref mut default_config = config.config;
            let file_config = match default_config.load_file_config() {
                Ok(config) => Some(config),
                Err(e) => {
                    eprintln!("Error while loading config: {}", e);
                    None
                }
            };
            if let Some(file_config) = file_config {
                *default_config = BaseConfig {
                    ..file_config
                }
            }
        }
        config.update_from(itr);

        match setup_logging {
            true => config.setup_logging(),
            _ => {},
        }
        trace!("{:#?}", config);
        config
    }
}


/// Base Config holds global value
#[derive(Debug, Args, Serialize, Deserialize)]
pub struct BaseConfig {

    /// Set log verbosity: (9: WARN, 1: INFO, 2: DEBUG, +: TRACE)
    #[arg(short, long="verbose", global=true, action=clap::ArgAction::Count)]
    #[serde(skip)]
    pub verbosity: u8,

    #[arg(long, global=true)]
    project_dir: PathBuf,

    #[arg(long, global = true)]
    venv_dir: Option<PathBuf>,

    #[arg(long, global=true)]
    prefix: String,

    #[arg(short, long, global=true)]
    config_path: PathBuf,

    #[arg(short, long, global=true, action=clap::ArgAction::SetTrue)]
    #[serde(skip)]
    pub save: bool
}


impl Default for BaseConfig {
    fn default() -> Self {
        Self {
            project_dir: IDEV_PROJECT_DIR.to_path_buf(),
            prefix: "iantirta|i".to_string(),
            config_path: IDEV_CONFIG_PATH.to_path_buf(),
            venv_dir: None,
            verbosity: 0,
            save: false,
        }
    }
}


// File BaseConfig impl
impl BaseConfig {

    fn get_config_path(&self) -> &PathBuf {
        &self.config_path
    }

    pub fn load_file_config(&self) -> Result<BaseConfig, Box<dyn std::error::Error>> {
        let config_path = self.get_config_path();
        let content = fs::read_to_string(config_path)?;
        let parsed: BaseConfig = toml::from_str(&content)?;
        Ok(parsed)
    }

    pub fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string(self)?;
        let config_path = self.get_config_path();

        let mut temp_path = config_path.clone();
        temp_path.set_extension("tmp");

        fs::write(&temp_path, content)?;
        fs::rename(temp_path, config_path)?;

        info!("Saving Config to: {:#?}", config_path);
        Ok(())
    }
}


// /// Project Level BaseConfig API
// impl BaseConfig {
//     pub fn get_project_re(&self) -> Regex {
//         let prefix = self.prefix.as_deref().unwrap();
//         let pattern = format!(r"^(?:{})?[\W_]*(.+)$", prefix);
//         Regex::new(&pattern)
//             .expect("Invalid regex pattern generated from config prefix")
//     }
// }


// /// Reflection BaseConfig
// impl BaseConfig {
//     pub fn get_project_dir(&self) -> PathBuf {
//         self.project_dir.as_deref()
//             .expect("No project dir was set, add it by --project_dir")
//             .to_path_buf()
//     }
// }
