use clap::Parser;
use std::{path::PathBuf, fs};
use anyhow::{Result, Context};
use tracing::info;
use tracing_subscriber::{self, fmt::format::FmtSpan};

mod error;
mod server;
mod tools;
mod config;

#[cfg(test)]
mod tests;
use config::TomlConfig;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to codem config file
    #[arg(value_name = "CONFIG_FILE")]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing subscriber
    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_ansi(false)
        .with_span_events(FmtSpan::CLOSE)
        .without_time()
        .init();

    info!("Starting Codem MCP server");
    let cli = Cli::parse();
    
    info!("Reading config file: {}", cli.config.display());
    // Parse config file
    let config_str = fs::read_to_string(&cli.config)
        .with_context(|| format!("Failed to read config file: {}", cli.config.display()))?;

    // First parse into intermediate TOML format
    let toml_config: TomlConfig = toml::from_str(&config_str)
        .context("Failed to parse config file")?;
        
    // Then convert to ClientConfig, which performs validation
    let config = toml_config.into_client_config().await
        .context("Failed to create client config")?;
        
    // Start server - now async all the way through
    info!("Starting server...");
    server::serve(config).await?;
    
    Ok(())
}