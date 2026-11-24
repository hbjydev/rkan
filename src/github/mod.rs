use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

use futures_util::StreamExt;
use octocrab::{
    Octocrab,
    models::{
        Repository,
        repos::{Asset, Release},
    },
};
use sha2::Digest;
use tracing::instrument;

mod cache;

const GITHUB_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
const RKAN_TEMP_FILE_PREFIX: &str = "rkan_asset_tmp_";

pub struct DownloadedAsset {
    pub size: u64,
    pub hash_sha256: String,
    pub hash_sha1: String,
    archive: AssetArchive,
}

impl DownloadedAsset {
    pub fn path(&self) -> &Path {
        match &self.archive {
            AssetArchive::Cached(path) => path,
            AssetArchive::Temporary(file) => file.path(),
        }
    }
}

enum AssetArchive {
    Cached(PathBuf),
    Temporary(tempfile::NamedTempFile),
}

#[derive(Default)]
struct AssetHasher {
    sha256: sha2::Sha256,
    sha1: sha1::Sha1,
    size: u64,
}

impl AssetHasher {
    fn update(&mut self, bytes: &[u8]) {
        self.sha256.update(bytes);
        self.sha1.update(bytes);
        self.size += bytes.len() as u64;
    }

    fn finish(self, archive: AssetArchive) -> DownloadedAsset {
        DownloadedAsset {
            size: self.size,
            hash_sha256: hex::encode_upper(self.sha256.finalize()),
            hash_sha1: hex::encode_upper(self.sha1.finalize()),
            archive,
        }
    }
}

#[derive(Clone)]
pub struct GithubClient {
    api: Octocrab,
    http: reqwest::Client,
    cache_root: Option<PathBuf>,
}

impl GithubClient {
    pub fn new(token: Option<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let reqwest_client = reqwest::Client::builder()
            .user_agent(GITHUB_USER_AGENT)
            .build()?;

        let mut client = Octocrab::builder();
        if let Some(token) = token {
            client = client.personal_token(token);
        }

        Ok(Self {
            api: client.build()?,
            http: reqwest_client,
            cache_root: cache::root(),
        })
    }

    // #[instrument(skip(self), err(Debug))]
    pub async fn get_repo_info(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Repository, Box<dyn std::error::Error>> {
        Ok(self.api.repos(owner, repo).get().await?)
    }

    // #[instrument(skip(self), err(Debug))]
    pub async fn get_latest_release(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Release, Box<dyn std::error::Error>> {
        Ok(self.api.repos(owner, repo).releases().get_latest().await?)
    }

    // #[instrument(skip(self), err(Debug))]
    pub async fn get_release_by_tag(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
    ) -> Result<Release, Box<dyn std::error::Error>> {
        Ok(self
            .api
            .repos(owner, repo)
            .releases()
            .get_by_tag(tag)
            .await?)
    }

    #[instrument(
        skip(self, asset),
        fields(
            asset_name = %asset.name,
            asset_url = %asset.browser_download_url
        ),
        err(Debug)
    )]
    pub async fn download_and_hash(
        &self,
        asset: &Asset,
    ) -> Result<DownloadedAsset, Box<dyn std::error::Error>> {
        let expected_sha256 = asset.digest.as_deref().and_then(cache::asset_sha256);
        let cache_path = self
            .cache_root
            .as_ref()
            .zip(expected_sha256.as_ref())
            .map(|(root, sha256)| root.join("artifacts").join(format!("{sha256}.zip")));

        if let Some((path, sha256)) = cache_path.clone().zip(expected_sha256.clone()) {
            match tokio::task::spawn_blocking(move || read_cached_asset(path, &sha256)).await? {
                Ok(cached) => {
                    tracing::debug!(path = ?cached.path(), "Using cached release asset");
                    return Ok(cached);
                }
                Err(error) if error.kind() == io::ErrorKind::NotFound => {}
                Err(error) => {
                    tracing::warn!(%error, path = ?cache_path, "Could not reuse cached asset")
                }
            }
        }

        tracing::debug!("Starting download of asset");
        let response = self
            .http
            .get(asset.browser_download_url.clone())
            .send()
            .await?
            .error_for_status()?;
        let mut stream = response.bytes_stream();

        // Stage the download beside the cache entry so it can be published atomically.
        let mut builder = tempfile::Builder::new();
        builder.prefix(RKAN_TEMP_FILE_PREFIX);
        let (mut temp_file, cache_path) = match cache_path {
            Some(path) => {
                let dir = path.parent().expect("artifact cache path has a parent");
                match fs::create_dir_all(dir).and_then(|()| builder.tempfile_in(dir)) {
                    Ok(file) => (file, Some(path)),
                    Err(error) => {
                        tracing::warn!(%error, ?dir, "Could not stage asset in cache");
                        (builder.tempfile()?, None)
                    }
                }
            }
            None => (builder.tempfile()?, None),
        };

        let mut hasher = AssetHasher::default();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            hasher.update(&chunk);
            temp_file.write_all(&chunk)?;
        }

