use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;
use anyhow::Result;

#[derive(Clone)]
pub struct GitHubGraphQLClient {
    pub client: Client,
    pub token: String,
}

impl GitHubGraphQLClient {
    pub fn new(user_agent: String, token: String) -> Result<GitHubGraphQLClient> {
        let client = Client::builder().user_agent(user_agent).build()?;

        Ok(GitHubGraphQLClient {
            client,
            token,
        })
    }

    pub async fn execute<Q: GraphQLQuery>(&self, variables: Q::Variables) -> Result<Response<Q::ResponseData>> {
        let request_body = Q::build_query(variables);
        
        let response: Response<Q::ResponseData> = self.client
            .post("https://api.github.com/graphql")
            .bearer_auth(&self.token)
            .json(&request_body)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }  
}