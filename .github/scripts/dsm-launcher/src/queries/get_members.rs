use anyhow::{anyhow, Result};
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;
use std::env;

use crate::graphql_queries::get_team_members::{
    get_team_members::{self, Variables as GetTeamMembersVars},
    GetTeamMembers,
};

pub async fn get_members() -> Result<Vec<String>> {
    let client = Client::builder().user_agent("dsm-launcher").build()?;
    let github_token = env::var("GITHUB_TOKEN")?;
    let repo_owner = env::var("GITHUB_REPO_OWNER")?;
    let team_slug = "DSM".to_string();

    let team_query = GetTeamMembers::build_query(GetTeamMembersVars {
        org: repo_owner,
        team_slug: team_slug,
    });

    let res = client
        .post("https://api.github.com/graphql")
        .bearer_auth(github_token)
        .json(&team_query)
        .send()
        .await?
        .json::<Response<get_team_members::ResponseData>>()
        .await?;

    let org_data = res
        .data
        .unwrap()
        .organization
        .ok_or_else(|| anyhow!("Organization not found"))?;

    let members = org_data
        .team
        .ok_or_else(|| anyhow!("Team not found"))?
        .members
        .nodes
        .unwrap();

    let assignees = members
        .iter()
        .filter_map(|x| x.as_ref().and_then(|y| Some(y.login.clone())))
        .collect::<Vec<String>>();

    Ok(assignees)
}
