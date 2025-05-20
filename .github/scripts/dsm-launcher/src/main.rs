use anyhow::Result;

mod graphql_queries {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/graphql/mod.rs"));
}
mod queries;
mod commands;
mod workflows;

use workflows::{
    close_issues::close_issues,
    create_issue::create_issue
};

#[tokio::main]
async fn main() -> Result<()> {
    close_issues().await?;
    create_issue().await?;

    Ok(())
}
