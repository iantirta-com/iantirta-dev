use std::{
    env,
    process::{
        exit
    },
};

use log::{error, trace};

use crate::{
    cli::{
        Cli,
        Action // needed to implement run()
    }
};


pub fn main() {
    let mut sysargv: Vec<String> = env::args()
        .collect();

    if sysargv.len() == 1 || sysargv[1].starts_with("-") {
        sysargv.insert(1, "develop".to_string());
    }

    let cli = Cli::parse_config(&sysargv, true);    

    let command = cli.command;

    trace!("Running command: {:#?}", command);    

    if cli.config.save() {
        cli.config.save_file_config().unwrap_or_else(|e| {
            error!("Saving failed with error: {e}")
        });
    };

    match command.run(&cli.config) {
        Ok(exit_code) => exit(exit_code),
        Err(e) => {
            error!("{e}");
        }
    }
}
