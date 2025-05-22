use anyhow::{Ok, Result};

use crate::application::{
    commands::close_issue::CloseIssueCommand,
    queries::{
        get_issues::GetIssuesQuery,
        get_repo::GetRepoQuery,
        get_org::GetOrgQuery
    },
};
use crate::domain::repository::{
    IssueRepository, 
    OrgRepository
};

pub async fn close_issues<R: OrgRepository, I: IssueRepository>(
    get_org: GetOrgQuery<R>,
    get_repo: GetRepoQuery<R>,
    get_issues: GetIssuesQuery<I>,
    close_issue: CloseIssueCommand<I>,
    owner: &str,
    repo_name: &str,
) -> Result<()> {
    let org_id = get_org.execute(owner).await?;
    let repo_id = get_repo.execute(&org_id, &repo_name).await?;
    let issues = get_issues.execute(&repo_id).await?;

    for issue in issues.iter() {
        close_issue.execute(issue).await?;
    }

    Ok(())
}