use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub struct Args {
    #[arg(
        short,
        long,
        value_delimiter(','),
        help = "comma-separated list of projects to search"
    )]
    pub projects: Vec<String>,
}
