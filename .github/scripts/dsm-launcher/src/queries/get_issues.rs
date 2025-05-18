use anyhow::{Result, anyhow};
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;
use std::env;

use crate::graphql_queries::get_open_issues::{
    get_open_issues::{
        self, Variables as GetIssuesVars, GetOpenIssuesNode, GetOpenIssuesNodeOnRepositoryIssuesNodes as IssueNodes
    },
    GetOpenIssues,
};

pub async fn get_issues(repo_id: String) -> Result<Vec<Option<IssueNodes>>> {
    let client = Client::builder().user_agent("dsm-launcher").build()?;
    let github_token = env::var("GITHUB_TOKEN")?;

    let issues_query = GetOpenIssues::build_query(GetIssuesVars {
        id: repo_id
    });

    let res = client
        .post("https://api.github.com/graphql")
        .bearer_auth(github_token)
        .json(&issues_query)
        .send()
        .await?
        .json::<Response<get_open_issues::ResponseData>>()
        .await?;

    let repo_data = res.data.unwrap().node;

    let issues = match repo_data {
        Some(GetOpenIssuesNode::Repository(x)) => x,
        _ => {
            return Err(anyhow!("No issues found"));
        }
    }
        .issues
        .nodes
        .unwrap();

    Ok(issues)
}
