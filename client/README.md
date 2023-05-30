# Client

A client that uploads potentially small files to a server and later downloads them, ensuring that they are correct and not corrupted in any way. This is achieved through the use of a Merkle tree.

## Installation

Clone the repository and install the necessary dependencies.

## Usage

To build the client, use the following command from the root of the workspace:
bash
```
cargo build --release --package client
```

## Usage

To run the client, use the following command from the root of the workspace:
bash
```
./target/release/client [COMMAND] [OPTIONS]
```

### Uploading Files

To upload files to the server, select option 1. The client will prompt the user for the path to the directory containing the files. The client will then upload the files to the server and compute the Merkle tree root hash. The root hash will be saved to disk.

### Downloading Files

To download a file from the server and verify its integrity, select option 2. The client will prompt the user for the name of the file to download. The client will then request the file and a Merkle proof from the server. The client will use the proof to verify the integrity of the file.

## Shortcomings

The current implementation only supports uploading and downloading files from a single server. It does not support multiple servers or load balancing.

## Future Improvements

Given more time, the client could be improved to support multiple servers and load balancing. Additionally, the user interface could be improved to be more user-friendly.