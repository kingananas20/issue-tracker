mod get_issue_reactions;
mod get_issues;
mod github_api;

use clap::Parser;
use colored::{Color, Colorize};
use dotenv::dotenv;
use github_api::GitHubAPI;

/// program to get all the issues on a github repository
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Owner of the repository
    owner: String,

    /// Repository name
    repository: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = Cli::parse();

    let token = std::env::var("GITHUB_PAT").expect("expected GITHUB_PAT in .env file");
    let user_agent = std::env::var("USER_AGENT").expect("expected USER_AGENT in .env file");
    let owner = cli.owner;
    let repository = cli.repository;

    let github_api = GitHubAPI::new(token, user_agent, owner, repository);

    let issues = github_api.get_issues(None).await;
    println!("Amount of issues: {:?}", issues.len());

    for issue in &issues {
        let reactions = github_api.get_issue_reactions(issue.number).await;
        let mut thumbs_up_count = 0;

        for reaction in &reactions {
            if reaction.content == "+1" {
                thumbs_up_count += 1;
            }
        }

        println!(
            "{} {} {}",
            format!("#{}", issue.number).color(Color::TrueColor {
                r: 32,
                g: 32,
                b: 32
            }),
            issue.title,
            thumbs_up_count.to_string().green()
        );
    }
}
