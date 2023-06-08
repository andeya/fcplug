#!/bin/bash

cargo build --package demo -vv
sh ./gen.sh
go mod tidy
CGO_ENABLED=1 go build
exec ./demo
