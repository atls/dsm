use thiserror::Error;

#[derive(Debug, Error)]
pub enum TeamError {
    #[error("No team node was found")]
    TeamNodeNotFound,

    #[error("get_team returned an empty response")]
    EmptyTeamResponse,

    #[error("No team node was found in the response data")]
    TeamResponseNodeNotFound,

    #[error("Empty respsonse for get_")]
    TeamNotFound,
}
