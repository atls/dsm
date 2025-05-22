use async_trait::async_trait;
use anyhow::{Result, Ok};

use crate::{
    domain::{
        errors::member_repository::MemberRepositoryError,
        member::{
            Member, 
            MemberId
        }, 
        org::OrgId, 
        repository::MemberRepository, 
        team::TeamId
    }, 
    graphql_queries::{
        get_team::{
            get_team::{
                GetTeamNode, 
                Variables as GetTeamVars
            }, 
            GetTeam
        }, 
        get_team_members::{
            get_team_members::{
                GetTeamMembersNode, 
                Variables as GetTeamMembersVars
            }, 
            GetTeamMembers
        }
    }
};

use super::{
    errors::GitHubAdapterError,
    GitHubAdapter
};

#[async_trait]
impl MemberRepository for GitHubAdapter {
    async fn get_team_members(&self, team_id: &TeamId) -> Result<Vec<Member>> {
        let variables = GetTeamMembersVars {
            id: team_id.to_string()
        };

        let response = self.client.execute::<GetTeamMembers>(variables)
            .await?;

        if let Some(errors) = response.errors {
            return Err(GitHubAdapterError::GraphQL(errors).into());
        }

        let response_data = response.data.ok_or(MemberRepositoryError::EmptyTeamMembersResponse)?;
        let node = response_data.node.ok_or(MemberRepositoryError::TeamNodeNotFound)?;
        let team = match node {
            GetTeamMembersNode::Team(team) => team,
            _ => {
                return Err(MemberRepositoryError::TeamNotFound.into());
            }
        };
        let members = team.members.nodes.ok_or(MemberRepositoryError::TeamMembersWereNotFound)?;

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
            return Err(GitHubAdapterError::GraphQL(errors).into());
        }

        let response_data = response.data.ok_or(MemberRepositoryError::EmptyTeamResponse)?;
        let node = response_data.node.ok_or(MemberRepositoryError::TeamResponseNodeNotFound)?;
        let org = match node {
            GetTeamNode::Organization(org) => org,
            _ => {
                return Err(MemberRepositoryError::OrgNotFound.into()); 
            }  
        };
        let team = org.team.ok_or(MemberRepositoryError::TeamNotFound)?;

        Ok(TeamId::new(team.id))
    }
}

