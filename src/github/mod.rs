use octorust::{auth::Credentials, Client};

const GITHUB_USER_AGENT: &str = "rkan-ckan";

#[derive(Clone)]
pub struct GithubClient(Client);

impl GithubClient {
    pub fn new(token: String) -> Self {
        let creds =  Credentials::Token(token);
        let client = Client::new(GITHUB_USER_AGENT, creds).unwrap();
        Self(client)
    }

    pub async fn get_repo_info(&self, owner: &str, repo: &str) -> Result<octorust::types::FullRepository, Box<dyn std::error::Error>> {
        Ok(self.0.repos().get(owner, repo).await?.body)
    }

    pub async fn get_latest_release(&self, owner: &str, repo: &str) -> Result<octorust::types::Release, Box<dyn std::error::Error>> {
        Ok(self.0.repos().get_latest_release(owner, repo).await?.body)
    }

    pub async fn get_release_by_tag(&self, owner: &str, repo: &str, tag: &str) -> Result<octorust::types::Release, Box<dyn std::error::Error>> {
        Ok(self.0.repos().get_release_by_tag(owner, repo, tag).await?.body)
    }
}