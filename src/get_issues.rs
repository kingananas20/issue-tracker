use futures::future::BoxFuture;
use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderMap, USER_AGENT};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PullRequest {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Issue {
    pub number: usize,
    pub title: String,
    pub pull_request: Option<PullRequest>,
}

pub fn get_issues_wrapper<'a>(
    owner: &'a str,
    repo: &'a str,
    url: Option<String>,
) -> BoxFuture<'a, Vec<Issue>> {
    Box::pin(get_issues(owner, repo, url))
}

pub async fn get_issues(owner: &str, repo: &str, url: Option<String>) -> Vec<Issue> {
    let token = std::env::var("GITHUB_PAT").expect("expected GITHUB_PAT in .env file");
    let request_url = url.unwrap_or(format!(
        "https://api.github.com/repos/{owner}/{repo}/issues?state=open&page=1&per_page=100",
        owner = owner,
        repo = repo,
    ));

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

    let new_url = construct_new_url(&response.headers());

    let issues = response
        .json::<Vec<Issue>>()
        .await
        .expect("something went wrong while parsing")
        .into_iter()
        .filter(|issue| issue.pull_request.is_none())
        .collect::<Vec<Issue>>();

    if let Some(new_url) = new_url {
        let more_issues = get_issues_wrapper(owner, repo, Some(new_url)).await;
        return issues.into_iter().chain(more_issues).collect();
    }

    issues
}

fn construct_new_url(headers: &HeaderMap) -> Option<String> {
    let link_header = headers.get("link")?.to_str().ok()?;

    for part in link_header.split(",").collect::<Vec<&str>>() {
        if part.contains("rel=\"next\"") {
            return Some(
                part.trim_start_matches("<")
                    .trim_end_matches(">; rel=\"next\"")
                    .to_string(),
            );
        }
    }

    None
}
