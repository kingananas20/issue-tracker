use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub login: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueReaction {
    pub content: String,
    pub user: User,
}

pub async fn get_issue_reaction(owner: &str, repo: &str, number: usize) -> Vec<IssueReaction> {
    let token = std::env::var("GITHUB_PAT").expect("expected GITHUB_PAT in .env file");
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/issues/{issue_number}/reactions",
        owner = owner,
        repo = repo,
        issue_number = number,
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header(AUTHORIZATION, format!("Bearer {token}", token = token))
        .header(USER_AGENT, "rust-issue-tracker")
        .header(ACCEPT, "application/vnd.github+json")
        .send()
        .await
        .expect("error while fetching");

    if !response.status().is_success() {
        return Vec::new();
    }

    let reactions = response
        .json::<Vec<IssueReaction>>()
        .await
        .expect("error while parsing");

    reactions
}
