#!/usr/bin/env python
import requests
import socket
import time

def main():
    runtime = 10
    PORT = 5005
    IP = '172.20.103.205'
    # IP = '127.0.0.1'
    r = requests.post("http://172.20.103.105/streamscape_api",
                      data='{"jsonrpc":"2.0","method":"network_status","id":"sbkb5u0c"}')
    json_string = r.json()
    start = time.time()
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    while time.time() - start < runtime:
        sock.sendto(str(json_string['result']), (IP, PORT))

if __name__ == '__main__':
    main()