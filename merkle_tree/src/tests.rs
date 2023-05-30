#![cfg(test)]

extern crate serde_json;

use crate::hashing::Hashing;
use crate::merkletree::MerkleTree;
use ring::digest::{Algorithm, SHA512};

static DIGEST: &Algorithm = &SHA512;

#[test]
fn test_from_vec() {
    let values = vec![vec![1], vec![2], vec![3]];
    let tree = MerkleTree::from_vec(DIGEST, values);

    let hashes = vec![
        DIGEST.hash_leaf(&vec![1]),
        DIGEST.hash_leaf(&vec![2]),
        DIGEST.hash_leaf(&vec![3]),
    ];

    let h01 = DIGEST.hash_nodes(&hashes[0], &hashes[1]);
    let h2 = &hashes[2];
    let root_hash = &DIGEST.hash_nodes(&h01, h2);

    assert_eq!(tree.count(), 3);
    assert_eq!(tree.height(), 2);
    assert_eq!(tree.root_hash().as_slice(), root_hash.as_ref());
}

#[test]
fn test_valid_proof() {
    let values = (1..10).map(|x| vec![x]).collect::<Vec<_>>();
    let tree = MerkleTree::from_vec(DIGEST, values.clone());
    let root_hash = tree.root_hash();

    for value in values {
        let proof = tree.gen_proof(value);
        let is_valid = proof.map(|p| p.validate(&root_hash)).unwrap_or(false);

        assert!(is_valid);
    }
}

#[test]
fn test_valid_proof_str() {
    let values = vec!["Hello", "my", "name", "is", "Rusty"];
    let tree = MerkleTree::from_vec(DIGEST, values);
    let root_hash = tree.root_hash();

    let value = "Rusty";

    let proof = tree.gen_proof(&value);
    let is_valid = proof.map(|p| p.validate(&root_hash)).unwrap_or(false);

    assert!(is_valid);
}

#[test]
fn test_wrong_proof() {
    let values1 = vec![vec![1], vec![2], vec![3], vec![4]];
    let tree1 = MerkleTree::from_vec(DIGEST, values1.clone());

    let values2 = vec![vec![4], vec![5], vec![6], vec![7]];
    let tree2 = MerkleTree::from_vec(DIGEST, values2);

    let root_hash = tree2.root_hash();

    for value in values1 {
        let proof = tree1.gen_proof(value);
        let is_valid = proof.map(|p| p.validate(root_hash)).unwrap_or(false);

        assert_eq!(is_valid, false);
    }
}

#[test]
fn test_nth_proof() {
    // Calculation depends on the total count. Try a few numbers: odd, even, powers of two...
    for &count in &[1, 2, 3, 10, 15, 16, 17, 22] {
        let values = (1..=count).map(|x| vec![x as u8]).collect::<Vec<_>>();
        let tree = MerkleTree::from_vec(DIGEST, values.clone());
        let root_hash = tree.root_hash();

        for i in 0..count {
            let proof = tree.gen_nth_proof(i).expect("gen proof by index");
            assert_eq!(vec![i as u8 + 1], proof.value);
            assert!(proof.validate(&root_hash));
            assert_eq!(i, proof.index(tree.count()));
        }

        assert!(tree.gen_nth_proof(count).is_none());
        assert!(tree.gen_nth_proof(count + 1000).is_none());
    }
}

#[test]
fn test_serialize_proof_with_serde() {
    let values = (1..10).map(|x| vec![x]).collect::<Vec<_>>();
    let tree = MerkleTree::from_vec(DIGEST, values);
    let proof = tree.gen_proof(vec![5]);

    let serialized = serde_json::to_string(&proof).expect("serialize proof");

    assert_eq!(
        proof,
        serde_json::from_str(&serialized).expect("deserialize proof")
    );
}
