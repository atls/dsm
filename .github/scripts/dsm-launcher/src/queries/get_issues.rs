use anyhow::Result;
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;

use crate::graphql_queries::get_open_issues::{
    get_open_issues::{
        self, GetOpenIssuesRepositoryIssuesNodes as IssuesNodes, Variables as GetIssuesVars,
    },
    GetOpenIssues,
};

pub async fn get_issues(
    repo_owner: String,
    repo_name: String,
    team_slug: String,
    client: &Client,
    github_token: &str,
) -> Result<(String, Vec<Option<IssuesNodes>>), reqwest::Error> {
    let issues_query = GetOpenIssues::build_query(GetIssuesVars {
        owner: repo_owner,
        repo: repo_name,
        team_slug: team_slug.clone(),
    });

    let res = client
        .post("https://api.github.com/graphql")
        .bearer_auth(github_token)
        .json(&issues_query)
        .send()
        .await?
        .json::<Response<get_open_issues::ResponseData>>()
        .await?;

    let repo_data = res.data.unwrap().repository.unwrap();
    let issues = repo_data.issues.nodes.unwrap();
    let repo_id = repo_data.id;

    Ok((repo_id, issues))
}
