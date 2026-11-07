use clap::{Parser};
use regex::Regex;

#[derive(Parser, Debug)]
#[command(name = "idev")]
#[command(about = "A fast environment and project manager", long_about = None)]
struct Args {

    /// The name of the project you want to open or manage
    #[arg(value_name = "PROJECT_NAME")]
    name: String,
}

fn main() {
    let args = Args::parse();
    let name = args.name.to_lowercase();

    let re = Regex::new(r"^(?:iantirta|i)?[\W_]*(.+)$").unwrap();

    let project_name = if let Some(caps) = re.captures(&name) {
        caps.get(1).unwrap().as_str()
    } else {
        &name
    };

    let full_project_name = format!("iantirta-{}", project_name);

    println!("User typed: {}", name);
    println!("Core name extracted: {}", project_name);
    println!("True project name: {}", full_project_name);

}