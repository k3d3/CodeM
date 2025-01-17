use clap::Parser;
use std::{path::PathBuf, fs};
use anyhow::{Result, Context};

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
    let cli = Cli::parse();
    
    // Parse config file
    let config_str = fs::read_to_string(&cli.config)
        .with_context(|| format!("Failed to read config file: {}", cli.config.display()))?;

    // First parse into intermediate TOML format
    let toml_config: TomlConfig = toml::from_str(&config_str)
        .context("Failed to parse config file")?;
        
    // Then convert to ClientConfig, which performs validation
    let config = toml_config.into_client_config()
        .context("Failed to create client config")?;
        
    // Start server - now async all the way through
    server::serve(config).await?;
    
    Ok(())
}