use anyhow::Result;

use crate::queries::{
    get_members::get_members,
    get_team::get_team,
    get_org::get_org,
    get_repo::get_repo,
};
use crate::commands::{
    create_issue::create_issue as create_issue_,
};

pub async fn create_issue() -> Result<()> {
    let repo_id = get_repo().await?;

    let assignees = match async move {
        let org_id = get_org().await?;
        let team_id = get_team(org_id).await?;
        get_members(team_id).await
    }.await {
        Ok(x) => x,
        Err(x) => {
            eprintln!("{}", x);
            Vec::new()
        }
    };

    create_issue_(
        repo_id,
        assignees,
    ).await?;

    Ok(())
}