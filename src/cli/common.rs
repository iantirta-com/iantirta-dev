use clap::{Args};

/// Common project argument
/// such as project_id
#[derive(Args, Debug, Default)]
pub struct ProjectArgs {

    #[arg(value_name = "PROJECT_ID")]
    pub id: Option<String>,

    #[arg(short = 'p', long = "project_id", value_name = "PROJECT_ID", conflicts_with = "id")]
    pub id_flag: Option<String>,
}