        let mut downloaded = hasher.finish(AssetArchive::Temporary(temp_file));
        if let Some(sha256) = expected_sha256 {
            verify_sha256(&downloaded, &sha256)?;
        }
        downloaded.archive = match (cache_path, downloaded.archive) {
            (Some(path), AssetArchive::Temporary(file)) => match file.persist(&path) {
                Ok(_) => AssetArchive::Cached(path),
                Err(error) => {
                    tracing::warn!(error = %error.error, ?path, "Could not publish cached asset");
                    AssetArchive::Temporary(error.file)
                }
            },
            (_, archive) => archive,
        };

        tracing::info!(
            size = downloaded.size,
            hash_sha256 = %downloaded.hash_sha256,
            hash_sha1 = %downloaded.hash_sha1,
            "Asset download and hashing complete",
        );

        Ok(downloaded)
    }
}

fn read_cached_asset(path: PathBuf, sha256: &str) -> io::Result<DownloadedAsset> {
    let mut file = BufReader::new(File::open(&path)?);
    let mut hasher = AssetHasher::default();
    loop {
        let bytes = file.fill_buf()?;
        if bytes.is_empty() {
            break;
        }
        hasher.update(bytes);
        let len = bytes.len();
        file.consume(len);
    }
    let cached = hasher.finish(AssetArchive::Cached(path));
    verify_sha256(&cached, sha256)?;
    Ok(cached)
}

fn verify_sha256(asset: &DownloadedAsset, expected: &str) -> io::Result<()> {
    if asset.hash_sha256 != expected {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "Release asset SHA-256 mismatch (expected {expected}, got {})",
                asset.hash_sha256
            ),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHA256: &str = "BA7816BF8F01CFEA414140DE5DAE2223B00361A396177A9CB410FF61F20015AD";

    #[tokio::test]
    async fn reuses_cached_asset_without_requesting_its_url() {
        let cache = tempfile::tempdir().unwrap();
        let mut client = GithubClient::new(None).unwrap();
        client.cache_root = Some(cache.path().to_path_buf());
        let artifacts = cache.path().join("artifacts");
        fs::create_dir(&artifacts).unwrap();
        let path = artifacts.join(format!("{SHA256}.zip"));
        fs::write(&path, b"abc").unwrap();

        let asset = serde_json::from_value(serde_json::json!({
            "url": "https://example.com/assets/1",
            "browser_download_url": "file:///unavailable.zip",
            "id": 1,
            "node_id": "asset",
            "name": "mod.zip",
            "state": "uploaded",
            "content_type": "application/zip",
            "size": 3,
            "digest": format!("sha256:{}", SHA256.to_ascii_lowercase()),
            "download_count": 0,
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-01T00:00:00Z"
        }))
        .unwrap();

        let downloaded = client.download_and_hash(&asset).await.unwrap();
        assert_eq!(downloaded.path(), path);
        assert_eq!(downloaded.size, 3);
        assert_eq!(downloaded.hash_sha256, SHA256);
        assert_eq!(
            downloaded.hash_sha1,
            "A9993E364706816ABA3E25717850C26C9CD0D89D"
        );
        drop(downloaded);
        assert!(path.is_file());
    }

    #[test]
    fn rejects_corrupted_cached_bytes() {
        let file = tempfile::NamedTempFile::new().unwrap();
        fs::write(file.path(), b"corrupted").unwrap();
        let result = read_cached_asset(file.path().to_path_buf(), SHA256);
        assert!(matches!(result, Err(err) if err.kind() == io::ErrorKind::InvalidData));
    }
}
