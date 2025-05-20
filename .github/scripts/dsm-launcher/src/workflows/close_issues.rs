use anyhow::{Ok as AOk, Result};

use crate::queries::{
    get_issues::get_issues,
    get_repo::get_repo,
};
use crate::commands::{
    close_issues::close_issue,
};

pub async fn close_issues() -> Result<()> {
    let repo_id = get_repo().await?;

    async move {
        let issues = get_issues(repo_id).await?;
        close_issue(issues).await?;
        AOk(())
    }.await
    .unwrap_or_else(|x| {
        eprintln!("{}", x);
    });

    Ok(())
}