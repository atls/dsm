use anyhow::Result;
use reqwest::Client;
use std::env;
use std::fs;

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
    let github_token = env::var("GITHUB_TOKEN")?;
    let repo_owner = env::var("GITHUB_REPO_OWNER")?;
    let repo_name = env::var("GITHUB_REPO_NAME")?;
    let team_slug = "DSM".to_string();
    let body = fs::read_to_string("./.github/ISSUE_TEMPLATE/dsm.md")?;
    let title = format!(
        "[DSM] {}",
        chrono::Utc::now().date_naive().format("%a %b %d %Y")
    );

    let client = Client::builder().user_agent("dsm-launcher").build()?;

    let (repo_id, issues) = get_issues(
        repo_owner.clone(),
        repo_name,
        team_slug.clone(),
        &client,
        &github_token,
    ).await?;

    close_issue(
        issues, 
        &github_token, 
        &client
    ).await?;

    let assignees = get_members(
        repo_owner, 
        team_slug.clone(), 
        &github_token, 
        &client
    ).await?;

    create_issue(
        repo_id,
        title,
        team_slug,
        assignees,
        body,
        &github_token,
        &client,
    ).await?;

    Ok(())
}
