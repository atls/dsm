use anyhow::Result;

mod graphql_queries {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/graphql/mod.rs"));
}
mod queries;

use queries::{
    close_issues::close_issue,
    create_issue::create_issue,
    get_issues::get_issues,
    get_members::get_members
};

#[tokio::main]
async fn main() -> Result<()> {
    let (repo_id, issues) = get_issues().await?;

    close_issue(
        issues, 
    ).await?;

    let assignees = get_members().await?;

    create_issue(
        repo_id,
        assignees,
    ).await?;

    Ok(())
}
