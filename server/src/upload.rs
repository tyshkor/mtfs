use crate::{Batch, State};
use anyhow::Result;
use axum::{
    body::boxed,
    extract::{Multipart, Query},
    http::StatusCode,
    response::Response,
    Extension,
};
use common::BatchId;
use futures::TryStreamExt;
use merkle_tree::merkletree::MerkleTree;
use serde_derive::Deserialize;
use tracing::info;
use std::sync::Arc;
use std::{io, path::PathBuf};
use tokio::io::AsyncReadExt;
use tokio_util::io::StreamReader;

#[derive(Deserialize)]
pub struct UploadParams {
    batch_id: common::BatchId,
}

pub async fn upload(
    state: Extension<Arc<State>>,
    batch_id: BatchId,
    mut multipart: Multipart,
) -> Result<()> {
    let mut filename_vec = vec![];

    let mut bytes_vec = vec![];
    let mut paths = vec![];

    let batch_dir = PathBuf::from(".").join(PathBuf::from(batch_id.clone()));
    std::fs::create_dir(batch_dir.clone())?;

    while let Some(field) = multipart.next_field().await? {
        let filename = if let Some(filename) = field.file_name() {
            filename.to_string()
        } else {
            continue;
        };

        let body_with_io_error = field.map_err(|err| io::Error::new(io::ErrorKind::Other, err));

        let body_reader = StreamReader::new(body_with_io_error);

        let real_path = batch_dir.join(filename.clone());

        filename_vec.push(filename.clone());
        state
            .client
            .put_file(batch_dir.clone(), filename.clone(), Box::pin(body_reader))
            .await?;
        let mut file = tokio::fs::File::open(real_path.clone()).await?;
        paths.push(real_path);

        let mut bytes = Vec::new();
        let _ = file.read_to_end(&mut bytes);
        bytes_vec.push(bytes);
    }

    let tree = MerkleTree::from_vec(common::DIGEST, bytes_vec.clone());

    state
        .batch_tree_map
        .lock()
        .await
        .insert(batch_id, Batch::new(tree.clone(), paths));

    Ok(())
}

pub async fn upload_handler(
    state: Extension<Arc<State>>,
    Query(UploadParams { batch_id }): Query<UploadParams>,
    multipart: Multipart,
) -> Response {
    info!("About to upload a batch");
    match upload(state, batch_id, multipart).await {
        Ok(_) => Response::builder()
            .status(StatusCode::CREATED)
            .body(boxed("OK".to_string()))
            .unwrap(),
        Err(err) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(boxed(err.to_string()))
            .unwrap(),
    }
}
