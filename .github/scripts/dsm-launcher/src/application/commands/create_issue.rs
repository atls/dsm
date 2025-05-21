use anyhow::Result;

use crate::domain::{issue::{Issue, IssueId}, repository::IssueRepository};

#[derive(Clone)]
pub struct CreateIssueCommand<R: IssueRepository> {
    pub repo: R,
}

impl<R: IssueRepository> CreateIssueCommand<R> {
    pub async fn execute(&self, issue: Issue) -> Result<IssueId> {
        self.repo.create_issue(issue).await
    }
}
