use clap::{Parser};
use crate::cli::Cli;


fn main() {
    let app = Cli::parse();
    app.run();
}
