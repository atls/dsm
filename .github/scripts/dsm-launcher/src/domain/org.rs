
pub struct Organization {
    id: OrgId,
}

impl Organization {
    pub fn new(id: OrgId) -> Self {
        Organization { id }
    }
}

#[derive(Debug, Clone)]
pub struct OrgId(String);


impl OrgId {
    pub fn new(id: String) -> Self {
        OrgId(id)
    }
    pub fn as_str(&self) -> &str {
        &self.0 
    }
}