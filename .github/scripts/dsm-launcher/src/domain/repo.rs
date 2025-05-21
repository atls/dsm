use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct RepoId(String);

impl RepoId {
    pub fn new(id: String) -> Self {
        RepoId(id)
    }
}

impl Deref for RepoId {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}