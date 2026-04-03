use std::path::PathBuf;

use clap::{Parser, Subcommand};
use futures_util::StreamExt;
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::github::GithubClient;

mod ckan;
mod config;
mod github;

#[derive(Parser)]
#[command(
    name = "rkan",
    version,
    about = "A tool to generate CKAN files for KSP mods from GitHub repositories",
    long_about = None
)]
struct App {
    #[clap(subcommand)]
    cmd: Cmds,

    /// GitHub token to use for API requests
    #[clap(
        short,
        long,
        global = true,
        env = "GITHUB_TOKEN",
        hide_env_values = true
    )]
    github_token: Option<String>,

    /// Where to find the mod configuration files
    #[clap(short, long, global = true, default_value = "./configs")]
    configs_dir: PathBuf,

    /// Where to output the generated CKAN files
    #[clap(short, long, global = true, default_value = "./ckan")]
    out_dir: PathBuf,

    /// Number of mods to process in parallel
    #[clap(short, long, global = true, default_value = "4")]
    jobs: usize,
}

#[derive(Subcommand)]
enum Cmds {
    /// Generate the CKAN files for the given packages
    Generate {
        /// The mod to generate the CKAN files for
        #[clap(value_delimiter = ',')]
        filter: Vec<String>,
    },
}

#[tokio::main]
async fn main() {
    let indicatif_layer = IndicatifLayer::new();
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_writer(indicatif_layer.get_stderr_writer()))
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(tracing::Level::INFO.into())
                .from_env_lossy(),
        )
        .with(indicatif_layer)
        .init();

    let app = App::parse();
    let configs = find_all_configs(&app.configs_dir);
    let gh = GithubClient::new(app.github_token.unwrap());

    match app.cmd {
        Cmds::Generate { filter } => {
            let filtered = if filter.is_empty() {
                configs
            } else {
                configs
                    .into_iter()
                    .filter(|c| filter.contains(&c.identifier))
                    .collect()
            };

            futures_util::stream::iter(filtered)
                .map(|mod_config| {
                    let out_dir = app.out_dir.clone();
                    let gh = gh.clone();
                    async move {
                        ckan::generate(ckan::GenerateOptions {
                            mod_config,
                            out_dir,
                            gh,
                            version: None,
                        })
                        .await
                        .map_err(|e| {
                            tracing::error!(error = ?e, "Failed to generate CKAN file");
                            e
                        })
                        .ok()
                    }
                })
                .buffer_unordered(app.jobs)
                .collect::<Vec<_>>()
                .await;
        }
    }
}

fn find_all_configs(configs_dir: &PathBuf) -> Vec<config::Mod> {
    let mut configs = Vec::new();

    for entry in std::fs::read_dir(configs_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        tracing::debug!(?path, "Checking entry: {:?}", entry.file_name());

        if path.is_dir() {
            configs.extend(find_all_configs(&path));
        } else if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("toml") {
            let config_data = std::fs::read_to_string(&path).unwrap();
            let config: config::Mod = toml::from_str(&config_data).unwrap();
            tracing::debug!(?path, "Loaded mod config: {:?}", config.identifier);
            configs.push(config);
        }
    }

    configs
}
