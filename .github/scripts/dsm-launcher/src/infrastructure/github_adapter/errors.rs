use thiserror::Error;

#[derive(Debug, Error)]
pub enum GitHubAdapterError {
    #[error("GraphQL error: {0:?}")]
    GraphQL(Vec<graphql_client::Error>),

    #[error("Unexpected node type")]
    UnexpectedNodeType,
}