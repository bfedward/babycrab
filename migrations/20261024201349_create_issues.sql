CREATE TYPE valid_issue_states AS ENUM ('Open', 'Closed', 'Unknown');

CREATE TABLE issues (
    id BIGINT PRIMARY KEY, 
    url TEXT NOT NULL,
    repository_url TEXT NOT NULL,
    state valid_issue_states NOT NULL,
    title TEXT NOT NULL,
    body TEXT,
    comment_count INTEGER NOT NULL DEFAULT 0,
    has_pull_request BOOLEAN NOT NULL DEFAULT FALSE,
    closed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_issues_repository_url
    ON issues (repository_url);

CREATE INDEX idx_issues_state
    ON issues (state);

CREATE INDEX idx_issues_updated_at
    ON issues (updated_at);