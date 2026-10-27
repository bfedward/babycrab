use chrono::{DateTime, Utc};
use octocrab::Octocrab;

use crate::{error::AppError, model::GitHubIssue};

pub async fn get_created_at_of_oldest_issue(
    octocrab: &Octocrab,
) -> Result<Option<DateTime<Utc>>, AppError> {
    let pages = octocrab
        .search()
        .issues_and_pull_requests("is:issue language:rust is:public label:\"good first issue\"")
        .sort("created")
        .order("asc")
        .per_page(1)
        .page(1u32)
        .send()
        .await
        .map_err(AppError::Octocrab)?;

    Ok(pages.items.first().map(|i| i.created_at))
}

pub async fn get_github_issues(
    octocrab: &Octocrab,
    query: &str,
    page: u32,
) -> Result<Vec<GitHubIssue>, AppError> {
    let pages = octocrab
        .search()
        .issues_and_pull_requests(&query)
        .sort("created")
        .order("asc")
        .per_page(100)
        .page(page)
        .send()
        .await
        .map_err(AppError::Octocrab)?;

    let issues: Vec<GitHubIssue> = pages.items.into_iter().map(|g| g.into()).collect();

    Ok(issues)
}
