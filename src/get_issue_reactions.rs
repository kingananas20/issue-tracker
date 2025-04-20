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

pub(crate) async fn get_issue_reactions(
    client: &reqwest::Client,
    token: &str,
    user_agent: &str,
    owner: &str,
    repository: &str,
    number: usize,
) -> Vec<IssueReaction> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/issues/{issue_number}/reactions",
        owner = owner,
        repo = repository,
        issue_number = number,
    );

    let response = client
        .get(&request_url)
        .header(AUTHORIZATION, format!("Bearer {token}", token = token))
        .header(USER_AGENT, user_agent)
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
