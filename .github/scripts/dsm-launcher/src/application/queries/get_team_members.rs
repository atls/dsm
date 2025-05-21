use anyhow::{anyhow, Result};
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;
use std::env;

use crate::{domain::{member::Member, org::OrgId, repo::RepoId, repository::MemberRepository, team::TeamId}, graphql_queries::get_team_members::{
    get_team_members::{self, GetTeamMembersNode, Variables as GetTeamMembersVars},
    GetTeamMembers,
}};

pub async fn get_members(
    id: String
) -> Result<Vec<String>> {
    let client = Client::builder().user_agent("dsm-launcher").build()?;
    let github_token = env::var("GITHUB_TOKEN")?;

    let team_query = GetTeamMembers::build_query(GetTeamMembersVars {
        id
    });

    let res = client
        .post("https://api.github.com/graphql")
        .bearer_auth(github_token)
        .json(&team_query)
        .send()
        .await?
        .json::<Response<get_team_members::ResponseData>>()
        .await?;

    let team_members_node = res
        .data
        .unwrap()
        .node
        .unwrap();

    let members = match team_members_node {
        GetTeamMembersNode::Team(x) => x,
        _ => {
            return Err(anyhow!("No members found"));
        }
    }
        .members
        .nodes
        .unwrap();


    let assignees = members
        .iter()
        .filter_map(|x| x.as_ref().and_then(|y| Some(y.login.clone())))
        .collect::<Vec<String>>();

    Ok(assignees)
}



pub struct GetTeamMembersQuery<R: MemberRepository> {
    repo: R,
}

impl<R: MemberRepository> GetTeamMembersQuery<R> {
    pub async fn execute(&self, team_id: &TeamId) -> Result<Vec<Member>> {
        self.repo.get_team_members(team_id).await
    }
}
