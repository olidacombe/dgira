use clap::Parser;
use clap_complete::Shell;

#[derive(Parser)]
#[command(version)]
pub struct Args {
    #[arg(
        default_value_t = false,
        short,
        long,
        help = "compact, i.e. summary only"
    )]
    pub compact: bool,
    /// Generate a SHELL completion script and print to stdout
    #[clap(long, value_enum, value_name = "SHELL")]
    pub completions: Option<Shell>,
    #[arg(short, long, help = "max results to fetch")]
    pub limit: Option<usize>,
    #[arg(
        short,
        long,
        value_delimiter(','),
        help = "comma-separated list of projects to search"
    )]
    pub projects: Vec<String>,
}
