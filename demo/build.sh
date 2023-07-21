#!/bin/bash

#cargo build --package demo --release
cargo build --package demo
go mod tidy
GOMODULE111=on CGO_ENABLED=1 go build ./src/gen
