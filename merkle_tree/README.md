# Merkle Tree

🌳🔒🔗

Merkle Tree is a Rust implementation of a Merkle tree using the `ring` crate for hashing functions and `serde` for serialization. 

## What is a Merkle Tree?

A Merkle tree is a hash-based data structure that allows efficient and secure verification of large data sets. It is widely used in distributed systems such as blockchain to ensure data integrity and security.

## Usage

```rust
use merkle_tree::MerkleTree;

fn main() {
    let data1: Vec<u8> = vec![8, 9, 0, 0, 2];
    let data2: Vec<u8> = vec![1, 3, 46, 789, 2];
    let data3: Vec<u8> = vec![7, 8, 9, 4, 67, 342798];

    let mut merkle_tree = MerkleTree::from_vec(vec![data1, data3, data3]);

    let root_hash = merkle_tree.root_hash();
    println!("Root hash: {:?}", root_hash);
}
```

## License

This project is licensed under the [MIT License](LICENSE).