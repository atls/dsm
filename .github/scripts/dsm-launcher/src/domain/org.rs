use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct OrgId(String);


impl OrgId {
    pub fn new(id: String) -> Self {
        OrgId(id)
    }
}

impl Deref for OrgId {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}