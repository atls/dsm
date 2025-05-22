use anyhow::Result;
use application::commands::close_issue::CloseIssueCommand;
use application::commands::create_issue::CreateIssueCommand;
use application::queries::get_issues::GetIssuesQuery;
use application::queries::get_org::GetOrgQuery;
use application::queries::get_repo::GetRepoQuery;
use application::queries::get_team::GetTeamQuery;
use application::queries::get_team_members::GetTeamMembersQuery;
use infrastructure::github_adapter::GitHubAdapter;
use std::fs;
use std::env;
use std::rc::Rc;

mod graphql_queries {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/graphql/mod.rs"));
}

mod application;
mod infrastructure;
mod domain;

use infrastructure::github_graphql_client::GitHubGraphQLClient;
use application::use_cases::{
    close_issues::close_issues,
    create_issue::create_issue,
};

#[tokio::main]
async fn main() -> Result<()> {
    let repo_owner = env::var("GITHUB_REPO_OWNER")?;
    let repo_name = env::var("GITHUB_REPO_NAME")?;
    let team_slug = "DSM";
    let body = fs::read_to_string("./.github/ISSUE_TEMPLATE/dsm.md")?;
    let title: String = format!(
        "[DSM] {}",
        chrono::Utc::now().date_naive().format("%a %b %d %Y")
    );

    let client = GitHubGraphQLClient::new(
        "dsm-launcher".to_string(),
        env::var("GITHUB_TOKEN")?,
    )?;
    
    let adapter = Rc::new(GitHubAdapter::new(client.clone()));

    let get_org = GetOrgQuery {
        repo: adapter.clone()
    };
    let get_repo = GetRepoQuery {
        repo: adapter.clone()
    };
    let get_issues = GetIssuesQuery {
        repo: adapter.clone()
    };
    let close_issue = CloseIssueCommand {
        repo: adapter.clone()
    };
    let create_issue_ = CreateIssueCommand {
        repo: adapter.clone()
    };
    let get_team = GetTeamQuery {
        repo: adapter.clone()
    };
    let get_team_members = GetTeamMembersQuery {
        repo: adapter
    };

    close_issues(
        get_org.clone(),
        get_repo.clone(),
        get_issues,
        close_issue,
        &repo_owner,
        &repo_name
    ).await?;
    create_issue(
        get_org,
        get_repo,
        get_team,
        get_team_members,
        create_issue_,
        &repo_owner,
        &repo_name,
        team_slug,
        &title,
        &body
    ).await?;

    Ok(())
}
