use async_trait::async_trait;
use anyhow::{Result, anyhow, Ok};

use crate::{
    domain::{
        repo::RepoId,
        org::OrgId,
        repository::OrgRepository
    }, 
    graphql_queries::{
        get_org::{
            get_org::Variables as GetOrgVars, 
            GetOrg
        }, get_repo::{
            get_repo::{
                GetRepoNode, 
                Variables as GetRepoVars
            }, 
            GetRepo
        }
    }
};

use super::GitHubAdapter;

#[async_trait]
impl OrgRepository for GitHubAdapter {
    async fn get_org(&self, owner: &str) -> Result<OrgId> {
        let vars = GetOrgVars {
            org: owner.to_string()
        };
        
        let response = self.client.execute::<GetOrg>(vars)
            .await?;

        if let Some(errors) = response.errors {
            return Err(anyhow!("GraphQL error: {:?}", errors));
        }

        let response_data = response.data.ok_or_else(|| anyhow!("get_org returned an empty response"))?;
        let org = response_data.organization.ok_or_else(|| anyhow!("No organization was found"))?;

        Ok(OrgId::new(org.id))
    }

    async fn get_repo(&self, org_id: &OrgId, repo_name: &str) -> Result<RepoId> {
        let vars = GetRepoVars {
            id: org_id.to_string(),
            repo: repo_name.to_string(),
        };

        let response = self.client.execute::<GetRepo>(vars)
            .await?;

        if let Some(errors) = response.errors {
            return Err(anyhow!("GraphQL error: {:?}", errors));
        }

        let response_data = response.data.ok_or_else(|| anyhow!("get_repo returned an empty response"))?;
        let node = response_data.node.ok_or_else(|| anyhow!("No repository node data was found"))?;
        let org = match node {
            GetRepoNode::Organization(org) => org,
            _ => {
                return Err(anyhow!("No organization was found"));
            }
        };
        let repo = org.repository.ok_or_else(|| anyhow!("No repository was found in the org"))?;

        Ok(RepoId::new(repo.id))
    }
}

