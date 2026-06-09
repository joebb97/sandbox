# Rust Async Echo Server

A small Tokio TCP echo server that logs bytes as they are read, then echoes them
back to the client.

## Run

```sh
cargo run
```

The server listens on `127.0.0.1:8080`.

## Test

From another terminal:

```sh
nc 127.0.0.1 8080
```

Anything sent through `nc` is logged by the server and echoed back.
