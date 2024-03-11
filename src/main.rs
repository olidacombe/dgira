use color_eyre::Result;
use dgira::{client, search_options};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let query = "order by created DESC".to_string();

    let jira = client();

    jira.search()
        .iter(query, &search_options())?
        .filter_map(|issue| serde_json::to_string(&issue).ok())
        .for_each(|issue| {
            println!("{issue}");
        });

    Ok(())
}
