use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Result;
use clap::Parser;
use commands::Commands;
use common::{ADDRESS, DEFAULT_ADDRESS, DEFAULT_PORT, PORT};
use reqwest::Client;
use state::State;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod commands;
mod download;
mod state;
mod upload;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let cli = Cli::parse();

    let mut state = State::load_state()?;

    let port: u16 = std::env::var(PORT)
        .ok()
        .unwrap_or(DEFAULT_PORT.to_string())
        .parse()?;

    let net: Ipv4Addr = std::env::var(ADDRESS)
        .ok()
        .unwrap_or(DEFAULT_ADDRESS.to_string())
        .parse()?;

    let addr = SocketAddr::from((net, port));

    info!("Address is {}", addr);

    // Create a new client.
    let client = Client::new();

    match &cli.command {
        Some(Commands::UploadBundle { batch_id, paths }) => {
            upload::upload_files(client.clone(), &mut state, addr, batch_id.clone(), paths).await?;
            Ok(())
        }
        Some(Commands::DownloadFile {
            batch_id,
            file_index,
            destination_path,
        }) => {
            download::download_file(client, &state, addr, destination_path, file_index, batch_id)
                .await?;
            Ok(())
        }
        None => Ok(()),
    }
}
