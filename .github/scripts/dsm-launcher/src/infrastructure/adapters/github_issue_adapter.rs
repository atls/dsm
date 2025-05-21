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

pub struct GitHubIssueAdapter {
    pub client: GitHubGraphQLClient,
}

#[async_trait]
impl IssueRepository for GitHubIssueAdapter {
    async fn get_issues(&self, repo_id: &RepoId) -> Result<Vec<IssueId>> {
        let vars = GetOpenIssuesVars {
            id: repo_id.as_str().to_string()
        };

        let response = self.client.execute::<GetOpenIssues>(vars).await?;

        if let Some(repo_data) = response.data {
            if let Some(node) = repo_data.node {
                let issues = match node {
                    GetOpenIssuesNode::Repository(x) => x,
                    _ => {
                        return Err(anyhow!("No issues found"));
                    }
                }.issues.nodes;
                
                if let Some(issues) = issues {
                    return Ok(issues
                        .into_iter()
                        .filter_map(|x| {
                            if let Some(y) = x {
                                return Some(IssueId::new(y.id))
                            }
                            None
                        } 
                    )
                    .collect::<Vec<IssueId>>());

                }
            }
        }

        Err(anyhow!("No response data was found"))
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

        if let Some(data) = response.data {
            if let Some(create_issue) = data.create_issue {
                if let Some(issue) = create_issue.issue {
                    return Ok(IssueId::new(issue.id));
                }
            }

            return Err(anyhow!("Failed to create an issue"));
        }

        Err(anyhow!("Failed to send GraphQL request"))
    }

    async fn close_issue(&self, issue_id: &IssueId) -> Result<()> {
        let vars = CloseIssueVars {
            id: issue_id.as_str().to_string()
        };

        self.client.execute::<CloseIssue>(vars).await?;

        Ok(())
    }
}
