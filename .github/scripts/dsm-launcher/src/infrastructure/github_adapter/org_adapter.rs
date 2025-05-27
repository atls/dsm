use async_trait::async_trait;
use anyhow::{Result, Ok};

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

use crate::domain::errors::{
    repo::RepoError,
    org::OrgError,
};

use super::{
    errors::GitHubAdapterError,
    GitHubAdapter
};

#[async_trait]
impl OrgRepository for GitHubAdapter {
    async fn get_org(&self, owner: &str) -> Result<OrgId> {
        let vars = GetOrgVars {
            org: owner.to_string()
        };
        
        let response = self.client.execute::<GetOrg>(vars)
            .await?;

        if let Some(errors) = response.errors {
            return Err(GitHubAdapterError::GraphQL(errors).into());
        }

        let response_data = response.data.ok_or(OrgError::EmptyGetOrgResponse)?;
        let org = response_data.organization.ok_or(OrgError::OrgNotFound)?;

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
            return Err(GitHubAdapterError::GraphQL(errors).into());
        }

        let response_data = response.data.ok_or(RepoError::EmptyGetRepoResponse)?;
        let node = response_data.node.ok_or(RepoError::RepoNodeNotFound)?;
        let org = match node {
            GetRepoNode::Organization(org) => org,
            _ => {
                return Err(GitHubAdapterError::UnexpectedNodeType.into());
            }
        };
        let repo = org.repository.ok_or(RepoError::RepoNotFound)?;

        Ok(RepoId::new(repo.id))
    }
}
