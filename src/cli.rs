use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Source path
    #[arg(short, long, default_value = "action.yml")]
    pub file: String,

    /// Dry run
    #[arg(long, default_value = "false")]
    pub dry: bool,

    /// Output filepath
    #[arg(short, long)]
    pub output: Option<String>,
}
