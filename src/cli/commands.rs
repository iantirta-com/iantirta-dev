use clap::{Subcommand};
use crate::cli::DevelopCommand;
use crate::config::BaseConfig;
use enum_dispatch::enum_dispatch;
use std::error::Error;


#[enum_dispatch]
pub trait Action {
    fn run(&self, config: &mut BaseConfig) -> Result<i32, Box<dyn Error>>;
}


#[enum_dispatch(Action)]
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Default
    Develop(DevelopCommand),

    //Config(ConfigCommand),
}
