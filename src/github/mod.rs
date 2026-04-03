use octorust::auth::Credentials;

const GITHUB_USER_AGENT: &str = "rkan-ckan";

pub mod download;

#[derive(Clone)]
pub struct GithubClient(octorust::Client, reqwest::Client);

impl GithubClient {
    pub fn new(token: Option<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let reqwest_client = reqwest::Client::new();
        let creds = token.map(|val| Credentials::Token(val));
        let client = octorust::Client::new(GITHUB_USER_AGENT, creds)?;
        Ok(Self(client, reqwest_client))
    }

    pub async fn get_repo_info(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<octorust::types::FullRepository, Box<dyn std::error::Error>> {
        Ok(self.0.repos().get(owner, repo).await?.body)
    }

    pub async fn get_latest_release(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<octorust::types::Release, Box<dyn std::error::Error>> {
        Ok(self.0.repos().get_latest_release(owner, repo).await?.body)
    }

    pub async fn get_release_by_tag(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
    ) -> Result<octorust::types::Release, Box<dyn std::error::Error>> {
        Ok(self
            .0
            .repos()
            .get_release_by_tag(owner, repo, tag)
            .await?
            .body)
    }
}
