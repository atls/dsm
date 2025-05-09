use anyhow::Result;
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;
use std::env;
use std::fs;

mod graphql_queries {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/graphql/mod.rs"));
}

use graphql_queries::{
    get_open_issues::{self, Variables as GetIssuesVars},
    close_issue::{self, Variables as CloseIssueVars},
    create_issue::{self, Variables as CreateIssueVars},
};

#[tokio::main]
async fn main() -> Result<()> {
    let github_token = env::var("GITHUB_TOKEN")?;
    let repo_owner = env::var("GITHUB_REPO_OWNER")?;
    let repo_name = env::var("GITHUB_REPO_NAME")?;

    let client = Client::builder().user_agent("dsm-launcher").build()?;

    let issues_query = get_open_issues::GetOpenIssues::build_query(GetIssuesVars {
        owner: repo_owner.clone(),
        repo: repo_name.clone(),
    });

    let res = client
        .post("https://api.github.com/graphql")
        .bearer_auth(&github_token)
        .json(&issues_query)
        .send()
        .await?
        .json::<Response<get_open_issues::ResponseData>>()
        .await?;

    let repo_data = res.data.unwrap().repository.unwrap();
    let repo_id = repo_data.id;
    let issues = repo_data.issues.nodes.unwrap();

    for issue in &issues {
        if let Some(id) = issue.as_ref().and_then(|i| i.id.clone()) {
            let close_mut = close_issue::CloseIssue::build_query(CloseIssueVars { id });
            client
                .post("https://api.github.com/graphql")
                .bearer_auth(&github_token)
                .json(&close_mut)
                .send()
                .await?;
        }
    }

    let body = fs::read_to_string(".github/ISSUE_TEMPLATE/dsm.md")?;
    let title = format!("[DSM] {}", chrono::Utc::now().date_naive());

    let create_mut = create_issue::CreateIssue::build_query(CreateIssueVars {
        repo_id,
        title,
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
