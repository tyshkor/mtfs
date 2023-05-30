use crate::state::State;
use anyhow::Result;
use merkle_tree::merkletree::MerkleTree;
use reqwest::{
    multipart::{self, Part},
    Body, Client,
};
use std::{net::SocketAddr, path::PathBuf};
use tokio::io::AsyncReadExt;
use tokio_util::codec::{BytesCodec, FramedRead};
use tracing::info;

pub(crate) async fn upload_files(
    client: Client,
    state: &mut State,
    addr: SocketAddr,
    batch_id: common::BatchId,
    file_name_vec: &[PathBuf],
) -> Result<()> {
    let part_vec: Vec<_> = file_name_vec
        .iter()
        .map(move |file_name| async {
            let mut file = tokio::fs::File::open(file_name.clone()).await?;

            let mut bytes = Vec::new();
            #[allow(clippy::let_underscore_future)]
            let _ = file.read_to_end(&mut bytes);
            let stream = FramedRead::new(file, BytesCodec::new());
            let file_body = Body::wrap_stream(stream);

            // Each file needs a form
            let some_file = multipart::Part::stream(file_body)
                .file_name(
                    file_name
                        .file_name()
                        .ok_or(anyhow::Error::msg("not a filename"))?
                        .to_str()
                        .ok_or(anyhow::Error::msg("not a valid str"))?
                        .to_string(),
                )
                .mime_str("application/octet-stream")?;

            let part_name = file_name
                .file_name()
                .ok_or(anyhow::Error::msg("not a filename"))?
                .to_str()
                .ok_or(anyhow::Error::msg("not a valid str"))?
                .to_string();

            Ok::<(Part, Vec<u8>, String), anyhow::Error>((some_file, bytes, part_name))
        })
        .collect();

    // Create the multipart form
    let mut form = multipart::Form::new();

    let mut bytes_vec = vec![];

    for tuple in part_vec.into_iter() {
        let (part, bytes, part_name) = tuple.await?;

        form = form.part(part_name, part);

        bytes_vec.push(bytes);
    }

    let tree = MerkleTree::from_vec(common::DIGEST, bytes_vec.clone());

    state
        .batch_root_map
        .insert(batch_id.clone(), tree.root_hash().clone());

    // Send the request
    let _ = client
        .post(format!(
            "http://{}{}?batch_id={}",
            addr,
            common::UPLOAD_ROUTE,
            batch_id
        ))
        .multipart(form)
        .send()
        .await?;
    info!("Request was sent");

    state.save_state()?;
    info!("State was updated");

    Ok(())
}
