use anyhow::Result;

use crate::domain::{issue::IssueId, repository::IssueRepository};

pub struct CloseIssueCommand<R: IssueRepository> {
    repo: R,
}

impl<R: IssueRepository> CloseIssueCommand<R> {
    pub async fn execute(&self, issue_id: &IssueId) -> Result<()> {
        self.repo.close_issue(issue_id).await
    }
}
