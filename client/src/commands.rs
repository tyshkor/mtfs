use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand)]
pub(crate) enum Commands {
    UploadBundle {
        #[arg(short, long)]
        batch_id: common::BatchId,
        #[arg(short, long)]
        paths: Vec<PathBuf>,
    },
    DownloadFile {
        #[arg(short, long)]
        batch_id: common::BatchId,
        #[arg(short, long)]
        file_index: u64,
        #[arg(short, long)]
        destination_path: PathBuf,
    },
}
