use async_trait::async_trait;
use anyhow::{Result, anyhow, Ok};

use crate::{
    domain::{
        member::{Member, MemberId}, org::OrgId, repository::MemberRepository, team::TeamId
    }, graphql_queries::{get_team::{get_team::{GetTeamNode, Variables as GetTeamVars}, GetTeam}, get_team_members::{get_team_members::{GetTeamMembersNode, Variables as GetTeamMembersVars}, GetTeamMembers}}, infrastructure::github_graphql_client::GitHubGraphQLClient
};

pub struct GitHubMemberAdapter {
    pub client: GitHubGraphQLClient,
}

#[async_trait]
impl MemberRepository for GitHubMemberAdapter {
    async fn get_team_members(&self, team_id: &TeamId) -> Result<Vec<Member>> {
        let variables = GetTeamMembersVars {
            id: team_id.as_str().to_string()
        };


        let response = self.client.execute::<GetTeamMembers>(variables)
            .await?;

        match response.data {
            Some(data) => match data.node {
                Some(GetTeamMembersNode::Team(team)) => {
                    if let Some(members) = team.members.nodes {
                        return Ok(members
                            .into_iter()
                            .filter_map(|x| {
                                match x { 
                                    Some(member) => Some(Member { id: MemberId::new(member.id), login: member.login }),
                                    _ => None
                                }
                            })
                            .collect::<Vec<Member>>());
                    } else {
                        return Err(anyhow!("No team members data found"));
                    }
                },
                _ => {
                    return Err(anyhow!("No team data found"));
                }
            }
            None => {
                return Err(anyhow!("No team found"));
            }
        }

    }

    async fn get_team(&self, org_id: &OrgId, team_slug: &str) -> Result<TeamId> {
        let variables = GetTeamVars {
            id: org_id.as_str().to_string(),
            team_slug: team_slug.to_string()
        };

        let response = self.client.execute::<GetTeam>(variables)
            .await?;

        match response.data {
            Some(data) => match data.node {
                Some(GetTeamNode::Organization(org)) => {
                    if let Some(team) = org.team {
                        Ok(TeamId::new(team.id))
                    } else {
                        Err(anyhow!("No team found"))
                    }
                },
                _ => {
                    Err(anyhow!("No team data found"))
                }
            }
            None => {
                Err(anyhow!("No organization found"))
            }
        }
    }
}
