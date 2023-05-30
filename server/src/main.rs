use anyhow::Result;
use axum::{
    routing::{on, MethodFilter},
    Extension, Router,
};
use batch::Batch;
use client::Client;
use common::{ADDRESS, DEFAULT_ADDRESS, DEFAULT_PORT, DOWNLOAD_ROUTE, PORT, UPLOAD_ROUTE};
use download::download_handler;
use std::{
    collections::BTreeMap,
    net::{Ipv4Addr, SocketAddr},
    path::PathBuf,
    sync::Arc,
};
use tokio::sync::Mutex;
use upload::upload_handler;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

pub mod batch;
pub mod client;
pub mod download;
pub mod upload;

const FILE_DIR_PATH: &str = "FILE_DIR_PATH";
const DEFAULT_FILE_DIR_PATH: &str = ".";

pub struct State {
    pub client: Arc<Client>,
    pub batch_tree_map: Mutex<BTreeMap<String, Batch>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let client = Arc::new(client::Client::new(PathBuf::from(
        std::env::var(FILE_DIR_PATH)
            .ok()
            .unwrap_or(DEFAULT_FILE_DIR_PATH.to_string()),
    )));

    info!("Client is ready");

    let state = Arc::new(State {
        client,
        batch_tree_map: Default::default(),
    });

    let router = Router::new()
        .route(UPLOAD_ROUTE, on(MethodFilter::POST, upload_handler))
        .route(DOWNLOAD_ROUTE, on(MethodFilter::GET, download_handler))
        .layer(Extension(state));

    info!("Router is ready");

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

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    info!("Going down");
    Ok(())
}
