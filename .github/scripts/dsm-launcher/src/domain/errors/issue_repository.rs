use thiserror::Error;

#[derive(Debug, Error)]
pub enum IssueRepositoryError {
    #[error("No repository data was found")]
    RepoDataNotFound,

    #[error("No repository node data was found")]
    RepoNodeNotFound,

    #[error("Unexpected node type: expected Repository")]
    UnexpectedNodeType,

    #[error("No issues found in repository")]
    IssuesWereNotFound,

    #[error("Empty response from create_issue")]
    EmptyCreateIssueResponse,

    #[error("No created issue was found")]
    CreatedIssueNotFound,

    #[error("No created issue body was found")]
    CreatedIssueBodyNotFound,
}