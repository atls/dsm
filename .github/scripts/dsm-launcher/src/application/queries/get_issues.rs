use anyhow::Result;
use std::rc::Rc;

use crate::domain::{issue::OpenIssue, repo::RepoId, repository::IssueRepository};

#[derive(Clone)]
pub struct GetIssuesQuery<R: IssueRepository> {
    pub repo: Rc<R>,
}

impl<R: IssueRepository> GetIssuesQuery<R> {
    pub async fn execute(&self, repo_id: &RepoId) -> Result<Vec<OpenIssue>> {
        self.repo.get_issues(repo_id).await
    }
}
