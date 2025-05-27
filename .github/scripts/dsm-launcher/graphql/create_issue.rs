#![allow(clippy::all, warnings)]
pub struct CreateIssue;
pub mod create_issue {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CreateIssue";
    pub const QUERY : & str = "mutation CreateIssue($repoId: ID!, $title: String!, $body: String!, $assigneeIds: [ID!]!, $issueTypeId: ID!) {\n  createIssue(input: {\n    repositoryId: $repoId,\n    title: $title,\n    assigneeIds: $assigneeIds,\n    body: $body,\n    issueTypeId: $issueTypeId,\n  }) {\n    issue {\n      id\n    }\n  }\n}" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "repoId")]
        pub repo_id: ID,
        pub title: String,
        pub body: String,
        #[serde(rename = "assigneeIds")]
        pub assignee_ids: Vec<ID>,
        #[serde(rename = "issueTypeId")]
        pub issue_type_id: ID,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "createIssue")]
        pub create_issue: Option<CreateIssueCreateIssue>,
    }
    #[derive(Deserialize)]
    pub struct CreateIssueCreateIssue {
        pub issue: Option<CreateIssueCreateIssueIssue>,
    }
    #[derive(Deserialize)]
    pub struct CreateIssueCreateIssueIssue {
        pub id: ID,
    }
}
impl graphql_client::GraphQLQuery for CreateIssue {
    type Variables = create_issue::Variables;
    type ResponseData = create_issue::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: create_issue::QUERY,
            operation_name: create_issue::OPERATION_NAME,
        }
    }
}
