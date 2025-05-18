use anyhow::{Result, Ok as AOk};

mod graphql_queries {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/graphql/mod.rs"));
}
mod queries;

use queries::{
    close_issues::close_issue,
    create_issue::create_issue,
    get_issues::get_issues,
    get_members::get_members,
    get_team::get_team,
    get_org::get_org,
    get_repo::get_repo,
};

#[tokio::main]
async fn main() -> Result<()> {
    let repo_id = get_repo().await?;

    let repo_id_clone = repo_id.clone();
    async move {
        let issues = get_issues(repo_id_clone).await?;
        close_issue(issues).await?;
        AOk(())
    }.await
    .unwrap_or_else(|x| {
        eprintln!("{}", x);
    });

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

    create_issue(
        repo_id,
        assignees,
    ).await?;

    Ok(())
}
