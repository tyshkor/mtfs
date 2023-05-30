use std::cmp::Ordering;
use ring::digest::Algorithm;
use serde_derive::{Deserialize, Serialize};
use crate::hashing::Hashing;
use crate::tree::BinaryTree;

/// A `Proof` stucture contains all data to prove that some value is a member
/// of a `MerkleTree` with root hash `root_hash`, and hash function `algorithm`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Proof<T> {
    /// The hashing algorithm used in the original `MerkleTree`
    #[serde(with = "algorithm_serde")]
    pub algorithm: &'static Algorithm,
    /// The hash of the root of the original `MerkleTree`
    pub root_hash: Vec<u8>,
    /// The first `Conjecture` of the `Proof`
    pub conjecture: Conjecture,
    /// The value concerned by this `Proof`
    pub value: T,
}

mod algorithm_serde {
    use ring::digest::{self, Algorithm};
    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(
        algorithm: &'static Algorithm,
        se: S,
    ) -> Result<S::Ok, S::Error> {
        format!("{:?}", algorithm).serialize(se)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<&'static Algorithm, D::Error> {
        let alg_str: String = Deserialize::deserialize(de)?;
        match &*alg_str {
            "SHA1" => Ok(&digest::SHA1_FOR_LEGACY_USE_ONLY),
            "SHA256" => Ok(&digest::SHA256),
            "SHA384" => Ok(&digest::SHA384),
            "SHA512" => Ok(&digest::SHA512),
            "SHA512_256" => Ok(&digest::SHA512_256),
            _ => Err(D::Error::custom("unknown hash algorithm")),
        }
    }
}

impl<T: PartialEq> PartialEq for Proof<T> {
    fn eq(&self, other: &Proof<T>) -> bool {
        self.root_hash == other.root_hash && self.conjecture == other.conjecture && self.value == other.value
    }
}

impl<T: Eq> Eq for Proof<T> {}

impl<T: Ord> PartialOrd for Proof<T> {
    fn partial_cmp(&self, other: &Proof<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> Ord for Proof<T> {
    fn cmp(&self, other: &Proof<T>) -> Ordering {
        self.root_hash
            .cmp(&other.root_hash)
            .then(self.value.cmp(&other.value))
            .then_with(|| self.conjecture.cmp(&other.conjecture))
    }
}

impl<T> Proof<T> {
    /// Constructs a new `Proof`
    pub fn new(algorithm: &'static Algorithm, root_hash: Vec<u8>, conjecture: Conjecture, value: T) -> Self {
        Proof {
            algorithm,
            root_hash,
            conjecture,
            value,
        }
    }

    /// Checks whether this inclusion proof is well-formed,
    /// and whether its root hash matches the given `root_hash`.
    pub fn validate(&self, root_hash: &[u8]) -> bool {
        if self.root_hash != root_hash || self.conjecture.node_hash != root_hash {
            return false;
        }

        self.conjecture.validate(self.algorithm)
    }

    /// Returns the index of this proof's value, given the total number of items in the tree.
    ///
    /// # Panics
    ///
    /// Panics if the proof is malformed. Call `validate` first.
    pub fn index(&self, count: usize) -> usize {
        self.conjecture.index(count)
    }
}

/// A `Conjecture` holds the hash of a node, the hash of its sibling node,
/// and a sub conjecture, whose `node_hash`, when combined with this `sibling_hash`
/// must be equal to this `node_hash`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Conjecture {
    pub node_hash: Vec<u8>,
    pub sibling_hash: Option<Side<Vec<u8>>>,
    pub sub_conjecture: Option<Box<Conjecture>>,
}

impl Conjecture {
    /// Attempts to generate a proof that the a value with hash `needle` is a
    /// member of the given `tree`.
    pub fn new<T>(tree: &BinaryTree<T>, needle: &[u8]) -> Option<Conjecture> {
        match *tree {
            BinaryTree::Empty { .. } => None,

            BinaryTree::Leaf { ref hash, .. } => Conjecture::new_leaf_proof(hash, needle),

            BinaryTree::Node {
                ref hash,
                ref left,
                ref right,
            } => Conjecture::new_tree_proof(hash, needle, left, right),
        }
    }

