use anyhow::{Ok, Result};

use crate::application::{
    commands::create_issue::CreateIssueCommand,
    queries::{
        get_repo::GetRepoQuery,
        get_team::GetTeamQuery,
        get_team_members::GetTeamMembersQuery,
        get_org::GetOrgQuery
    },
};
use crate::domain::{
    repository::{
        IssueRepository, 
        MemberRepository, 
        OrgRepository
    },
    issue::Issue,
};

pub async fn create_issue<R: OrgRepository, M: MemberRepository, I: IssueRepository>(
    get_org: GetOrgQuery<R>,
    get_repo: GetRepoQuery<R>,
    get_team: GetTeamQuery<M>,
    get_team_members: GetTeamMembersQuery<M>,
    create_issue: CreateIssueCommand<I>,
    owner: &str,
    repo_name: &str,
    team_slug: &str,
    title: &str,
    body: &str,
) -> Result<()> {
    let org_id = get_org.execute(owner).await?;
    let repo_id = get_repo.execute(&org_id, &repo_name).await?;

    let team_id = get_team.execute(&org_id, &team_slug).await?;
    let members = get_team_members.execute(&team_id).await?;

    let new_issue = Issue {
        id: None,
        repo_id: repo_id.to_string(),
        title: title.to_string(),
        body: body.to_string(),
        team_slug: team_slug.to_string(),
        assignees: members,
    };

    create_issue.execute(new_issue).await?;

    Ok(())
}
