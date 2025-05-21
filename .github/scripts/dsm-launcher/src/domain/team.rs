pub struct Team {
    id: TeamId,
    team_slug: String,
}

impl Team {
    pub fn new(id: TeamId, team_slug: String) -> Self {
        Team { id, team_slug }
    }
}

pub struct TeamId(String);

impl TeamId {
    pub fn new(id: String) -> Self {
        TeamId(id)
    }
    pub fn as_str(&self) -> &str {
        &self.0 
    }
}