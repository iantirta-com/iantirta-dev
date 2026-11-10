use std::error::Error;

use clap::Subcommand;
use enum_dispatch::enum_dispatch;

use crate::{
    config::{
        Config,
    },
    cli::{
        DevelopCommand,
        GitCommand
    },
};


#[enum_dispatch]
pub trait Action {
    fn run(&self, config: &Config) -> Result<i32, Box<dyn Error>>;
}


#[enum_dispatch(Action)]
#[derive(Subcommand, Debug)]
pub enum Commands {

    Develop(DevelopCommand),

    //Config(ConfigCommand),
    Git(GitCommand),
}

impl Default for Commands {
    fn default() -> Self {
        Commands::Develop(DevelopCommand::default())
    }
}