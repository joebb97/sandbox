# Echo Server/Client in Go

Supports tcp and udp

## Run Server

```
$ go run echo_net.go -s [-proto tcp|udp] [-p port] [-a addr]
```

## Run Client
```
$ go run echo_net.go -c [-proto tcp|udp] [-p port] [-a addr]
```

## Expected Behavior

Whatever the client sends, the server sends back.
The server prints what it got from the client. The client prints what got sent back.
