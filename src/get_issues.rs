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

fn get_issues_wrapper<'a>(
    client: &'a reqwest::Client,
    token: &'a str,
    user_agent: &'a str,
    owner: &'a str,
    repository: &'a str,
    url: Option<String>,
) -> BoxFuture<'a, Vec<Issue>> {
    Box::pin(get_issues(
        client, token, user_agent, owner, repository, url,
    ))
}

pub(crate) async fn get_issues(
    client: &reqwest::Client,
    token: &str,
    user_agent: &str,
    owner: &str,
    repository: &str,
    url: Option<String>,
) -> Vec<Issue> {
    let request_url = url.unwrap_or(format!(
        "https://api.github.com/repos/{owner}/{repo}/issues?state=open&page=1&per_page=100",
        owner = owner,
        repo = repository,
    ));

    let response = client
        .get(&request_url)
        .header(AUTHORIZATION, format!("Bearer {token}", token = token))
        .header(USER_AGENT, format!("{}", user_agent))
        .header(ACCEPT, "application/vnd.github+json")
        .send()
        .await
        .expect("error while fetching");

    if !response.status().is_success() {
        println!("Responses status: {}", response.status().as_u16());
        println!("Headers: {:?}", response.headers());
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
        let more_issues =
            get_issues_wrapper(client, token, user_agent, owner, repository, Some(new_url)).await;
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
