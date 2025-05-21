use anyhow::Result;

use crate::domain::{issue::IssueId, repo::RepoId, repository::IssueRepository};

#[derive(Clone)]
pub struct GetIssuesQuery<R: IssueRepository> {
    pub repo: R,
}

impl<R: IssueRepository> GetIssuesQuery<R> {
    pub async fn execute(&self, repo_id: &RepoId) -> Result<Vec<IssueId>> {
        self.repo.get_issues(repo_id).await
    }
}
