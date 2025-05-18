use anyhow::{anyhow, Ok, Result};
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;
use std::env;

use crate::graphql_queries::get_repo::{
    get_repo::{self, Variables as GetRepoVars},
    GetRepo,
};

pub async fn get_repo() -> Result<String> {
    let client = Client::builder().user_agent("dsm-launcher").build()?;
    let github_token = env::var("GITHUB_TOKEN")?;
    let repo_owner = env::var("GITHUB_REPO_OWNER")?;
    let repo_name = env::var("GITHUB_REPO_NAME")?;

    let team_query = GetRepo::build_query(GetRepoVars {
        owner: repo_owner,
        repo: repo_name
    });

    let res = client
        .post("https://api.github.com/graphql")
        .bearer_auth(github_token)
        .json(&team_query)
        .send()
        .await?
        .json::<Response<get_repo::ResponseData>>()
        .await?;

    let org_id = res
        .data
        .unwrap()
        .repository
        .ok_or_else(|| anyhow!("Repository not found"))?
        .id;
    
    Ok(org_id)
}