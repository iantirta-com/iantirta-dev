
use std::process::exit;
use std::{env};
use clap::{Parser, CommandFactory};
use crate::config::*;
use crate::cli::{Commands, DevelopCommand};

pub fn main() {
    println!("Cli Called");

    let mut sysargv: Vec<String> = env::args()
        .collect();
    let mut argv = &sysargv[1 ..];

    if sysargv.len() == 1 || sysargv[1].starts_with("-") {
        sysargv.insert(1, "develop".to_string());
    }
    println!("Modified arguments: {:#?}", sysargv);

    let mut config = Config::parse_from(&sysargv);
    exit(config.action_run())
}
