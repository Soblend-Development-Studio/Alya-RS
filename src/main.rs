mod bot;
mod config;
mod db;
mod handlers;
mod commands;
mod services;
mod utils;

use std::sync::Arc;
use tracing::{info, error};
use anyhow::Result;

use crate::bot::AlyaBot;
use crate::config::BotConfig;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("alya_rs=info".parse()?)
                .add_directive("warn".parse()?)
        )
        .init();

    // Load .env if present
    dotenvy::dotenv().ok();

    let config = BotConfig::from_env();

    info!("✿ Alya Kujou Bot (Rust Edition) - Iniciando...");
    info!("✿ Version: {}", env!("CARGO_PKG_VERSION"));

    let bot = match AlyaBot::new(config).await {
        Ok(b) => Arc::new(b),
        Err(e) => {
            error!("ꕢ Error inicializando el bot: {}", e);
            std::process::exit(1);
        }
    };

    info!("ꕣ Bot inicializado correctamente");

    // Setup graceful shutdown
    let bot_clone = bot.clone();
    tokio::spawn(async move {
        let _ = tokio::signal::ctrl_c().await;
        info!("\nCTRL+C recibido. Cerrando gracefully...");
        bot_clone.shutdown().await;
        std::process::exit(0);
    });

    // Start the bot
    if let Err(e) = bot.run().await {
        error!("ꕢ Error fatal en el bot: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
