#!/bin/bash
# cleanup
rm -rf ./go_gen
rm -rf ./gen.sh
#exit 0;

#generate
cargo build --package demo --release
sh ./gen.sh
go mod tidy
CGO_ENABLED=1 go run .
