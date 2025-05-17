use anyhow::{anyhow, Ok, Result};
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;
use std::env;

use crate::graphql_queries::get_team::{
    get_team::{self, Variables as GetTeamVars},
    GetTeam,
};

pub async fn get_team() -> Result<String> {
    let client = Client::builder().user_agent("dsm-launcher").build()?;
    let github_token = env::var("GITHUB_TOKEN")?;
    let repo_owner = env::var("GITHUB_REPO_OWNER")?;
    let team_slug = "DSM".to_string();

    let team_query = GetTeam::build_query(GetTeamVars {
        org: repo_owner,
        team_slug: team_slug,
    });

    let res = client
        .post("https://api.github.com/graphql")
        .bearer_auth(github_token)
        .json(&team_query)
        .send()
        .await?
        .json::<Response<get_team::ResponseData>>()
        .await?;

    let org_data = res
        .data
        .unwrap()
        .organization
        .ok_or_else(|| anyhow!("Organization not found"))?;

    let id = org_data
        .team
        .ok_or_else(|| anyhow!("Team not found"))?
        .id;
    
    Ok(id)
}