mod get_issues;

use anyhow::{Context, Result};
use dotenv::dotenv;
use get_issues::get_issues;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let issues = get_issues("zed-industries", "zed")
        .await
        .context("error while getting issues")?;
    println!("{:?}", issues);

    Ok(())
}
