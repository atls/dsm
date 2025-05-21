use async_trait::async_trait;
use anyhow::{Result, anyhow, Ok};

use crate::{domain::{org::OrgId, repo::RepoId, repository::OrgRepository}, graphql_queries::{get_org::{get_org::Variables as GetOrgVars, GetOrg}, get_repo::{get_repo::{GetRepoNode, Variables as GetRepoVars}, GetRepo}}, infrastructure::github_graphql_client::GitHubGraphQLClient};


pub struct GitHubOrgAdapter {
    pub client: GitHubGraphQLClient,
}

#[async_trait]
impl OrgRepository for GitHubOrgAdapter {
    async fn get_org(&self, owner: &str) -> Result<OrgId> {
        let vars = GetOrgVars {
            org: owner.to_string()
        };
        
        let response = self.client.execute::<GetOrg>(vars)
            .await?
            .data;

        if let Some(response_data) = response {
            if let Some(org) = response_data.organization {
                return Ok(OrgId::new(org.id));
            }
        }

        Err(anyhow!("No organization found"))
    }

    async fn get_repo(&self, org_id: &OrgId, repo_name: &str) -> Result<RepoId> {
        let vars = GetRepoVars {
            id: org_id.as_str().to_string(),
            repo: repo_name.to_string(),
        };

        let response = self.client.execute::<GetRepo>(vars)
            .await?
            .data;

        if let Some(response_data) = response {
            match response_data.node {
                Some(GetRepoNode::Organization(x)) => {
                    if let Some(repo) = x.repository {
                        return Ok(RepoId::new(repo.id));
                    } else {
                        return Err(anyhow!("No org data found"));
                    }
                },
                _ => {
                    return Err(anyhow!("No organization found"));
                }
            }
        }

        Err(anyhow!("Failed to recieve response"))      
    }
}