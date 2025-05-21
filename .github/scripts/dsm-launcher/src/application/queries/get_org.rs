use anyhow::Result;

use crate::domain::{org::OrgId, repository::OrgRepository};


#[derive(Clone)]
pub struct GetOrgQuery<R: OrgRepository> {
    pub repo: R,
}

impl<R: OrgRepository> GetOrgQuery<R> {
    pub async fn execute(&self, owner:  &str) -> Result<OrgId> {
        self.repo.get_org(owner).await
    }
}
