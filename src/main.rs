use clap::{CommandFactory, Parser};
use clap_complete::generate;
use color_eyre::Result;
use dgira::{cli::Args, client, query, search_options};
use gouqi::Issue;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    if let Some(shell) = args.completions {
        generate(
            shell,
            &mut Args::command_for_update(),
            "blabla",
            &mut &std::io::stdout(),
        );
        return Ok(());
    }

    let search_options = search_options(&args);
    let query = query(&args);

    let jira = client();

    let results = jira.search().iter(&query, &search_options)?;
    let results: Box<dyn Iterator<Item = Issue>> = match args.limit {
        Some(limit) => Box::new(results.take(limit)),
        _ => Box::new(results),
    };
    results
        .filter_map(|issue| serde_json::to_string(&issue).ok())
        .for_each(|issue| {
            println!("{issue}");
        });

    Ok(())
}
