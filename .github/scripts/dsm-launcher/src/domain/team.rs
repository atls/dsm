use std::ops::Deref;

pub struct TeamId(pub String);

impl TeamId {
    pub fn new(id: String) -> Self {
        TeamId(id)
    }
}

impl Deref for TeamId {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
