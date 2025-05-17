use anyhow::Result;
use graphql_client::GraphQLQuery;
use reqwest::Client;
use std::env;
use std::fs;

use crate::graphql_queries::create_issue::{
    create_issue::Variables as CreateIssueVars, CreateIssue,
};

pub async fn create_issue(
    repo_id: String,
    assignees: Vec<String>,
) -> Result<()> {
    let client = Client::builder().user_agent("dsm-launcher").build()?;
    let github_token = env::var("GITHUB_TOKEN")?;
    let team_slug = "DSM".to_string();

    let body = fs::read_to_string("./.github/ISSUE_TEMPLATE/dsm.md")?;
    let title: String = format!(
        "[DSM] {}",
        chrono::Utc::now().date_naive().format("%a %b %d %Y")
    );

    let create_mut = CreateIssue::build_query(CreateIssueVars {
        repo_id,
        title,
        type_: team_slug,
        assignees,
        body,
    });

    client
        .post("https://api.github.com/graphql")
        .bearer_auth(github_token)
        .json(&create_mut)
        .send()
        .await?;

    Ok(())
}
