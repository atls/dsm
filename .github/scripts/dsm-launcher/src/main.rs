use anyhow::Result;
use application::commands::close_issue::CloseIssueCommand;
use application::commands::create_issue::CreateIssueCommand;
use application::queries::get_issue_types::GetIssueTypes;
use application::queries::get_issues::GetIssuesQuery;
use application::queries::get_org::GetOrgQuery;
use application::queries::get_repo::GetRepoQuery;
use application::queries::get_team::GetTeamQuery;
use application::queries::get_team_members::GetTeamMembersQuery;
use chrono::{FixedOffset, Utc};
use infrastructure::github_adapter::GitHubAdapter;
use std::env;
use std::fs;
use std::rc::Rc;

mod graphql_queries {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/graphql/mod.rs"));
}

mod application;
mod domain;
mod infrastructure;

use application::use_cases::{close_issues::close_issues, create_issue::create_issue};
use infrastructure::github_graphql_client::GitHubGraphQLClient;

const DSM_TEAM_SLUG: &str = "DSM";
const DSM_TITLE_DATE_FORMAT: &str = "%a %b %d %Y";
const MOSCOW_UTC_OFFSET_SECONDS: i32 = 3 * 60 * 60;

#[tokio::main]
async fn main() -> Result<()> {
    let repo_owner = env::var("GITHUB_REPO_OWNER")?;
    let repo_name = env::var("GITHUB_REPO_NAME")?;
    let body = fs::read_to_string("./.github/ISSUE_TEMPLATE/dsm.md")?;
    let moscow_offset =
        FixedOffset::east_opt(MOSCOW_UTC_OFFSET_SECONDS).expect("Moscow UTC offset must be valid");
    let title: String = format!(
        "[DSM] {}",
        Utc::now()
            .with_timezone(&moscow_offset)
            .date_naive()
            .format(DSM_TITLE_DATE_FORMAT)
    );

    let client = GitHubGraphQLClient::new("dsm-launcher".to_string(), env::var("GITHUB_TOKEN")?)?;

    let adapter = Rc::new(GitHubAdapter::new(client.clone()));

    let get_org = GetOrgQuery {
        repo: adapter.clone(),
    };
    let get_repo = GetRepoQuery {
        repo: adapter.clone(),
    };
    let get_issues = GetIssuesQuery {
        repo: adapter.clone(),
    };
    let get_issue_types = GetIssueTypes {
        repo: adapter.clone(),
    };
    let close_issue = CloseIssueCommand {
        repo: adapter.clone(),
    };
    let create_issue_ = CreateIssueCommand {
        repo: adapter.clone(),
    };
    let get_team = GetTeamQuery {
        repo: adapter.clone(),
    };
    let get_team_members = GetTeamMembersQuery { repo: adapter };

    let org_id = get_org.execute(&repo_owner).await?;
    let repo_id = get_repo.execute(&org_id, &repo_name).await?;
    let current_issue_id = get_issues
        .clone()
        .execute(&repo_id)
        .await?
        .into_iter()
        .find(|issue| issue.title == title)
        .map(|issue| issue.id.to_string());

    close_issues(
        get_org.clone(),
        get_repo.clone(),
        get_issues,
        close_issue,
        &repo_owner,
        &repo_name,
        current_issue_id.as_deref(),
    )
    .await?;

    if current_issue_id.is_none() {
        create_issue(
            get_org,
            get_repo,
            get_team,
            get_team_members,
            get_issue_types,
            create_issue_,
            &repo_owner,
            &repo_name,
            DSM_TEAM_SLUG,
            DSM_TEAM_SLUG,
            &title,
            &body,
        )
        .await?;
    }

    Ok(())
}
