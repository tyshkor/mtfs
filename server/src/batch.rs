use merkle_tree::merkletree::MerkleTree;
use std::path::PathBuf;

pub struct Batch {
    pub(crate) tree: MerkleTree<Vec<u8>>,
    pub(crate) paths: Vec<PathBuf>,
}

impl Batch {
    pub(crate) fn new(tree: MerkleTree<Vec<u8>>, paths: Vec<PathBuf>) -> Batch {
        Batch { tree, paths }
    }
}
