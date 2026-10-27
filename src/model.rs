use std::fmt;

use chrono::{DateTime, Utc};
use octocrab::models::{IssueState, issues::Issue};

#[derive(sqlx::Type, Debug, Clone, Copy)]
#[sqlx(type_name = "valid_issue_states")]
pub enum State {
    #[sqlx(rename = "Open")]
    Open,
    #[sqlx(rename = "Closed")]
    Closed,
    #[sqlx(rename = "Unknown")]
    Unknown,
}

impl From<IssueState> for State {
    fn from(value: IssueState) -> Self {
        match value {
            IssueState::Open => Self::Open,
            IssueState::Closed => Self::Closed,
            _ => Self::Unknown,
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Open => write!(f, "Open"),
            State::Closed => write!(f, "Closed"),
            State::Unknown => write!(f, "Unknown"),
        }
    }
}

pub struct GitHubIssue {
    pub id: u64,
    pub url: String,
    pub repository_url: String,
    pub state: State,
    pub title: String,
    pub body: Option<String>,
    pub comment_count: u32,
    pub has_pull_request: bool,
    pub closed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Issue> for GitHubIssue {
    fn from(issue: Issue) -> Self {
        Self {
            id: *issue.id,
            url: issue.url.to_string(),
            repository_url: issue.repository_url.to_string(),
            state: issue.state.into(),
            title: issue.title,
            body: issue.body,
            comment_count: issue.comments,
            has_pull_request: issue.pull_request.is_some(),
            closed_at: issue.closed_at,
            created_at: issue.created_at,
            updated_at: issue.updated_at,
        }
    }
}

impl fmt::Display for GitHubIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Id:             {}", self.id)?;
        writeln!(f, "URL:            {}", self.url.as_str())?;
        writeln!(f, "Repo URL:       {}", self.repository_url.as_str())?;
        writeln!(f, "State:          {}", self.state)?;
        writeln!(f, "Title:          {}", self.title)?;
        writeln!(f, "Body:           {}", self.body.as_deref().unwrap_or(""))?;
        writeln!(f, "Comment Count:  {}", self.comment_count)?;
        writeln!(f, "PR:             {}", self.has_pull_request)?;
        writeln!(f, "Created:        {}", self.created_at.to_rfc3339())?;
        writeln!(f, "Updated:        {}", self.updated_at.to_rfc3339())?;
        writeln!(
            f,
            "Closed:         {}",
            self.closed_at.map(|c| c.to_rfc3339()).unwrap_or_default()
        )?;

        Ok(())
    }
}
