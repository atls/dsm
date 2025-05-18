use anyhow::{anyhow, Ok, Result};
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;
use std::env;

use crate::graphql_queries::get_org::{
    get_org::{self, Variables as GetOrgVars},
    GetOrg,
};

pub async fn get_org() -> Result<String> {
    let client = Client::builder().user_agent("dsm-launcher").build()?;
    let github_token = env::var("GITHUB_TOKEN")?;
    let repo_owner = env::var("GITHUB_REPO_OWNER")?;

    let team_query = GetOrg::build_query(GetOrgVars {
        org: repo_owner,
    });

    let res = client
        .post("https://api.github.com/graphql")
        .bearer_auth(github_token)
        .json(&team_query)
        .send()
        .await?
        .json::<Response<get_org::ResponseData>>()
        .await?;

    let org_id = res
        .data
        .unwrap()
        .organization
        .ok_or_else(|| anyhow!("Organization not found"))?
        .id;
    
    Ok(org_id)
}