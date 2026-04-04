use std::io::Write;

use futures_util::StreamExt;
use octorust::auth::Credentials;
use sha2::Digest;
use tracing::instrument;

const GITHUB_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
const RKAN_TEMP_FILE_PREFIX: &str = "rkan_asset_tmp_";

pub struct DownloadedAsset {
    pub size: u64,
    pub hash_sha256: String,
    pub hash_sha1: String,
    pub temp_file: tempfile::NamedTempFile,
}

#[derive(Clone)]
pub struct GithubClient(octorust::Client, reqwest::Client);

impl GithubClient {
    pub fn new(token: Option<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let reqwest_client = reqwest::Client::builder()
            .user_agent(GITHUB_USER_AGENT)
            .build()?;

        let creds = token.map(Credentials::Token);
        let client = octorust::Client::new(GITHUB_USER_AGENT, creds)?;

        Ok(Self(client, reqwest_client))
    }

    // #[instrument(skip(self), err(Debug))]
    pub async fn get_repo_info(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<octorust::types::FullRepository, Box<dyn std::error::Error>> {
        Ok(self.0.repos().get(owner, repo).await?.body)
    }

    // #[instrument(skip(self), err(Debug))]
    pub async fn get_latest_release(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<octorust::types::Release, Box<dyn std::error::Error>> {
        Ok(self.0.repos().get_latest_release(owner, repo).await?.body)
    }

    // #[instrument(skip(self), err(Debug))]
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

    #[instrument(
        skip(self, asset, dir),
        fields(
            asset_name = %asset.name,
            asset_url = %asset.browser_download_url,
            workdir = ?dir
        ),
        err(Debug)
    )]
    pub async fn download_and_hash(
        &self,
        asset: octorust::types::ReleaseAsset,
        dir: Option<&std::path::Path>,
    ) -> Result<DownloadedAsset, Box<dyn std::error::Error>> {
        tracing::debug!("Starting download of asset");
        let response = self.1.get(&asset.browser_download_url).send().await?;
        let mut stream = response.bytes_stream();

        let mut temp_file = if let Some(dir) = dir {
            tempfile::Builder::new()
                .prefix(RKAN_TEMP_FILE_PREFIX)
                .tempfile_in(dir)?
        } else {
            tempfile::Builder::new()
                .prefix(RKAN_TEMP_FILE_PREFIX)
                .tempfile()?
        };

        tracing::debug!(path = ?temp_file.path(), "Temporary file created for asset download");

        let mut hasher_sha256 = sha2::Sha256::new();
        let mut hasher_sha1 = sha1::Sha1::new();
        let mut size: u64 = 0;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            hasher_sha256.update(&chunk);
            hasher_sha1.update(&chunk);
            size += chunk.len() as u64;
            temp_file.write_all(&chunk)?;
        }

        let hash_sha256 = hex::encode(hasher_sha256.finalize()).to_uppercase();
        let hash_sha1 = hex::encode(hasher_sha1.finalize()).to_uppercase();

        tracing::info!(
            size = size,
            hash_sha256 = %hash_sha256,
            hash_sha1 = %hash_sha1,
            "Asset download and hashing complete",
        );

        Ok(DownloadedAsset {
            size,
            hash_sha256,
            hash_sha1,
            temp_file,
        })
    }
}
