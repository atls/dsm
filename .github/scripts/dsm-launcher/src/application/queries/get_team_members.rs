use anyhow::Result;
use crate::domain::{member::Member, repository::MemberRepository, team::TeamId};

pub struct GetTeamMembersQuery<R: MemberRepository> {
    repo: R,
}

impl<R: MemberRepository> GetTeamMembersQuery<R> {
    pub async fn execute(&self, team_id: &TeamId) -> Result<Vec<Member>> {
        self.repo.get_team_members(team_id).await
    }
}
