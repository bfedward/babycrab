use std::time::Duration;

use chrono::{Datelike, Days, Months, NaiveDate, Utc};
use sqlx::PgPool;

use crate::{db_client, error::AppError, github_client};

pub async fn harvest_issues(github_token: String, pool: &PgPool) -> Result<(), AppError> {
    let octocrab = octocrab::Octocrab::builder()
        .personal_token(github_token)
        .build()
        .map_err(AppError::Octocrab)?;

    let oldest_created_date = github_client::get_created_at_of_oldest_issue(&octocrab)
        .await?
        .ok_or(AppError::CouldNotGetOldestIssue)?;

    println!("Oldest issue created: {oldest_created_date}");

    let mut start =
        NaiveDate::from_ymd_opt(oldest_created_date.year(), oldest_created_date.month(), 1)
            .ok_or(AppError::CouldNotMakeDateForIssueSearch)?;

    let today = Utc::now().date_naive();

    while start <= today {
        let next_month = start
            .checked_add_months(Months::new(1))
            .ok_or(AppError::CouldNotMakeDateForIssueSearch)?;

        // the "created" search parameter has an inclusive end date
        let end = next_month
            .checked_sub_days(Days::new(1))
            .ok_or(AppError::CouldNotMakeDateForIssueSearch)?
            .min(today);

        println!("Harvesting {start} to {end}");

        let query = format!(
            r#"is:issue language:rust is:public label:"good first issue" created:{start}..{end}"#
        );

        for page in 1u32.. {
            // Sleep so we don't hammer GitHub with too many requests
            tokio::time::sleep(Duration::from_millis(1000)).await;

            let result = github_client::get_github_issues(&octocrab, &query, page).await;

            let issues = match result {
                Ok(result) => result,

                Err(AppError::Octocrab(octocrab::Error::GitHub { source, backtrace }))
                    if source.status_code.as_u16() == 403 =>
                {
                    if let Some(reset_at) = source.rate_limit_reset_at() {
                        let wait = (reset_at - Utc::now()).to_std().unwrap_or(Duration::ZERO)
                            + Duration::from_secs(2);

                        println!(
                            "GitHub rate limit reached. Waiting {} seconds until reset.",
                            wait.as_secs()
                        );

                        tokio::time::sleep(wait).await;

                        // Retry this same page.
                        continue;
                    }

                    return Err(AppError::Octocrab(octocrab::Error::GitHub {
                        source,
                        backtrace,
                    }));
                }

                Err(e) => return Err(e),
            };

            println!("Month {start}: page {page}, {} issues", issues.len());

            if issues.is_empty() {
                break;
            }

            db_client::insert_issues_to_database(pool, &issues).await?;

            // Search API returns at most 1,000 results per query.
            if page == 10 {
                println!("WARNING: query {start}..{end} exceeds the 1,000-result pagination limit");
                break;
            }
        }

        start = next_month;
    }

    Ok(())
}
