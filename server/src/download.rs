use crate::State;
use anyhow::Result;
use axum::{
    body::{boxed, StreamBody},
    extract::Query,
    http::StatusCode,
    response::Response,
    Extension,
};
use common::BatchId;
use serde_derive::Deserialize;
use std::{pin::Pin, sync::Arc};
use tokio::io::{AsyncRead, AsyncReadExt};
use tokio_util::io::ReaderStream;
use tracing::info;

#[derive(Deserialize)]
pub struct DownloadParams {
    file_index: u64,
    batch_id: BatchId,
}

pub async fn download(
    state: Extension<Arc<State>>,
    file_index: u64,
    batch_id: BatchId,
) -> Result<(String, Pin<Box<dyn AsyncRead + Send + Sync>>)> {
    let map = state.batch_tree_map.lock().await;

    let batch = map
        .get(&batch_id)
        .ok_or(anyhow::Error::msg("no such batch"))?;

    let real_path = batch.paths[file_index as usize].clone();
    let mut file = tokio::fs::File::open(real_path.clone()).await?;

    let mut bytes = Vec::new();
    let _ = file.read_to_end(&mut bytes);

    let proof = batch
        .tree
        .gen_proof(bytes.clone())
        .ok_or(anyhow::Error::msg("not found in tree"))?;
    let stream = state.client.get_file(real_path).await?;

    let proof_string = serde_json::to_string(&proof)?;

    Ok((proof_string, stream))
}

pub async fn download_handler(
    state: Extension<Arc<State>>,
    Query(DownloadParams {
        file_index,
        batch_id,
    }): Query<DownloadParams>,
) -> Response {
    info!("About to downlaod a file");
    match download(state, file_index, batch_id).await {
        Ok((proof, stream)) => {
            info!("Download was successful");
            Response::builder()
                .header(common::PROOF_HEADER, proof)
                .body(boxed(StreamBody::new(ReaderStream::new(stream))))
                .unwrap()
        }
        Err(err) => {
            info!("Download failed, because: {}", err);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(boxed(err.to_string()))
                .unwrap()
        }
    }
}
