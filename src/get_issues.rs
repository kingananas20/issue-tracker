use anyhow::{Context, Result};
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PullRequest {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Issue {
    pub number: usize,
    pub title: String,
    pub pull_request: Option<PullRequest>,
}

pub async fn get_issues(owner: &str, repo: &str) -> Result<Vec<Issue>> {
    let token = std::env::var("GITHUB_PAT").context("expected GITHUB_PAT in .env file")?;
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/issues?state=open&page=1&per_page=100",
        owner = owner,
        repo = repo,
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header(AUTHORIZATION, format!("Bearer {token}", token = token))
        .header(USER_AGENT, "rust-issue-tracker")
        .header(ACCEPT, "application/vnd.github+json")
        .send()
        .await;

    let response = match response {
        Ok(res) if res.status().is_success() => res,
        _ => return Ok(Vec::new()),
    };

    let issues = response
        .json::<Vec<Issue>>()
        .await
        .context("something went wrong while parsing")?
        .into_iter()
        .filter(|issue| issue.pull_request.is_none())
        .collect::<Vec<_>>();

    Ok(issues)
}
