use anyhow::Result;

use crate::domain::{
    org::OrgId, 
    repo::RepoId, 
    repository::OrgRepository
};

#[derive(Clone)]
pub struct GetRepoQuery<R: OrgRepository> {
    pub repo: R,
}

impl<R: OrgRepository> GetRepoQuery<R> {
    pub async fn execute(&self, org_id: &OrgId, repo_name:  &str) -> Result<RepoId> {
        self.repo.get_repo(org_id, repo_name).await
    }
}
