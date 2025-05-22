use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrgRepositoryError {
    #[error("Empty response from get_org")]
    EmptyGetOrgResponse,

    #[error("Organization not found")]
    OrgNotFound,

    #[error("Empty response from get_repo")]
    EmptyGetRepoResponse,

    #[error("Repository node not found")]
    RepoNodeNotFound,

    #[error("Repository not found in the organization")]
    RepoNotFound,
}