use anyhow::Result;

use crate::domain::{org::OrgId, repository::OrgRepository};


pub struct GetOrgQuery<R: OrgRepository> {
    repo: R,
}

impl<R: OrgRepository> GetOrgQuery<R> {
    pub async fn execute(&self, owner:  &str) -> Result<OrgId> {
        self.repo.get_org(owner).await
    }
}
