use anyhow::{anyhow, Ok, Result};
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;
use std::env;

use crate::graphql_queries::get_team::{
    get_team::{self, Variables as GetTeamVars, GetTeamNode},
    GetTeam,
};

pub async fn get_team(org_id: String) -> Result<String> {
    let client = Client::builder().user_agent("dsm-launcher").build()?;
    let github_token = env::var("GITHUB_TOKEN")?;
    let team_slug = "DSM".to_string();

    let team_query = GetTeam::build_query(GetTeamVars {
        id: org_id,
        team_slug
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
        .node;

    let id = match org_data {
        Some(GetTeamNode::Organization(x)) => x,
        _ => {
            return Err(anyhow!("No organization found"));
        }
    }
        .team
        .ok_or_else(|| anyhow!("Team not found"))?
        .id;
    
    Ok(id)
}