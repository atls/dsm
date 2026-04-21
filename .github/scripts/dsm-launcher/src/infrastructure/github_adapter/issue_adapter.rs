use anyhow::{Ok, Result};
use async_trait::async_trait;

use crate::domain::{
    issue::{Issue, IssueId, IssueType, OpenIssue},
    repo::RepoId,
    repository::IssueRepository,
};
use crate::graphql_queries::{
    close_issue::{close_issue::Variables as CloseIssueVars, CloseIssue},
    create_issue::{create_issue::Variables as CreateIssueVars, CreateIssue},
    get_issue_types::{
        get_issue_types::{GetIssueTypesNode, Variables as GetIssueTypesVars},
        GetIssueTypes,
    },
    get_open_issues::{
        get_open_issues::{GetOpenIssuesNode, Variables as GetOpenIssuesVars},
        GetOpenIssues,
    },
};

use crate::domain::errors::{issue::IssueError, issue_types::IssueTypesError, repo::RepoError};

use super::{errors::GitHubAdapterError, GitHubAdapter};

#[async_trait]
impl IssueRepository for GitHubAdapter {
    async fn get_issues(&self, repo_id: &RepoId) -> Result<Vec<OpenIssue>> {
        let vars = GetOpenIssuesVars {
            id: repo_id.to_string(),
        };

        let response = self.client.execute::<GetOpenIssues>(vars).await?;

        if let Some(errors) = response.errors {
            return Err(GitHubAdapterError::GraphQL(errors).into());
        }

        let repo_data = response.data.ok_or(RepoError::RepoDataNotFound)?;
        let node = repo_data.node.ok_or(RepoError::RepoNodeNotFound)?;
        let issues = match node {
            GetOpenIssuesNode::Repository(x) => x,
            _ => {
                return Err(GitHubAdapterError::UnexpectedNodeType.into());
            }
        }
        .issues
        .nodes;

        Ok(issues
            .ok_or(IssueError::IssuesWereNotFound)?
            .into_iter()
            .filter_map(|x| x.map(|issue| OpenIssue::new(issue.id, issue.title)))
            .collect::<Vec<OpenIssue>>())
    }

    async fn get_issue_types(&self, repo_id: &RepoId) -> Result<Vec<IssueType>> {
        let vars = GetIssueTypesVars {
            id: repo_id.to_string(),
        };

        let response = self.client.execute::<GetIssueTypes>(vars).await?;

        if let Some(errors) = response.errors {
            return Err(GitHubAdapterError::GraphQL(errors).into());
        }

        let data = response.data.ok_or(RepoError::RepoDataNotFound)?;
        let node = data.node.ok_or(RepoError::RepoNodeNotFound)?;
        let issue_types = match node {
            GetIssueTypesNode::Repository(repo) => repo.issue_types,
            _ => return Err(GitHubAdapterError::UnexpectedNodeType.into()),
        };

        let issue_types = issue_types
            .ok_or(IssueTypesError::IssueTypesNodeNotFound)?
            .nodes
            .ok_or(IssueTypesError::IssueTypesNotFound)?
            .into_iter()
            .filter_map(|x| x.map(|y| IssueType::new(y.id, y.name)))
            .collect::<Vec<IssueType>>();

        Ok(issue_types)
    }

    async fn create_issue(&self, issue: Issue) -> Result<IssueId> {
        let logins: Vec<String> = issue.assignees.iter().map(|x| x.id.to_string()).collect();

        let vars = CreateIssueVars {
            repo_id: issue.repo_id,
            title: issue.title,
            body: issue.body,
            assignee_ids: logins,
            issue_type_id: issue.issue_type_id,
        };

        let response = self.client.execute::<CreateIssue>(vars).await?;

        if let Some(errors) = response.errors {
            return Err(GitHubAdapterError::GraphQL(errors).into());
        }

        let response_data = response.data.ok_or(IssueError::EmptyCreateIssueResponse)?;
        let issue = response_data
            .create_issue
            .ok_or(IssueError::CreatedIssueNotFound)?;

        Ok(IssueId::new(
            issue.issue.ok_or(IssueError::CreatedIssueBodyNotFound)?.id,
        ))
    }

    async fn close_issue(&self, issue_id: &IssueId) -> Result<()> {
        let vars = CloseIssueVars {
            id: issue_id.to_string(),
        };

        let response = self.client.execute::<CloseIssue>(vars).await?;

        if let Some(errors) = response.errors {
            return Err(GitHubAdapterError::GraphQL(errors).into());
        }

        Ok(())
    }
}
