use crate::hashing::{Hashable, Hashing};
use ring::digest::{Algorithm, Digest};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BinaryTree<T> {
    Empty {
        hash: Vec<u8>,
    },
    Leaf {
        hash: Vec<u8>,
        value: T,
    },
    Node {
        hash: Vec<u8>,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    },
}

impl<T> BinaryTree<T> {
    pub fn empty(hash: Digest) -> Self {
        BinaryTree::Empty {
            hash: hash.as_ref().into(),
        }
    }

    pub fn new(hash: Digest, value: T) -> Self {
        BinaryTree::Leaf {
            hash: hash.as_ref().into(),
            value,
        }
    }

    pub fn new_leaf(algo: &'static Algorithm, value: T) -> BinaryTree<T>
    where
        T: Hashable,
    {
        let hash = algo.hash_leaf(&value);
        BinaryTree::new(hash, value)
    }

    pub fn hash(&self) -> &Vec<u8> {
        match *self {
            BinaryTree::Empty { ref hash } => hash,
            BinaryTree::Leaf { ref hash, .. } => hash,
            BinaryTree::Node { ref hash, .. } => hash,
        }
    }
}
