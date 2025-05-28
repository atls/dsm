#![allow(clippy::all, warnings)]
pub struct CloseIssue;
pub mod close_issue {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CloseIssue";
    pub const QUERY : & str = "mutation CloseIssue($id: ID!) {\n  closeIssue(input: { issueId: $id }) {\n    issue {\n      number\n      state\n    }\n  }\n}" ;
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
    #[derive()]
    pub enum IssueState {
        OPEN,
        CLOSED,
        Other(String),
    }
    impl ::serde::Serialize for IssueState {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                IssueState::OPEN => "OPEN",
                IssueState::CLOSED => "CLOSED",
                IssueState::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for IssueState {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = ::serde::Deserialize::deserialize(deserializer)?;
            match s.as_str() {
                "OPEN" => Ok(IssueState::OPEN),
                "CLOSED" => Ok(IssueState::CLOSED),
                _ => Ok(IssueState::Other(s)),
            }
        }
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub id: ID,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "closeIssue")]
        pub close_issue: Option<CloseIssueCloseIssue>,
    }
    #[derive(Deserialize)]
    pub struct CloseIssueCloseIssue {
        pub issue: Option<CloseIssueCloseIssueIssue>,
    }
    #[derive(Deserialize)]
    pub struct CloseIssueCloseIssueIssue {
        pub number: Int,
        pub state: IssueState,
    }
}
impl graphql_client::GraphQLQuery for CloseIssue {
    type Variables = close_issue::Variables;
    type ResponseData = close_issue::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: close_issue::QUERY,
            operation_name: close_issue::OPERATION_NAME,
        }
    }
}
