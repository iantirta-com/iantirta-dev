use std::error::Error;

use clap::{Subcommand};
use enum_dispatch::enum_dispatch;

use crate::cli::DevelopCommand;
use crate::config::BaseConfig;

#[enum_dispatch]
pub trait Action {
    fn run(&self, config: &mut BaseConfig) -> Result<i32, Box<dyn Error>>;
}


#[enum_dispatch(Action)]
#[derive(Subcommand, Debug)]
pub enum Commands {
    Develop(DevelopCommand),

    //Config(ConfigCommand),
}
