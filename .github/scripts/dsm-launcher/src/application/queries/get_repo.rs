use anyhow::{anyhow, Ok, Result};
use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;
use std::env;

use crate::{domain::{org::OrgId, repo::RepoId}, graphql_queries::get_repo::{
    get_repo::{self, Variables as GetRepoVars},
    GetRepo,
}};

use crate::domain::repository::OrgRepository;

pub struct GetRepoQuery<R: OrgRepository> {
    repo: R,
}

impl<R: OrgRepository> GetRepoQuery<R> {
    pub async fn execute(&self, org_id: &OrgId, repo_name:  &str) -> Result<RepoId> {
        self.repo.get_repo(org_id, repo_name).await
    }
}
