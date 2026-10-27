use std::fmt;

use crate::error::AppError::{
    CouldNotGetOldestIssue, CouldNotMakeDateForIssueSearch, IdConversion,
};

#[derive(Debug)]
pub enum AppError {
    Octocrab(octocrab::Error),
    DatabaseQueryError(sqlx::Error),
    IdConversion,
    CouldNotGetOldestIssue,
    CouldNotMakeDateForIssueSearch,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Octocrab(e) => write!(f, "{e}"),
            AppError::DatabaseQueryError(e) => write!(f, "{e}"),
            IdConversion => write!(f, "ID conversion"),
            CouldNotGetOldestIssue => write!(f, "Could not get oldest issue"),
            CouldNotMakeDateForIssueSearch => write!(f, "Could not make date for issue search"),
        }
    }
}
