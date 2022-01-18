# Echo Server/Client in Go

Supports tcp. Build with `bash -x ./scripts/build.sh` or `./scripts/build.sh`

## Run Server

```
$ cd cmd/; go run echo_net.go -s [-port port] [-a addr]
```
or
```
$ ./bin/mac/echo_net -s [-port port] [-a addr]
```

## Run Client
```
$ cd cmd/; go run echo_net.go -c [-port port] [-a addr]
```
or
```
$ ./bin/mac/echo_net -c [-port port] [-a addr]
```

## Expected Behavior

Whatever the client sends, the server sends back. Typically the server is run on linux in your docker compose network.
The server prints what it got from the client. The client prints what got sent back.

# Doing halfclose

Use the `-y` option for either server or client. Should only be specifed for one or the other, not both. When specifying `-y` for the server, don't specif `-y` for client and optionally specify `-e` for the client to test correct behavior. `-e` does nothing for the server
