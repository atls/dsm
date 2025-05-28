use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("No repository data was found")]
    RepoDataNotFound,

    #[error("No repository node data was found")]
    RepoNodeNotFound,

    #[error("Empty response for get_repo")]
    EmptyGetRepoResponse,

    #[error("Repository not found")]
    RepoNotFound,
}
