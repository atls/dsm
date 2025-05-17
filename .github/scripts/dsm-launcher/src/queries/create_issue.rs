use anyhow::Result;
use graphql_client::GraphQLQuery;
use reqwest::Client;

use crate::graphql_queries::create_issue::{
    create_issue::Variables as CreateIssueVars, CreateIssue,
};

pub async fn create_issue(
    repo_id: String,
    title: String,
    team_slug: String,
    assignees: Vec<String>,
    body: String,
    github_token: &str,
    client: &Client,
) -> Result<()> {
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
