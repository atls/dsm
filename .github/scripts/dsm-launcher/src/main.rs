use anyhow::Result;
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;
use std::env;
use std::fs;

mod graphql_queries {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/graphql/mod.rs"));
}

use graphql_queries::{
    get_open_issues_and_org::{get_open_issues_and_org::Variables as GetIssuesVars, get_open_issues_and_org, GetOpenIssuesAndOrg},
    close_issue::{close_issue::Variables as CloseIssueVars, CloseIssue},
    create_issue::{create_issue::Variables as CreateIssueVars, CreateIssue},
};

#[tokio::main]
async fn main() -> Result<()> {
    let github_token = env::var("GITHUB_TOKEN")?;
    let repo_owner = env::var("GITHUB_REPO_OWNER")?;
    let repo_name = env::var("GITHUB_REPO_NAME")?;
    let team_slug = "DSM".to_string();

    let client = Client::builder().user_agent("dsm-launcher").build()?;

    let issues_query = GetOpenIssuesAndOrg::build_query(GetIssuesVars {
        owner: repo_owner.clone(),
        repo: repo_name,
        team_slug: team_slug.clone(),
    });

    let res = client
        .post("https://api.github.com/graphql")
        .bearer_auth(&github_token)
        .json(&issues_query)
        .send()
        .await?
        .json::<Response<get_open_issues_and_org::ResponseData>>()
        .await?
        .data
        .unwrap();

    let repo_data = res.repository.unwrap();
    let repo_id = repo_data.id;
    let issues = repo_data.issues.nodes.unwrap();

    for issue in &issues {
        if let Some(id) = issue.as_ref().and_then(|i| Some(i.id.clone())) {
            let close_mut = CloseIssue::build_query(CloseIssueVars { id });
            client
                .post("https://api.github.com/graphql")
                .bearer_auth(&github_token)
                .json(&close_mut)
                .send()
                .await?;
        }
    }

    let org_data = res.organization.unwrap();
    let members = org_data.team.unwrap().members.nodes.unwrap();

    let assignees = members
        .iter()
        .filter_map(|x| {
            x.as_ref().and_then(|y| Some(y.login.clone()))
        })
        .collect::<Vec<String>>();

    let body = fs::read_to_string("./.github/ISSUE_TEMPLATE/dsm.md")?;
    let title = format!("[DSM] {}", chrono::Utc::now().date_naive().format("%a %b %d %Y"));

    let create_mut = CreateIssue::build_query(CreateIssueVars {
        repo_id,
        title,
        type_: team_slug,
        assignees,
        body,
    });

    client
        .post("https://api.github.com/graphql")
        .bearer_auth(&github_token)
        .json(&create_mut)
        .send()
        .await?;

    Ok(())
}
