use super::org::OrgId;

pub struct Repository {
    id: RepoId,
    org_id: OrgId,
    repo_name: String,
}

impl Repository {
    pub fn new(id: RepoId, org_id: OrgId, repo_name: String) -> Self {
        Repository {
            id,
            org_id,
            repo_name
        }
    }
}

#[derive(Debug, Clone)]
pub struct RepoId(String);

impl RepoId {
    pub fn new(id: String) -> Self {
        RepoId(id)
    }
    pub fn as_str(&self) -> &str {
        &self.0 
    }
}