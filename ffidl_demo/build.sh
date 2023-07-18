#!/bin/bash

#cargo build --package ffidl_demo --release
cargo build --package ffidl_demo
go mod tidy
GOMODULE111=on CGO_ENABLED=1 go build ./src/gen
