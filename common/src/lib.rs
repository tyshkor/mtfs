use ring::digest::{Algorithm, SHA512};

pub type BatchId = String;

pub static DIGEST: &Algorithm = &SHA512;

pub const PROOF_HEADER: &str = "proof";
pub const PORT: &str = "PORT";
pub const DEFAULT_PORT: u64 = 8080;
pub const ADDRESS: &str = "ADDRESS";
pub const DEFAULT_ADDRESS: &str = "0.0.0.0";
pub const UPLOAD_ROUTE: &str = "/upload";
pub const DOWNLOAD_ROUTE: &str = "/download";
