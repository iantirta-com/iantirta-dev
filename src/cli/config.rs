use clap::{Args, Subcommand};
use crate::config::{Config};


#[derive(Args, Debug, Clone)]
pub struct ConfigParams {

    /// Prefix, can be re or simple string
    #[arg(long)]
    prefix: Option<String>,

    /// The venv path for this project name (use full path.)
    #[arg(long)]
    venv_dir: Option<String>,

    /// The venv pattern for this project name
    /// e.g. <prefix>_<project_name>_<venv>
    #[arg(long)]
    venv_pattern: Option<String>,
}


#[derive(Subcommand, Debug, Clone)]
pub enum ConfigAction {
    /// Set configuration globally or per project
    Set(ConfigParams),
    
    /// Reset configuration globally or per project
    Reset(ConfigParams),

    /// List current configuration globally or per project
    List,

    /// Get current configuration globally or per project
    Get {
        /// The key to retrieve (e.g., prefix, venv_dir, venv_pattern)
        key: String,
    },
}


impl ConfigAction {

    pub fn run(&self, config: &mut Config) {

        let context = match &config.project_name {
            Some(proj) => format!("PROJECT ({})", proj),
            None => "GLOBAL".to_string(),
        };

        match self {
            ConfigAction::Set(params) => {
                println!("[{}] Setting config: {:?}", context, params);

                if let Some(val) = &params.prefix {
                    if let Err(e) = config.set("prefix", val) {
                        eprintln!("Failed to set prefix: {}", e);
                    } else {
                        println!("  -> prefix = {}", val);
                    }
                }
                
                if let Some(val) = &params.venv_dir {
                    if let Err(e) = config.set("venv_dir", val) {
                        eprintln!("Failed to set venv_dir: {}", e);
                    } else {
                        println!("  -> venv_dir = {}", val);
                    }
                }
                
                if let Some(val) = &params.venv_pattern {
                    if let Err(e) = config.set("venv_pattern", val) {
                        eprintln!("Failed to set venv_pattern: {}", e);
                    } else {
                        println!("  -> venv_pattern = {}", val);
                    }
                }
            }
            ConfigAction::Reset(params) => {
                println!("[{}] Resetting config: {:?}", context, params);
            }
            ConfigAction::List => {
                println!("[{}] Listing config...", context);
                match config.list() {
                    Ok(json_output) => println!("{}", json_output),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            ConfigAction::Get { key } => {
                println!("[{}] Getting config: {:?}", context, key);
                match config.get(key) {
                    Ok(Some(val)) => println!("[{}] {} = {}", context, key, val),
                    Ok(None) => println!("[{}] {} is currently unset (None)", context, key),
                    Err(e) => eprintln!("Error: {}", e), // E.g., "Key 'fake_key' does not exist"
                }
            }
        }
    }
}
