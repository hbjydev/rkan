use std::env;

use futures_util::{StreamExt, stream::FuturesUnordered};
use tracing::instrument;

use crate::{config::Mod, github::{GithubClient, DownloadedAsset}};
use super::types::{
    CkanFile,
    CkanDependency,
    CkanDownloadHash,
    CkanInstallDirective,
    CkanReleaseStatus,
    CkanResources,
};

struct FileTask {
    identifier: String,
    name: String,
    asset_pattern: Option<String>,
    provides: Vec<String>,
    conflicts: Vec<CkanDependency>,
}

struct ReleaseContext {
    description: String,
    authors: Vec<String>,
    version: String,
    tags: Vec<String>,
    license: String,
    release_status: CkanReleaseStatus,
    resources: CkanResources,
    depends: Vec<CkanDependency>,
    recommends: Vec<CkanDependency>,
    install: Vec<CkanInstallDirective>,
    ksp_version: String,
    publish_date: chrono::DateTime<chrono::Utc>,
    base_id: String,
}

pub struct GenerateOptions<'a> {
    pub mod_config: Mod,
    pub out_dir: &'a std::path::Path,
    pub gh: &'a GithubClient,
    pub version: Option<String>,
}

#[instrument(
    name = "generate_ckan",
    skip(options),
    fields(mod_id = %options.mod_config.identifier, version = %options.version.clone().unwrap_or("latest".to_string()))
)]
pub async fn generate(options: GenerateOptions<'_>) -> Result<(), Box<dyn std::error::Error>> {
    let GenerateOptions {
        mod_config,
        out_dir,
        gh,
        version,
    } = options;
    tracing::info!("Generating CKAN file");

    let (owner, repo) = mod_config
        .repo
        .split_once('/')
        .ok_or("Invalid repo format, expected owner/repo")?;

    let description = if let Some(abstract_) = mod_config.abstract_ {
        abstract_
    } else {
        tracing::debug!("No description set, pulling repo metadata from GitHub");
        gh.get_repo_info(owner, repo).await?.description
    };

    tracing::debug!(
        "Fetching release information from GitHub for {}/{}@{}",
        owner,
        repo,
        version.clone().unwrap_or("latest".to_string())
    );
    let release_info = if let Some(ver) = version {
        gh.get_release_by_tag(owner, repo, &ver).await?
    } else {
        gh.get_latest_release(owner, repo).await?
    };

    let release_status = if release_info.prerelease {
        CkanReleaseStatus::Testing
    } else {
        CkanReleaseStatus::Stable
    };
    let version = release_info.tag_name;
    let publish_date = match release_info.published_at {
        Some(date) => date,
        None => {
            tracing::warn!("Release does not have a published date, using current time");
            chrono::Utc::now()
        }
    };

    tracing::debug!(
        "Fetched release information: tag_name = {}, published_at = {}",
        version,
        publish_date
    );

    let base_id = mod_config.identifier.clone();
    let base_depends: Vec<CkanDependency> = mod_config
        .dependencies
        .into_iter()
        .map(CkanDependency::from)
        .collect();
    let base_conflicts: Vec<CkanDependency> = mod_config
        .conflicts
        .into_iter()
        .map(CkanDependency::from)
        .collect();
    let base_recommends: Vec<CkanDependency> = mod_config
        .recommends
        .into_iter()
        .map(CkanDependency::from)
        .collect();
    let base_install: Vec<CkanInstallDirective> = mod_config
        .install
        .into_iter()
        .map(CkanInstallDirective::from)
        .collect();
    let base_resources = CkanResources::from_config(mod_config.resources, &mod_config.repo);

    let tasks: Vec<FileTask> = if mod_config.variants.is_empty() {
        vec![FileTask {
            identifier: base_id.clone(),
            name: mod_config.name.clone(),
            asset_pattern: mod_config.asset_match.clone(),
            provides: mod_config.provides.clone(),
            conflicts: base_conflicts.clone(),
        }]
    } else {
        mod_config
            .variants
            .iter()
            .map(|v| FileTask {
                identifier: format!("{}-{}", base_id, v.identifier),
                name: format!("{} ({})", mod_config.name, v.name),
                asset_pattern: Some(v.asset_match.clone()),
                provides: std::iter::once(base_id.clone())
                    .chain(mod_config.provides.iter().cloned())
                    .collect(),
                conflicts: mod_config
                    .variants
                    .iter()
                    .filter(|other| other.identifier != v.identifier)
                    .map(|other| CkanDependency {
                        name: format!("{}-{}", base_id, other.identifier),
                        version_spec: None,
                    })
                    .chain(base_conflicts.iter().cloned())
                    .collect(),
            })
            .collect()
    };

    let ctx = ReleaseContext {
        description,
        authors: mod_config.authors,
        version,
        tags: mod_config.tags,
        license: mod_config.license,
        release_status,
        resources: base_resources,
        depends: base_depends,
        recommends: base_recommends,
        install: base_install,
        ksp_version: mod_config.ksp_version,
        publish_date,
        base_id,
    };

    let results: Vec<_> = tasks
        .into_iter()
        .map(|task| generate_file(task, &release_info.assets, &out_dir, &ctx, &gh))
        .collect::<FuturesUnordered<_>>()
        .collect()
        .await;

    for result in results { result?; }

    Ok(())
}

