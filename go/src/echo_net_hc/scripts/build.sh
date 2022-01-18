#!/bin/bash
set -x

mkdir -p bin
mkdir -p bin/mac
mkdir -p bin/linux

cd cmd/
env GOOS=darwin GOARCH=amd64 go build -o ../bin/mac
env GOOS=linux GOARCH=amd64 go build -o ../bin/linux
cd -
