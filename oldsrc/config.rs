

#[derive(Parser)]
#[command(name = "idev")]
#[command(
    about = "A fast environment development and project manager by iantirta.com",
    long_about = None
)]
struct Config {

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbosity: u8
}
