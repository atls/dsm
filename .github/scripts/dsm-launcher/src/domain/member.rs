pub struct Member {
    pub id: MemberId,
    pub login: String,
}

pub struct MemberId(String);

impl MemberId {
    pub fn new(id: String) -> Self {
        MemberId(id)
    }
    pub fn as_str(&self) -> &str {
        &self.0 
    }
}