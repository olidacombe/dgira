use color_eyre::Result;
use std::{env, sync::OnceLock};
use tracing::debug;

use gouqi::{Credentials, Jira};

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
