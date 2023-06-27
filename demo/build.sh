#!/bin/bash
# cleanup
rm -rf ./go_gen
rm -rf src/go_ffi.h
rm -rf src/go_ffi.a

# generate
cargo build --package demo --release
go mod tidy
CGO_ENABLED=1 go run .
