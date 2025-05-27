use anyhow::Result;
use std::rc::Rc;

use crate::domain::{org::OrgId, repository::MemberRepository, team::TeamId};

#[derive(Clone)]
pub struct GetTeamQuery<R: MemberRepository> {
    pub repo: Rc<R>,
}

impl<R: MemberRepository> GetTeamQuery<R> {
    pub async fn execute(&self, org_id: &OrgId, team_slug: &str) -> Result<TeamId> {
        self.repo.get_team(org_id, team_slug).await
    }
}