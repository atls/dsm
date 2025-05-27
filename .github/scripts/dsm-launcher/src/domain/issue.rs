use std::ops::Deref;

use super::member::Member;

pub struct Issue {
    pub id: Option<String>,
    pub repo_id: String,
    pub title: String,
    pub issue_type_id: String,
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

#[derive(Debug)]
pub struct IssueType {
    pub id: String,
    pub name: String,
}

impl IssueType {
    pub fn new(id: String, name: String) -> Self {
        IssueType { id, name }
    }
}

pub struct IssueTypeId(String);

impl Deref for IssueTypeId {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
