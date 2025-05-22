use anyhow::Result;
use std::rc::Rc;

use crate::domain::{member::Member, repository::MemberRepository, team::TeamId};

#[derive(Clone)]
pub struct GetTeamMembersQuery<R: MemberRepository> {
    pub repo: Rc<R>,
}

impl<R: MemberRepository> GetTeamMembersQuery<R> {
    pub async fn execute(&self, team_id: &TeamId) -> Result<Vec<Member>> {
        self.repo.get_team_members(team_id).await
    }
}
