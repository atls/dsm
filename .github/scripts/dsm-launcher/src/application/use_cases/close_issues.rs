use anyhow::{Ok, Result};
use std::ops::Deref;

use crate::application::{
    commands::close_issue::CloseIssueCommand,
    queries::{get_issues::GetIssuesQuery, get_org::GetOrgQuery, get_repo::GetRepoQuery},
};
use crate::domain::repository::{IssueRepository, OrgRepository};

pub async fn close_issues<R: OrgRepository, I: IssueRepository>(
    get_org: GetOrgQuery<R>,
    get_repo: GetRepoQuery<R>,
    get_issues: GetIssuesQuery<I>,
    close_issue: CloseIssueCommand<I>,
    owner: &str,
    repo_name: &str,
    keep_issue_id: Option<&str>,
) -> Result<()> {
    let org_id = get_org.execute(owner).await?;
    let repo_id = get_repo.execute(&org_id, &repo_name).await?;
    let issues = get_issues.execute(&repo_id).await?;

    for issue in issues.iter() {
        if keep_issue_id == Some(issue.id.deref()) {
            continue;
        }

        close_issue.execute(&issue.id).await?;
    }

    Ok(())
}
