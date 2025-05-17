use anyhow::Result;

mod graphql_queries {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/graphql/mod.rs"));
}
mod queries;

use queries::{
    close_issues::close_issue,
    create_issue::create_issue,
    get_issues::get_issues,
    get_members::get_members,
    get_team::get_team
};

#[tokio::main]
async fn main() -> Result<()> {
    let (repo_id, issues) = get_issues().await?;

    close_issue(issues)
        .await
        .unwrap_or_else(|x| {
            eprintln!("{}", x)
    });

    let assignees = match get_team().await {
        Ok(id) => {
            get_members(id)
                .await
                .unwrap_or_else(|x| {
                    eprintln!("{}", x);
                    Vec::new()
                })
        },
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
