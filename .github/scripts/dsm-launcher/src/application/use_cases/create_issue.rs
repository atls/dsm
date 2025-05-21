use std::{env, fs};
use anyhow::{Ok, Result};

use crate::domain::issue::Issue;
use crate::domain::repo::RepoId;
use crate::domain::repository::{IssueRepository, MemberRepository, OrgRepository};
use crate::infrastructure::adapters::github_issue_adapter::GitHubIssueAdapter;
use crate::infrastructure::adapters::github_member_adapter::GitHubMemberAdapter;
use crate::infrastructure::adapters::github_org_adapter::GitHubOrgAdapter;
use crate::infrastructure::github_graphql_client::GitHubGraphQLClient;

pub async fn create_issue() -> Result<()> {
    let client = GitHubGraphQLClient::new(
        "dsm-launcher".to_string(),
        env::var("GITHUB_TOKEN")?,
    )?;

    let repo_owner = env::var("GITHUB_REPO_OWNER")?;
    let repo_name = env::var("GITHUB_REPO_NAME")?;
    let team_slug = "DSM".to_string();

    let body = fs::read_to_string("./.github/ISSUE_TEMPLATE/dsm.md")?;
    let title: String = format!(
        "[DSM] {}",
        chrono::Utc::now().date_naive().format("%a %b %d %Y")
    );

    let issue_adapter = GitHubIssueAdapter { client: client.clone() };
    let org_adapter = GitHubOrgAdapter { client: client.clone() };
    let member_adapter = GitHubMemberAdapter { client };

    let org_id = org_adapter.get_org(&repo_owner).await?;
    let repo_id: RepoId = org_adapter.get_repo(&org_id, &repo_name).await?;

    let team_id = member_adapter.get_team(&org_id, &team_slug).await?;
    let members = member_adapter.get_team_members(&team_id).await?;

    let new_issue = Issue {
        id: None,
        repo_id: repo_id.as_str().to_string(),
        title,
        body,
        team_slug,
        assignees: members,
    };

    issue_adapter.create_issue(new_issue).await?;

    Ok(())
}