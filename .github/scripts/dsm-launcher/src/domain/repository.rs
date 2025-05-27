use async_trait::async_trait;
use anyhow::Result;

use super::{
    issue::{Issue, IssueId}, member::Member, org::OrgId, repo::RepoId, team::TeamId
};

#[async_trait]
pub trait IssueRepository {
    async fn get_issues(&self, repo: &RepoId) -> Result<Vec<IssueId>>; //RepoId IssueId
    async fn create_issue(&self, issue: Issue) -> Result<IssueId>;
    async fn close_issue(&self, issue_id: &IssueId) -> Result<()>;
}

#[async_trait]
pub trait MemberRepository {
    async fn get_team(&self, org_id: &OrgId, team_slug: &str) -> Result<TeamId>;
    async fn get_team_members(&self, team_id: &TeamId) -> Result<Vec<Member>>;
}

#[async_trait]
pub trait OrgRepository {
    async fn get_org(&self, owner: &str) -> Result<OrgId>;
    async fn get_repo(&self, org_id: &OrgId, repo_name: &str) -> Result<RepoId>;
}
