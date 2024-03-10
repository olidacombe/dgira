use color_eyre::Result;
use dgira::client;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let query = "order by created DESC".to_string();

    let jira = client();

    jira.search()
        .iter(query, &Default::default())?
        .filter_map(|issue| serde_json::to_string(&issue).ok())
        .for_each(|issue| {
            println!("{issue}");
        });

    Ok(())
}
