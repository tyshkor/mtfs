# MTFS - Merkle Tree File Service

🌲📂 A client-server application that securely transfers and verifies files using a Merkle tree.

## Description

This project is an implementation of a client-server application that securely transfers and verifies files using a Merkle tree. The client has a large set of potentially small files {F0, F1, …, Fn} and wants to upload them to a server and then delete its local copies. Later, the client can download an arbitrary file from the server and be convinced that the file is correct and is not corrupted in any way (in transport, tampered with by the server, etc.).

The application consists of a client, server, and a Merkle tree. The Merkle tree is implemented in the application rather than using a library, but a library can be used for the underlying hash functions. The client computes a single Merkle tree root hash and keeps it on its disk after uploading the files to the server and deleting its local copies. The client can request the i-th file Fi and a Merkle proof Pi for it from the server. The client uses the proof and compares the resulting root hash with the one it persisted before deleting the files - if they match, the file is correct.

## Demo

A demo of the app that can be tried is provided using Docker Compose. The `docker-compose.yml` file contains the necessary configurations to run the client and server containers.

```
$ docker-compose up
```

## Crates

This workspace contains the following crates:

* [client](client/README.md)
* [server](server/README.md)
* [common](common/README.md)
* [merkle_tree](merkle_tree/README.md)

Each crate has its own README.md file that provides more information about the crate.

## Report

A report is provided explaining the approach, other ideas, what went well or not, and any short-comings the solution has. Additionally, suggestions on how to improve on these short-comings given more time are included.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.