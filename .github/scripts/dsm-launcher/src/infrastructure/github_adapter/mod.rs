use super::github_graphql_client::GitHubGraphQLClient;

mod issue_adapter;
mod member_adapter;
mod org_adapter;
mod errors;

#[derive(Clone)]
pub struct GitHubAdapter {
    pub client: GitHubGraphQLClient,
}

impl GitHubAdapter {
    pub fn new(client: GitHubGraphQLClient) -> Self {
        GitHubAdapter {
            client
        }
    }
}
