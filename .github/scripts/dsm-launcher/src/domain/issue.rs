use std::ops::Deref;

use super::member::Member;

pub struct Issue {
    pub id: Option<String>, // IssueId,
    pub repo_id: String,
    pub title: String,
    pub team_slug: String,
    pub body: String,
    pub assignees: Vec<Member>,
}

pub struct IssueId(String);

impl IssueId {
    pub fn new(id: String) -> Self {
        IssueId(id)
    }
}

impl Deref for IssueId {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}