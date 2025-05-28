use thiserror::Error;

#[derive(Debug, Error)]
pub enum IssueError {
    #[error("No issues found in repository")]
    IssuesWereNotFound,

    #[error("Empty response from create_issue")]
    EmptyCreateIssueResponse,

    #[error("No created issue was found")]
    CreatedIssueNotFound,

    #[error("No created issue body was found")]
    CreatedIssueBodyNotFound,
}
