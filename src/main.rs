mod get_issue_reaction;
mod get_issues;

use colored::{Color, Colorize};
use dotenv::dotenv;
use get_issue_reaction::get_issue_reaction;
use get_issues::get_issues;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let owner = "FlorianWoelki";
    let repo = "obsidian-iconize";

    println!("{:?}", get_issue_reaction(owner, repo, 128).await);

    let issues = get_issues(owner, repo, None).await;
    println!("Amount of issues: {:?}", issues.len());

    for issue in &issues {
        let reactions = get_issue_reaction(owner, repo, issue.number).await;
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
