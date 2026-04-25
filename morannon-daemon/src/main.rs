mod auth;
mod config;
mod daemon;
mod endpoints;

use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version)]
struct Args {
    /// Path to the configuration file.
    #[arg(short, long)]
    config: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "morannon_daemon=info,tower_http=info".into()),
        )
        .init();

    let (config, _) = config::load(args.config)
        .inspect(|(_, path)| tracing::info!("Configuration loaded successfully from: {}", path.display()))?;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .name("morannon-daemon")
        .build()?
        .block_on(async {
            daemon::run(config.daemon).await
        })
}