#[instrument(
    name = "generate_ckan_file",
    skip(task, assets, out_dir, ctx, gh),
    fields(file_id = %task.identifier)
)]
async fn generate_file(
    task: FileTask,
    assets: &[octorust::types::ReleaseAsset],
    out_dir: &std::path::Path,
    ctx: &ReleaseContext,
    gh: &GithubClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let asset = if let Some(pattern) = &task.asset_pattern {
        let re = regex::Regex::new(pattern)?;
        assets
            .iter()
            .find(|a| re.is_match(&a.name))
            .ok_or(format!("No asset matching '{}' found in release", pattern))?
    } else {
        assets
            .first()
            .ok_or("Release does not have any assets".to_string())?
    };

    let workdir = std::env::temp_dir().join(env!("CARGO_PKG_NAME"));
    if !workdir.exists() {
        std::fs::create_dir_all(&workdir)?;
    }

    let DownloadedAsset {
        size: download_size,
        hash_sha256: download_hash_sha256,
        hash_sha1: download_hash_sha1,
        temp_file
    } = gh.download_and_hash(asset.clone(), Some(&workdir)).await?;

    let install_size = check_install_size(temp_file.path())?;
    tracing::debug!("Install size: {}", install_size);

    let ckan_file = CkanFile {
        identifier: task.identifier.clone(),
        name: task.name,
        abstract_: ctx.description.clone(),
        author: ctx.authors.clone(),
        version: ctx.version.clone(),
        tags: ctx.tags.clone(),
        license: ctx.license.clone(),
        release_status: ctx.release_status.clone(),
        resources: ctx.resources.clone(),
        provides: task.provides,
        depends: ctx.depends.clone(),
        conflicts: task.conflicts,
        recommends: ctx.recommends.clone(),
        install: ctx.install.clone(),
        ksp_version: ctx.ksp_version.to_string(),
        download: asset.browser_download_url.clone(),
        download_size,
        download_hash: CkanDownloadHash {
            sha256: download_hash_sha256,
            sha1: download_hash_sha1,
        },
        // CKAN doesn't know how to process any other content types, so we hardcode it to
        // application/zip.
        download_content_type: "application/zip".to_string(),
        install_size,
        release_date: asset
            .updated_at
            .unwrap_or(ctx.publish_date)
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        x_generated_by: concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")).to_string(),
        spec_version: 1,
    };

    let json = serde_json::to_string_pretty(&ckan_file)?;
    let base_out_dir = out_dir.join(&ctx.base_id);
    let output_path = base_out_dir.join(format!("{}-{}.ckan", task.identifier, ctx.version));

    std::fs::create_dir_all(&base_out_dir)?;
    std::fs::write(&output_path, json)?;
    tracing::info!("Generated CKAN file");

    Ok(())
}

fn check_install_size(path: &std::path::Path) -> Result<u64, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    let mut total: u64 = 0;
    for i in 0..archive.len() {
        total += archive.by_index(i)?.size();
    }
    Ok(total)
}