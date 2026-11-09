use clap::{Subcommand};
use crate::cli::DevelopCommand;
use crate::config::BaseConfig;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Action {
    fn run(&self, config: &mut BaseConfig) -> i32;
}

#[enum_dispatch(Action)]
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Default
    Develop(DevelopCommand),

    //Config(ConfigCommand),
}


