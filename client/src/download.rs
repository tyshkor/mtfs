use crate::state::State;
use anyhow::Result;
use merkle_tree::proof::Proof;
use reqwest::Client;
use tracing::info;
use std::{io::Cursor, net::SocketAddr, path::PathBuf};

pub(crate) async fn download_file(
    client: Client,
    state: &State,
    addr: SocketAddr,
    file_name: &PathBuf,
    file_index: &u64,
    batch_id: &common::BatchId,
) -> Result<()> {
    let response = client
        .get(format!(
            "http://{}{}?file_index={}&batch_id={}",
            addr,
            common::DOWNLOAD_ROUTE,
            file_index,
            batch_id
        ))
        .send()
        .await?;
    let header = response
        .headers()
        .get(common::PROOF_HEADER)
        .ok_or(anyhow::Error::msg("proof header was not provided"))?;

    let proof: Proof<Vec<u8>> = serde_json::from_str(&header.to_str()?)?;

    let root_hash = state
        .batch_root_map
        .get(batch_id)
        .ok_or(anyhow::Error::msg("no such batch_id in state"))?;

    if proof.validate(root_hash) {
        info!("Proof is valid");
        let mut file = std::fs::File::create(file_name)?;
        let mut content = Cursor::new(response.bytes().await?);
        std::io::copy(&mut content, &mut file)?;
        info!("File was downloaded");
        Ok(())
    } else {
        Err(anyhow::Error::msg("Proof was not valid"))
    }
}
