# iperf Server/Client in Go

Supports tcp and udp

## Run Server

```
$ go run iperf.go -s [-proto tcp|udp] [-p port] [-a addr] [-b buffersize]
```

## Run Client
```
$ go run iperf.go -c [-proto tcp|udp] [-p port] [-a addr] [-b buffersize] [-t runtime]
```

## Expected Behavior

Should see the rate of transfer between server and client. For potentially more accurate results with TCP,
use `-b 131072` or `-b 262144` (128Kb and 256Kb, respectively) for both the server and client.
