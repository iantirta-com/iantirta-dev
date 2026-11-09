
use std::process::exit;
use std::{env};
use clap::{Parser};
use crate::config::*;
use crate::cli::{Action};

pub fn main() {
    let mut sysargv: Vec<String> = env::args()
        .collect();

    if sysargv.len() == 1 || sysargv[1].starts_with("-") {
        sysargv.insert(1, "develop".to_string());
    }

    let mut config = Config::parse_from(&sysargv);
    let command = config.command.unwrap();

    match command.run(&mut config.config) {
        Ok(exit_code) => exit(exit_code),
        Err(e) => {
            eprintln!("Command failed with error: {}", e);
            // exit(1);
        }
    }
}
