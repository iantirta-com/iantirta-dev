use clap::{Args, Subcommand};
use crate::config::BaseConfig;
use crate::cli::{Action, ProjectArgs};
use std::process::exit;


#[derive(Args, Debug)]
pub struct DevelopCommand {
    #[command(flatten)]
    pub project: ProjectArgs,
}

impl Action for DevelopCommand {
    fn run(&self, config: &mut BaseConfig) -> i32 {

        let ref project_id = match (&self.project.id, &self.project.id_flag) {
            (Some(id), None) => id,
            (None, Some(flag)) => flag,
            (None, None) => {
                eprintln!("Error: A Project ID is required!");
                exit(1);
            },
            (Some(_), Some(_)) => {
                eprintln!("Error: Cannot provide both a positional ID and a flag!");
                exit(1);
            }
        };

        println!(
            "Develop Command Run! Project ID: {:#?} | {:#?}", project_id, self
        );

        config.verbosity = 2;
        
        0
    }
}
