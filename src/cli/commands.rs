use clap::{Parser, Subcommand, CommandFactory};
use crate::{cli::ConfigAction, config::{Config}};
use regex::Regex;


#[derive(Parser)]
#[command(name = "idev")]
#[command(about = "A fast environment development and project manager by iantirta.com", long_about = None)]
pub struct Cli {

    /// The name of the project you want to open or manage
    #[arg(value_name = "PROJECT_NAME")]
    pub name: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}


impl Cli {

    fn _get_project_core_name(&self, prefix: &str, project_name: Option<&str>) -> Option<String> {
        if let Some(raw_name) = project_name {
            let name: String = raw_name.to_lowercase();

            let pattern = format!(r"^(?:{})?[\W_]*(.+)$", prefix);
            let re = Regex::new(&pattern)
                .expect("Invalid regex pattern generated from config prefix");

            println!("Prefix known: {}", prefix);

            if let Some(caps) = re.captures(&name) {
                // If group 1 exists, convert the matched &str to an owned String
                if let Some(match_str) = caps.get(1) {
                    return Some(match_str.as_str().to_string());
                }
            }
            Some(name)
        } else {
            None
        }
    }

    pub fn run(&self) {

        let mut config = Config::load(self.name.as_deref());
        let prefix = match config.get("prefix") {
            Ok(Some(p)) => p,
            _ => panic!("Prefix is needed to use idev"), // Stops execution instantly
        };

        let parsed_project = self._get_project_core_name(
            &prefix,
            self.name.as_deref(),
        );

        if let Some(project) = &parsed_project {
            println!("Resolved project: {}", project);
        }

        if let Some(cmd) = &self.command {
            cmd.run(&mut config)
        } else if let Some(project) = &parsed_project {
            println!("Opening/Running project: {}", project);

            // available is <prefix> and <project_name>
            // {prefix}_{project_name}_venv
            if let Ok(Some(venv_pattern)) = config.get("venv_pattern") {
                let venv_name = venv_pattern
                    .replace("{prefix}", "")
                    .replace("{project_name}", project);

                let pattern = format!(r"^(?:{})?[\W_]*(?:{})$", prefix, venv_name);
                let re = Regex::new(&pattern)
                    .expect("Invalid regex pattern generated from config prefix");

                println!("Target VENV Name: {}", venv_name);

            } else {
                println!("No venv_pattern found in configuration.");
            }
        } else {
            Cli::command().print_long_help().unwrap();
        }
    }
}


#[derive(Subcommand)]
pub enum Commands {
    /// Configuration management
    Config {
        #[command(subcommand)]
        action: ConfigAction
    }
}


impl Commands {
    pub fn run(&self, config: &mut Config) {
        match self {
            Commands::Config { action } => {
                // Delegate to the config module's run method
                action.run(config);
            }
        }
    }
}
