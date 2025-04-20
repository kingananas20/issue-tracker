pub struct GitHubAPI {
    client: reqwest::Client,
    token: String,
    user_agent: String,
    owner: String,
    repository: String,
}

impl GitHubAPI {
    pub fn new(token: String, user_agent: String, owner: String, repository: String) -> Self {
        let client = reqwest::Client::new();

        GitHubAPI {
            client,
            token,
            user_agent,
            owner,
            repository,
        }
    }

    pub async fn get_issues(&self, url: Option<String>) -> Vec<crate::get_issues::Issue> {
        crate::get_issues::get_issues(
            &self.client,
            &self.token,
            &self.user_agent,
            &self.owner,
            &self.repository,
            url,
        )
        .await
    }

    pub async fn get_issue_reactions(
        &self,
        number: usize,
    ) -> Vec<crate::get_issue_reactions::IssueReaction> {
        crate::get_issue_reactions::get_issue_reactions(
            &self.client,
            &self.token,
            &self.user_agent,
            &self.owner,
            &self.repository,
            number,
        )
        .await
    }
}
