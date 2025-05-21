use anyhow::Result;

mod graphql_queries {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/graphql/mod.rs"));
}

mod application;
mod infrastructure;
mod domain;

use application::use_cases::{
    close_old_issues::close_old_issues,
    create_issue::create_issue,
};

#[tokio::main]
async fn main() -> Result<()> {

    close_old_issues().await?;
    create_issue().await?;

    Ok(())
}
