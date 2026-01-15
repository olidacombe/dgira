//! A tool for making tools - with Jira
//!
//! ## Setup
//!
//! `dgira` needs three environment variables:
//!
//! ```bash
//! export JIRA_HOST=https://foo.atlassian.net/
//! export JIRA_USER=timmy
//! export JIRA_PASS=.....
//! ```
use cli::Args;
use color_eyre::Result;
use gouqi::{Credentials, Jira, SearchOptions};
use std::{env, sync::OnceLock};
use tracing::debug;

pub mod cli;

fn credentials() -> Result<Credentials> {
    let user = env::var("JIRA_USER")?;
    let pass = env::var("JIRA_PASS")?;
    Ok(Credentials::Basic(user, pass))
}

fn host() -> Result<String> {
    debug!("Looking up `JIRA_HOST`");
    Ok(env::var("JIRA_HOST")?)
}

fn init_client() -> Result<Jira> {
    Ok(Jira::new(host()?, credentials()?)?)
}

pub fn client() -> &'static Jira {
    static CLIENT: OnceLock<Jira> = OnceLock::new();
    CLIENT.get_or_init(|| init_client().expect("Failed to initialize Jira client"))
}

pub fn search_options(args: &Args) -> SearchOptions {
    let mut opts = SearchOptions::builder();
    if args.compact {
        opts.fields(vec!["summary"]);
    }
    opts.build()
}

pub fn query(args: &Args) -> String {
    let mut query = "order by created DESC".to_string();
    query = format!(
        "{} {query}",
        args.projects
            .iter()
            .map(|p| format!("project = {p}"))
            .collect::<Vec<_>>()
            .join(" or ")
    );
    query
}
