use std::path::PathBuf;

use clap::{Parser, Subcommand};
use futures_util::StreamExt;
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::config::find_all_configs;
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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();

    let app = App::parse();
    let gh = GithubClient::new(app.github_token)?;
    
    match app.cmd {
        Cmds::Generate { filter } => {
            let configs = find_all_configs(&app.configs_dir, filter);

            let errors = futures_util::stream::iter(configs)
                .map(|mod_config| {
                    let out_dir = app.out_dir.clone();
                    let gh = gh.clone();
                    async move {
                        ckan::generator::generate(ckan::generator::GenerateOptions {
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
                    }
                })
                .buffer_unordered(app.jobs)
                .collect::<Vec<Result<(), Box<dyn std::error::Error>>>>()
                .await;

            let error_count = errors.iter().filter(|r| r.is_err()).count();
            if error_count > 0 {
                tracing::error!("Generation completed with {} errors", error_count);
            } else {
                tracing::info!("Generation completed successfully");
            }

            Ok(())
        }
    }
}

fn init_tracing() {
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
}