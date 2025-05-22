use super::github_graphql_client::GitHubGraphQLClient;

mod issue_impl;
mod member_impl;
mod org_impl;

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
