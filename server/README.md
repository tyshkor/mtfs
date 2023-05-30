# MTFS Server

This project is a server implementation for the Merkle Tree file verification system. The server is designed to work in conjunction with a client and a Merkle tree to support uploading, downloading, and verifying files on the server.

## How it Works

The server receives upload requests from the client containing potentially small files {F0, F1, …, Fn}. Upon receipt, the server generates a Merkle tree that is used to verify the integrity of the files during future requests.

The client can request an arbitrary file from the server along with a Merkle proof for it. Using the proof, the client can verify that the file is correct and has not been tampered with during transport or by the server.

## Usage

To build the server, use the following command from the root of the workspace:
bash
```
cargo build --release --package server
```

To use this server implementation, you will need to deploy it on a machine accessible to the client. Once deployed, start the server using the following command:

```
$ ./target/release/server
```

The server will listen on port 8000 for incoming requests.

## Dependencies

This server implementation is written in Rust and uses the following main dependencies:

- `axum` for web server functionality
- `serde` for serialization and deserialization of data
- local `merkle_tree` implementation

## Contributors

This project is open for contributions. If you would like to contribute, please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License - see the LICENSE.md file for details.