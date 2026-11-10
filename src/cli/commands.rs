use std::error::Error;

use clap::Subcommand;
use enum_dispatch::enum_dispatch;

use crate::{
    config::{
        Config,
    },
    cli::{
        DevelopCommand
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
}

impl Default for Commands {
    fn default() -> Self {
        Commands::Develop(DevelopCommand::default())
    }
}