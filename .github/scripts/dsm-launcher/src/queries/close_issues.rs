use anyhow::Result;
use graphql_client::GraphQLQuery;
use reqwest::Client;
use std::env;

use crate::graphql_queries::{
    close_issue::{close_issue::Variables as CloseIssueVars, CloseIssue},
    get_open_issues::get_open_issues::GetOpenIssuesRepositoryIssuesNodes as IssuesNodes,
};

pub async fn close_issue(
    issues: Vec<Option<IssuesNodes>>,
) -> Result<()> {
    let client = Client::builder().user_agent("dsm-launcher").build()?;
    let github_token = env::var("GITHUB_TOKEN")?;

    for issue in &issues {
        if let Some(issue) = issue {
            let id = issue.id.clone();
            let close_mut = CloseIssue::build_query(CloseIssueVars { id });
            client
                .post("https://api.github.com/graphq")
                .bearer_auth(&github_token)
                .json(&close_mut)
                .send()
                .await?;
        }
    }

    Ok(())
}
