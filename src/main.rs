use clap::{Parser};
use iantirta_dev::cli::Cli;


fn main() {
    let app = Cli::parse();
    app.run();
}
