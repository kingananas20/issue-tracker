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

pub fn get_issues_wrapper(url: Option<String>) -> BoxFuture<'static, Vec<Issue>> {
    Box::pin(get_issues(url))
}

pub async fn get_issues(url: Option<String>) -> Vec<Issue> {
    let token = std::env::var("GITHUB_PAT").expect("expected GITHUB_PAT in .env file");
    let request_url = url.unwrap_or(format!(
        "https://api.github.com/repos/{owner}/{repo}/issues?state=open&page=1&per_page=25",
        owner = "zed-industries",
        repo = "zed",
    ));

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
        _ => return Vec::new(),
    };

    let new_url = construct_new_url(&response.headers());

    let issues = response
        .json::<Vec<Issue>>()
        .await
        .expect("something went wrong while parsing")
        .into_iter()
        .filter(|issue| issue.pull_request.is_none())
        .collect::<Vec<Issue>>();

    if let Some(new_url) = new_url {
        let more_issues = get_issues_wrapper(Some(new_url)).await;
        return issues.into_iter().chain(more_issues).collect();
    }

    issues
}

fn construct_new_url(headers: &HeaderMap) -> Option<String> {
    headers.get("link").and_then(|link_header| {
        link_header.to_str().ok().and_then(|link_value| {
            link_value.contains("rel=\"next\"").then(|| {
                link_value
                    .split(';')
                    .collect::<Vec<&str>>()
                    .get(0)
                    .expect("could not find new url with page")
                    .trim_start_matches("<")
                    .trim_end_matches(">")
                    .to_string()
            })
        })
    })
}
