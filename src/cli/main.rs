
use std::process::exit;
use std::{env};
use clap::{Parser};
use log::{Level, LevelFilter};
use env_logger::Builder;
use log::{info, debug, error, warn};

use crate::config::*;
use crate::cli::{Action};


pub fn main() {
    let mut sysargv: Vec<String> = env::args()
        .collect();

    if sysargv.len() == 1 || sysargv[1].starts_with("-") {
        sysargv.insert(1, "develop".to_string());
    }

    let mut config = Config::parse_config(&sysargv, true);    
    
    let command = config.command;

    debug!("Running command: {:#?}", command);    

    if config.config.save {
        config.config.save_config().unwrap_or_else(|e| {
            error!("Saving failed with error: {e}")
        });
    };

    match command.run(&mut config.config) {
        Ok(exit_code) => exit(exit_code),
        Err(e) => {
            error!("Command failed with error: {}", e);
        }
    }
}
