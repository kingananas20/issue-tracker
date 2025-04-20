mod get_issue_reactions;
mod get_issues;
mod github_api;

use std::collections::HashMap;

use clap::Parser;
use colored::{Color, Colorize};
use dotenv::dotenv;
use futures::{StreamExt, stream::FuturesUnordered};
use github_api::GitHubAPI;

/// program to get all the issues on a github repository
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Owner of the repository
    owner: String,

    /// Repository name
    repository: String,

    /// amount of issues you want to show
    limit: Option<usize>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = Cli::parse();

    let token = std::env::var("GITHUB_PAT").expect("expected GITHUB_PAT in .env file");
    let user_agent = std::env::var("USER_AGENT").expect("expected USER_AGENT in .env file");
    let owner = cli.owner;
    let repository = cli.repository;

    let limit = cli.limit.unwrap_or(0);
    println!("{}", limit);

    let github_api = GitHubAPI::new(token, user_agent, owner, repository);

    let issues = github_api.get_issues(None).await;
    let mut futures = FuturesUnordered::new();

    for issue in &issues {
        let issue_number = issue.number;
        let issue_title = &issue.title;
        let github_api_ref = &github_api;

        futures.push({
            async move {
                let reactions = github_api_ref.get_issue_reactions(issue_number).await;
                (issue_number, issue_title, reactions)
            }
        });
    }

    let mut results: HashMap<usize, (&str, usize)> = HashMap::new();
    while let Some((issue_number, issue_title, reactions)) = futures.next().await {
        let reaction_count = reactions.iter().filter(|r| r.content == "+1").count();
        if reaction_count > 0 {
            results
                .entry(issue_number)
                .and_modify(|e| e.1 += reaction_count)
                .or_insert((issue_title, reaction_count));
        }
    }

    let mut sorted_results = results.into_iter().collect::<Vec<(usize, (&str, usize))>>();
    sorted_results.sort_by(|a, b| b.1.1.cmp(&a.1.1));

    println!("Amount of issues: {:?}", issues.len());
    for (index, (issue_number, (issue_title, reaction_count))) in sorted_results.iter().enumerate()
    {
        if limit != 0 && index >= limit {
            break;
        }
        println!(
            "{} {} {} {}",
            format!("{:03}.", index + 1).color(Color::TrueColor {
                r: 128,
                g: 128,
                b: 128
            }),
            format!("{}", issue_title),
            format!("#{}", issue_number).color(Color::TrueColor {
                r: 32,
                g: 32,
                b: 32
            }),
            format!("({} 👍)", reaction_count.to_string().green())
        );
    }
}
