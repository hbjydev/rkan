use futures_util::StreamExt;
use sha2::Digest;
use tracing::instrument;
use std::io::Write;

const RKAN_TEMP_DIR_PREFIX: &str = "rkan_asset_tmp_";

pub struct DownloadedAsset {
    pub size: u64,
    pub hash_sha256: String,
    pub hash_sha1: String,
    pub temp_file: tempfile::NamedTempFile,
}

#[instrument(
    skip(asset, dir),
    fields(
        asset_name = %asset.name,
        asset_url = %asset.browser_download_url,
        workdir = ?dir
    ),
    err(Debug)
)]
pub async fn download_and_hash(
    asset: octorust::types::ReleaseAsset,
    dir: Option<&std::path::Path>,
) -> Result<DownloadedAsset, Box<dyn std::error::Error>> {
    tracing::debug!("Starting download of asset");
    let response = reqwest::get(&asset.browser_download_url).await?;
    let mut stream = response.bytes_stream();

    let mut temp_file = if let Some(dir) = dir {
        tempfile::Builder::new()
            .prefix(RKAN_TEMP_DIR_PREFIX)
            .tempfile_in(dir)?
    } else {
        tempfile::Builder::new()
            .prefix(RKAN_TEMP_DIR_PREFIX)
            .tempfile()?
    };

    tracing::debug!(path = ?temp_file.path(), "Temporary file created for asset download");

    let mut hasher_sha256 = sha2::Sha256::new();
    let mut hasher_sha1 = sha1::Sha1::new();
    let mut size: u64 = 0;

    while let Some(chunk) = stream.next().await {
        tracing::debug!("Received chunk of size {}", chunk.as_ref().map(|c| c.len()).unwrap_or(0));
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