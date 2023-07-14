#!/bin/bash

#cargo build --package demo --release
cargo build --package demo
go mod tidy
CGO_ENABLED=1 go run .
