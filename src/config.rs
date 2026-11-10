
use std::{
    fs,
    path::{
        PathBuf
    },
    error::{
        Error
    },
    process::{
        exit
    }
};

use clap::{
    Args,
};
use serde::{
    Deserialize,
    Serialize
};

use regex::Regex;

use env_logger::Builder;
use log::{
    LevelFilter,
    debug,
    info,
    error,
};

use crate::{
    constants::{
        IDEV_CONFIG_PATH,
        IDEV_PROJECT_DIR
    },
};


/// Base Config that holds global value
/// This implement clap and serde
#[derive(Debug, Args, Serialize, Deserialize)]
pub struct Config {

    /// Set log verbosity: (9: WARN, 1: INFO, 2: DEBUG, +: TRACE)
    #[arg(short, long="verbose", global=true, action=clap::ArgAction::Count)]
    #[serde(skip)]
    verbosity: u8,

    /// Prefix for each project
    #[arg(long, global=true, required=false)]
    prefix: String,

    /// Project parent directory
    #[arg(long, global=true, required=false)]
    project_dir: PathBuf,

    /// Project venv parent directort
    #[arg(long, global=true)]
    venv_dir: Option<PathBuf>,

    /// Project venv prefix
    /// Available format: (
    ///    {prefix}: Project prefix
    ///    {project_id}: Project id
    ///)
    #[arg(long, global=true)]
    venv_prefix: Option<String>,

    /// Config rc file path
    #[arg(short, long, global=true, required=false)]
    config_path: PathBuf,

    /// Wheter to save this config into a file
    #[arg(short, long, global=true, action=clap::ArgAction::SetTrue)]
    #[serde(skip)]
    save: bool
}


impl Default for Config {
    fn default() -> Self {
        let mut config = Self {
            verbosity: 0,
            project_dir: IDEV_PROJECT_DIR.to_path_buf(),
            prefix: "iantirta|i".to_string(),
            venv_dir: None,
            venv_prefix: None,
            config_path: IDEV_CONFIG_PATH.to_path_buf(),
            save: false,
        };

        let file_config = match config.load_file_config() {
            Ok(c) => Some(c),
            Err(e) => {
                eprintln!("Error while loading config: {}", e);
                None
            }
        };
        if let Some(file_config) = file_config {
            config = Config {
                ..file_config
            }
        }
        config
    }
}

/// Logging
impl Config {

    pub fn setup_logging(&self) {
        let log_level = match self.verbose() {
            0 => LevelFilter::Warn,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        };

        Builder::new()
            .filter_level(LevelFilter::Warn)
            .filter_module("iantirta_dev", log_level)
            .format_timestamp(None)
            .format_target(false)
            .init();

        debug!("Log level setup for: {}", log_level);
    }
}


/// File BaseConfig impl
impl Config {

    pub fn load_file_config(&self) -> Result<Config, Box<dyn Error>> {
        let content = fs::read_to_string(&self.config_path)?;
        let parsed: Config = toml::from_str(&content)?;
        Ok(parsed)
    }

    pub fn save_file_config(&self) -> Result<(), Box<dyn Error>> {
        let content = toml::to_string(self)?;
        let config_path = self.config_path();

        let mut temp_path = config_path.clone();
        temp_path.set_extension("tmp");

        fs::write(&temp_path, content)?;
        fs::rename(temp_path, config_path)?;

        info!("Saving Config to: {:#?}", config_path);
        Ok(())
    }
}


/// Reflection
impl Config {
    pub fn config_path(&self) -> &PathBuf {
        &self.config_path
    }

    pub fn verbose(&self) -> &u8 {
        &self.verbosity
    }

    pub fn save(&self) -> bool {
        self.save
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    pub fn venv_prefix(&self) -> &str {
        match self.venv_prefix {
            Some(ref s) => &s,
            None => ""
        }
    }

    pub fn venv_dir(&self) -> &PathBuf {
        match self.venv_dir {
            Some(ref s) => &s,
            None => {
                error!("venv_dir is not set");
                exit(1)
            }
        }
    }
}


/// Project Level BaseConfig API
impl Config {
    pub fn project_re(&self) -> Regex {
        let pattern = format!(r"^(?:{})?[\W_]*(.+)$", self.prefix());
        Regex::new(&pattern).unwrap()
    }

    pub fn venv_re(&self, project_id: &str) -> Regex {
        let venv_prefix = self.venv_prefix()
            .replace("{prefix}", "")
            .replace("{project_id}", &project_id);
        
        let pattern = format!(r"^(?:{})?[\W_]*(?:{})$", self.prefix(), venv_prefix);
        Regex::new(&pattern).unwrap()
    }

    /// Exit error if project dir is not a dir
    pub fn project_dir(&self) -> &PathBuf {
        if !self.project_dir.is_dir() {
            error!("Given project_dir should be a directory: {:?}", self.project_dir);
            exit(1);
        };
        &self.project_dir
    }
}
