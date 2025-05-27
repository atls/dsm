use anyhow::Result;
use std::rc::Rc;

use crate::domain::{issue::IssueType, repo::RepoId, repository::IssueRepository};

#[derive(Clone)]
pub struct GetIssueTypes<R: IssueRepository> {
    pub repo: Rc<R>,
}

impl<R: IssueRepository> GetIssueTypes<R> {
    pub async fn execute(&self, repo_id: &RepoId) -> Result<Vec<IssueType>> {
        self.repo.get_issue_types(repo_id).await
    }
}
