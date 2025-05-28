#![allow(clippy::all, warnings)]
pub struct GetOrg;
pub mod get_org {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "GetOrg";
    pub const QUERY: &str =
        "query GetOrg($org: String!) {\n    organization(login: $org) {\n        id\n    }\n}";
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
        pub org: String,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub organization: Option<GetOrgOrganization>,
    }
    #[derive(Deserialize)]
    pub struct GetOrgOrganization {
        pub id: ID,
    }
}
impl graphql_client::GraphQLQuery for GetOrg {
    type Variables = get_org::Variables;
    type ResponseData = get_org::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_org::QUERY,
            operation_name: get_org::OPERATION_NAME,
        }
    }
}
