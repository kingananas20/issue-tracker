mod get_issue_reaction;
mod get_issues;

use dotenv::dotenv;
use get_issues::get_issues;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let issues = get_issues(None).await;
    println!("{:?}", issues.len());
}
