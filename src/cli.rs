use clap::Parser;

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
