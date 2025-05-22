use thiserror::Error;

#[derive(Debug, Error)]
pub enum MemberRepositoryError {
    #[error("get_team_members returned an empty response")]
    EmptyTeamMembersResponse,

    #[error("No team node was found")]
    TeamNodeNotFound,

    #[error("No team members were found")]
    TeamMembersWereNotFound,

    #[error("get_team returned an empty response")]
    EmptyTeamResponse,

    #[error("No team node was found in the response data")]
    TeamResponseNodeNotFound,

    #[error("No team was found in the organization")]
    TeamNotFound,

    #[error("No organization was found")]
    OrgNotFound,
}
