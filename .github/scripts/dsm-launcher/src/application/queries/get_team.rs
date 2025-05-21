use anyhow::Result;

use crate::domain::{member::Member, repository::MemberRepository, team::TeamId};

pub struct GetTeamQuery<R: MemberRepository> {
    repo: R,
}

impl<R: MemberRepository> GetTeamQuery<R> {
    pub async fn execute(&self, team_id: &TeamId) -> Result<Vec<Member>> {
        self.repo.get_team_members(team_id).await
    }
}
