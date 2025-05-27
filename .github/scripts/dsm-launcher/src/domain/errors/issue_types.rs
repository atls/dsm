use thiserror::Error;

#[derive(Debug, Error)]
pub enum IssueTypesError {
    #[error("No issue types nodes were found")]
    IssueTypesNodeNotFound,

    #[error("No issue types were found")]
    IssueTypesNotFound,

    #[error("No issue type with the given name was found")]
    IssueTypeNotFound,
}
