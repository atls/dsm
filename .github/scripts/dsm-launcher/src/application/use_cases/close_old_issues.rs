use std::env;
use anyhow::{Ok, Result};

use crate::domain::repo::RepoId;
use crate::domain::repository::{IssueRepository, OrgRepository};
use crate::infrastructure::adapters::github_issue_adapter::GitHubIssueAdapter;
use crate::infrastructure::adapters::github_org_adapter::GitHubOrgAdapter;
use crate::infrastructure::github_graphql_client::GitHubGraphQLClient;


pub async fn close_old_issues() -> Result<()> {
    let client = GitHubGraphQLClient::new(
        "dsm-launcher".to_string(),
        env::var("GITHUB_TOKEN")?,
    )?;

    let repo_owner = env::var("GITHUB_REPO_OWNER")?;
    let repo_name = env::var("GITHUB_REPO_NAME")?;

    let issue_adapter = GitHubIssueAdapter { client: client.clone() };
    let org_adapter = GitHubOrgAdapter { client };

    let org_id = org_adapter.get_org(&repo_owner).await?;
    let repo_id: RepoId = org_adapter.get_repo(&org_id, &repo_name).await?;
    let issues = issue_adapter.get_issues(&repo_id).await?;

    for issue in issues.iter() {
        issue_adapter.close_issue(issue).await?;
    }

    Ok(())
}