#!/bin/bash
# cleanup
rm -rf ./go_gen
rm -rf ./gen.sh
rm -rf ./demo
#exit 0;

#generate
cargo build --package demo
sh ./gen.sh
go mod tidy
CGO_ENABLED=1 go build
exec ./demo
