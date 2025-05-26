use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrgError {
    #[error("Empty response from get_org")]
    EmptyGetOrgResponse,

    #[error("Organization not found")]
    OrgNotFound,
}
