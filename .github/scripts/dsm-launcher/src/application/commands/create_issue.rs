use anyhow::Result;
use std::rc::Rc;

use crate::domain::{issue::{Issue, IssueId}, repository::IssueRepository};

#[derive(Clone)]
pub struct CreateIssueCommand<R: IssueRepository> {
    pub repo: Rc<R>,
}

impl<R: IssueRepository> CreateIssueCommand<R> {
    pub async fn execute(&self, issue: Issue) -> Result<IssueId> {
        self.repo.create_issue(issue).await
    }
}
