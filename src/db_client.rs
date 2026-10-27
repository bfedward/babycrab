use sqlx::PgPool;

use crate::{
    error::AppError,
    model::{GitHubIssue, State},
};

pub async fn insert_issues_to_database(
    pool: &PgPool,
    issues: &[GitHubIssue],
) -> Result<(), AppError> {
    if issues.is_empty() {
        return Ok(());
    }

    let ids: Vec<i64> = issues
        .iter()
        .map(|issue| i64::try_from(issue.id))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| AppError::IdConversion)?;

    let urls: Vec<&str> = issues.iter().map(|issue| issue.url.as_str()).collect();

    let repository_urls: Vec<&str> = issues
        .iter()
        .map(|issue| issue.repository_url.as_str())
        .collect();

    let states: Vec<State> = issues.iter().map(|issue| issue.state).collect();

    let titles: Vec<&str> = issues.iter().map(|issue| issue.title.as_str()).collect();

    let bodies: Vec<Option<&str>> = issues.iter().map(|issue| issue.body.as_deref()).collect();

    let comment_counts: Vec<i32> = issues
        .iter()
        .map(|issue| issue.comment_count as i32)
        .collect();

    let has_pull_requests: Vec<bool> = issues.iter().map(|issue| issue.has_pull_request).collect();

    let closed_ats: Vec<Option<chrono::DateTime<chrono::Utc>>> =
        issues.iter().map(|issue| issue.closed_at).collect();

    let created_ats: Vec<chrono::DateTime<chrono::Utc>> =
        issues.iter().map(|issue| issue.created_at).collect();

    let updated_ats: Vec<chrono::DateTime<chrono::Utc>> =
        issues.iter().map(|issue| issue.updated_at).collect();

    sqlx::query(
        r#"
        INSERT INTO issues (
            id,
            url,
            repository_url,
            state,
            title,
            body,
            comment_count,
            has_pull_request,
            closed_at,
            created_at,
            updated_at
        )
        SELECT *
        FROM UNNEST(
            $1::BIGINT[],
            $2::TEXT[],
            $3::TEXT[],
            $4::valid_issue_states[],
            $5::TEXT[],
            $6::TEXT[],
            $7::INTEGER[],
            $8::BOOLEAN[],
            $9::TIMESTAMPTZ[],
            $10::TIMESTAMPTZ[],
            $11::TIMESTAMPTZ[]
        )
        ON CONFLICT (id) DO UPDATE SET
            url = EXCLUDED.url,
            repository_url = EXCLUDED.repository_url,
            state = EXCLUDED.state,
            title = EXCLUDED.title,
            body = EXCLUDED.body,
            comment_count = EXCLUDED.comment_count,
            has_pull_request = EXCLUDED.has_pull_request,
            closed_at = EXCLUDED.closed_at,
            created_at = EXCLUDED.created_at,
            updated_at = EXCLUDED.updated_at
        WHERE EXCLUDED.updated_at > issues.updated_at
        "#,
    )
    .bind(&ids)
    .bind(&urls)
    .bind(&repository_urls)
    .bind(&states)
    .bind(&titles)
    .bind(&bodies)
    .bind(&comment_counts)
    .bind(&has_pull_requests)
    .bind(&closed_ats)
    .bind(&created_ats)
    .bind(&updated_ats)
    .execute(pool)
    .await
    .map_err(AppError::DatabaseQueryError)?;

    Ok(())
}