    /// Tries to generate a proof that the i-th leaf is a member of the given tree. 
    /// `count` must be equal to the number of leaves in the `tree`.
    /// Returns the new `Conjecture` and the i`-th value.
    /// `None` is returned in case `idx >= count`.
    pub fn new_by_index<T>(tree: &BinaryTree<T>, i: usize, count: usize) -> Option<(Conjecture, &T)> {
        if i >= count {
            return None;
        }
        match *tree {
            BinaryTree::Empty { .. } => None,
            BinaryTree::Leaf {
                ref hash,
                ref value,
                ..
            } => {
                if count != 1 {
                    return None;
                }
                let conjecture = Conjecture {
                    node_hash: hash.clone(),
                    sibling_hash: None,
                    sub_conjecture: None,
                };
                Some((conjecture, value))
            }
            BinaryTree::Node {
                ref hash,
                ref left,
                ref right,
            } => {
                let left_count = count.next_power_of_two() / 2;
                let (sub_conjecture_val, sibling_hash);
                if i < left_count {
                    sub_conjecture_val = Conjecture::new_by_index(left, i, left_count);
                    sibling_hash = Side::Right(right.hash().clone());
                } else {
                    sub_conjecture_val = Conjecture::new_by_index(right, i - left_count, count - left_count);
                    sibling_hash = Side::Left(left.hash().clone());
                }
                sub_conjecture_val.map(|(sub_conjecture, value)| {
                    let conjecture = Conjecture {
                        node_hash: hash.clone(),
                        sibling_hash: Some(sibling_hash),
                        sub_conjecture: Some(Box::new(sub_conjecture)),
                    };
                    (conjecture, value)
                })
            }
        }
    }

    /// Returns the index of this conjecture's value, given the total number of items in the tree.
    ///
    /// # Panics
    ///
    /// Panics if the conjecture is malformed. Call `validate_lemma` first.
    pub fn index(&self, count: usize) -> usize {
        let left_count = count.next_power_of_two() / 2;
        match (self.sub_conjecture.as_ref(), self.sibling_hash.as_ref()) {
            (None, None) => 0,
            (Some(l), Some(&Side::Left(_))) => left_count + l.index(count - left_count),
            (Some(l), Some(&Side::Right(_))) => l.index(left_count),
            (None, Some(_)) | (Some(_), None) => panic!("malformed conjecture"),
        }
    }

    fn new_leaf_proof(hash: &[u8], needle: &[u8]) -> Option<Conjecture> {
        if *hash == *needle {
            Some(Conjecture {
                node_hash: hash.into(),
                sibling_hash: None,
                sub_conjecture: None,
            })
        } else {
            None
        }
    }

    fn new_tree_proof<T>(
        hash: &[u8],
        needle: &[u8],
        left: &BinaryTree<T>,
        right: &BinaryTree<T>,
    ) -> Option<Conjecture> {
        Conjecture::new(left, needle)
            .map(|conjecture| {
                let right_hash = right.hash().clone();
                let sub_conjecture = Some(Side::Right(right_hash));
                (conjecture, sub_conjecture)
            })
            .or_else(|| {
                let sub_conjecture = Conjecture::new(right, needle);
                sub_conjecture.map(|conjecture| {
                    let left_hash = left.hash().clone();
                    let sub_conjecture = Some(Side::Left(left_hash));
                    (conjecture, sub_conjecture)
                })
            })
            .map(|(sub_conjecture, sibling_hash)| Conjecture {
                node_hash: hash.into(),
                sibling_hash,
                sub_conjecture: Some(Box::new(sub_conjecture)),
            })
    }

    fn validate(&self, algorithm: &'static Algorithm) -> bool {
        match self.sub_conjecture {
            None => self.sibling_hash.is_none(),
            Some(ref sub) => match self.sibling_hash {
                None => false,
                Some(Side::Left(ref hash)) => {
                    let combined = algorithm.hash_nodes(hash, &sub.node_hash);
                    let hashes_match = combined.as_ref() == self.node_hash.as_slice();
                    hashes_match && sub.validate(algorithm)
                }
                Some(Side::Right(ref hash)) => {
                    let combined = algorithm.hash_nodes(&sub.node_hash, hash);
                    let hashes_match = combined.as_ref() == self.node_hash.as_slice();
                    hashes_match && sub.validate(algorithm)
                }
            },
        }
    }
}

/// Tags a value so that we know from which branch of a `Tree` (if any) it was found.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Side<T> {
    /// The value was found in the left branch
    Left(T),
    /// The value was found in the right branch
    Right(T),
}
