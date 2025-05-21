use async_trait::async_trait;
use anyhow::{anyhow, Ok, Result};

use crate::domain::issue::IssueId;
use crate::domain::repo::RepoId;
use crate::domain::{
    repository::IssueRepository,
    issue::Issue,
};
use crate::graphql_queries::close_issue::{CloseIssue, close_issue::Variables as CloseIssueVars};
use crate::graphql_queries::create_issue::{CreateIssue, create_issue::Variables as CreateIssueVars};
use crate::graphql_queries::get_open_issues::{GetOpenIssues, get_open_issues::{Variables as GetOpenIssuesVars, GetOpenIssuesNode},};
use crate::infrastructure::{
    github_graphql_client::GitHubGraphQLClient
};

#[derive(Clone)]
pub struct GitHubIssueAdapter {
    pub client: GitHubGraphQLClient,
}

impl GitHubIssueAdapter {
    pub fn new(client: GitHubGraphQLClient) -> Self {
        GitHubIssueAdapter {
            client
        }
    }
}

#[async_trait]
impl IssueRepository for GitHubIssueAdapter {
    async fn get_issues(&self, repo_id: &RepoId) -> Result<Vec<IssueId>> {
        let vars = GetOpenIssuesVars {
            id: repo_id.to_string()
        };

        let response = self.client.execute::<GetOpenIssues>(vars).await?;

        if let Some(errors) = response.errors {
            return Err(anyhow!("GraphQL error: {:?}", errors));
        }

        let repo_data = response.data.ok_or_else(|| anyhow!("No repository data was found"))?;
        let node = repo_data.node.ok_or_else(|| anyhow!("No repository node data was found"))?;
        let issues = match node {
            GetOpenIssuesNode::Repository(x) => x,
            _ => {
                return Err(anyhow!("No issues found"));
            }
        }.issues.nodes;

        Ok(issues
            .ok_or_else(|| anyhow!("No issues found"))?
            .into_iter()
            .filter_map(|x| {
                if let Some(y) = x {
                    return Some(IssueId::new(y.id))
                }
                None
            })
            .collect::<Vec<IssueId>>())
    }

    async fn create_issue(&self, issue: Issue) -> Result<IssueId> {
        let logins: Vec<String> = issue.assignees
            .into_iter()
            .map(|x| x.login)
            .collect();

        let vars = CreateIssueVars {
            repo_id: issue.repo_id,
            title: issue.title,
            body: issue.body,
            assignees: logins,
            type_: issue.team_slug
        };

        let response = self.client.execute::<CreateIssue>(vars).await?;

        if let Some(errors) = response.errors {
            return Err(anyhow!("GraphQL error: {:?}", errors));
        }

        let response_data = response.data.ok_or_else(|| anyhow!("Create issue returned an empty body"))?;
        let issue = response_data.create_issue.ok_or_else(|| anyhow!("No created issue was found"))?;

        Ok(IssueId::new(
            issue
            .issue
            .ok_or_else(|| anyhow!("No created issue body was found"))?
            .id
        ))
    }

    async fn close_issue(&self, issue_id: &IssueId) -> Result<()> {
        let vars = CloseIssueVars {
            id: issue_id.to_string()
        };

        let response = self.client.execute::<CloseIssue>(vars).await?;

        if let Some(errors) = response.errors {
            return Err(anyhow!("GraphQL error: {:?}", errors));
        }

        Ok(())
    }
}
