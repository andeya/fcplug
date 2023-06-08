#!/bin/bash

cp -rf /Users/henrylee2cn/rust/fcplug/target/debug/demo.h .
cp -rf /Users/henrylee2cn/rust/fcplug/target/debug/libdemo.a .
mkdir -p go_gen
protoc --proto_path= --go_out go_gen idl.proto