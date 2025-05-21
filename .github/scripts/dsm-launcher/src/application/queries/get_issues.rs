use anyhow::Result;

use crate::domain::{issue::IssueId, repo::RepoId, repository::IssueRepository};

pub struct GetIssuesQuery<R: IssueRepository> {
    repo: R,
}

impl<R: IssueRepository> GetIssuesQuery<R> {
    pub async fn execute(&self, repo_id: &RepoId) -> Result<Vec<IssueId>> {
        self.repo.get_issues(repo_id).await
    }
}
