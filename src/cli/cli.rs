use std::{
    ffi::{
        OsString,
    }
};

use clap::{
    Parser,
};

use log::{
    trace,
};

use crate::{
    config::{
        Config,
    },
    cli::{
        Commands
    }
};


/// Cli Parser
#[derive(Parser, Debug, Default)]
#[command(name = "idev", version, author)]
#[command(
    about = "A fast environment development and project manager by iantirta.com",
    long_about = None
)]
pub struct Cli {

    #[command(flatten)]
    pub config: Config,

    #[command(subcommand)]
    pub command: Commands,
}


/// Custom parser
impl Cli {

    /// Parse config with default and file config present.
    /// and will setup logging if set to true.
    /// 
    /// all log before this, count as danger log.
    pub fn parse_config<I, T>(itr: I, setup_logging: bool) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let mut cli = Self::default();
        cli.update_from(itr);

        match setup_logging {
            true => cli.config.setup_logging(),
            _ => {},
        }
        trace!("{:#?}", cli);
        cli
    }
}