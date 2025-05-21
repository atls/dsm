use async_trait::async_trait;
use anyhow::{Result, anyhow, Ok};

use crate::{
    domain::{
        member::{Member, MemberId}, org::OrgId, repository::MemberRepository, team::TeamId
    }, graphql_queries::{get_team::{get_team::{GetTeamNode, Variables as GetTeamVars}, GetTeam}, get_team_members::{get_team_members::{GetTeamMembersNode, Variables as GetTeamMembersVars}, GetTeamMembers}}, infrastructure::github_graphql_client::GitHubGraphQLClient
};

#[derive(Clone)]
pub struct GitHubMemberAdapter {
    pub client: GitHubGraphQLClient,
}

impl GitHubMemberAdapter {
    pub fn new(client: GitHubGraphQLClient) -> Self {
        GitHubMemberAdapter {
            client
        }
    }
}

#[async_trait]
impl MemberRepository for GitHubMemberAdapter {
    async fn get_team_members(&self, team_id: &TeamId) -> Result<Vec<Member>> {
        let variables = GetTeamMembersVars {
            id: team_id.to_string()
        };


        let response = self.client.execute::<GetTeamMembers>(variables)
            .await?;

        if let Some(errors) = response.errors {
            return Err(anyhow!("GraphQL error: {:?}", errors));
        }

        let response_data = response.data.ok_or_else(|| anyhow!("get_team_members returned an empty response"))?;
        let node = response_data.node.ok_or_else(|| anyhow!("No team node was found"))?;
        let team = match node {
            GetTeamMembersNode::Team(team) => team,
            _ => {
                return Err(anyhow!("No team was found on the node"));
            }
        };
        let members = team.members.nodes.ok_or_else(|| anyhow!("No team members were found"))?;

        Ok(members
            .into_iter()
            .filter_map(|x| {
                if let Some(member) = x {
                    let id = MemberId::new(member.id);
                    let login = member.login;
                    
                    return Some(Member::new(id, login));
                }

                None
            })
            .collect::<Vec<Member>>()
        )
    }

    async fn get_team(&self, org_id: &OrgId, team_slug: &str) -> Result<TeamId> {
        let variables = GetTeamVars {
            id: org_id.to_string(),
            team_slug: team_slug.to_string()
        };

        let response = self.client.execute::<GetTeam>(variables)
            .await?;

        if let Some(errors) = response.errors {
            return Err(anyhow!("GraphQL error: {:?}", errors));
        }

        let response_data = response.data.ok_or_else(|| anyhow!("get_team returned empty response"))?;
        let node = response_data.node.ok_or_else(|| anyhow!("No team node was found in the response data"))?;
        let org = match node {
            GetTeamNode::Organization(org) => org,
            _ => {
                return Err(anyhow!("No organization was found")); 
            }  
        };
        let team = org.team.ok_or_else(|| anyhow!("No team was found"))?;

        Ok(TeamId::new(team.id))
    }
}
