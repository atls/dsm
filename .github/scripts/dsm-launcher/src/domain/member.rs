use std::ops::Deref;

pub struct Member {
    pub id: MemberId,
    pub login: String,
}

impl Member {
    pub fn new(id: MemberId, login: String) -> Self {
        Member {
            id,
            login
        }
    }
}

pub struct MemberId(String);

impl MemberId {
    pub fn new(id: String) -> Self {
        MemberId(id)
    }
}

impl Deref for MemberId {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
