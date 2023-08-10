# fcplug

Foreign-Clang-Plugin solution, such as solving rust and go two-way calls.

## Features

| ⇊Caller \ Callee⇉ | Go | Rust |
|-------------------|:--:|:----:|
| Go                | -  |  ✅   |
| Rust              | ✅  |  -   |

- Protobuf IDL serialization solution: Supported!
- Thrift IDL serialization solution: In development...
- No serialization solution: In development...

## Schematic

![Fcplug Schematic](https://github.com/andeya/fcplug/raw/HEAD/doc/fcplug_schematic.png)

## Prepare

- Install rust nightly

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default nightly
```

- Install go

> Download [Go](https://go.dev/doc/install)
>
> Version go≥1.18
>
> Set environment variables: `CGO_ENABLED=1`

- Install protoc

> Use [protoc v23.2](https://github.com/protocolbuffers/protobuf/releases/tag/v23.2)
>
> Use [protoc-gen-go v1.5.3](https://pkg.go.dev/github.com/golang/protobuf@v1.5.3/protoc-gen-go)

## Example of use

Take Protobuf IDL serialization solution as an example.

See the [echo_pb](https://github.com/andeya/fcplug/raw/HEAD/samples/echo_pb)

#### Step 1: create/prepare a crate

> Generally, Fcplug is executed in a Crate's build.sh,
> and the code is automatically generated to the current Crate.

- If you do not have a Crate, execute the following command to create it:

```shell
cargo new --lib {crate_name}
```

- Added C static libraries, types and some dependent packages, edited in Cargo.toml as follows:

```toml
[lib]
crate-type = ["rlib", "staticlib"]

[dependencies]
fcplug = "0.3"
pilota = "0.7.0"
serde = "1"
serde_json = "1"

[build-dependencies]
fcplug-build = "0.3"
```

#### Step 2: Write the IDL file that defines the FFI interface

> Write the IDL file {ffi_name} .proto in ProtoBuf format, you can put it in the root directory of {crate_name},
> the content example is as follows:

```protobuf
syntax = "proto3";

message Ping {
  string msg = 1;
}

message Pong {
  string msg = 1;
}

// go call rust
service RustFFI {
  rpc echo_rs (Ping) returns (Pong) {}
}

// rust call go
service GoFFI {
  rpc echo_go (Ping) returns (Pong) {}
}
```

#### Step 3: Scripting auto-generated code `build.rs`

```rust
#![allow(unused_imports)]

use fcplug_build::{Config, generate_code, UnitLikeStructPath};

fn main() {
    generate_code(Config {
        idl_file: "./echo.proto".into(),
        // 默认搜索 $GOROOT > $PATH 
        go_root_path: None,
        go_mod_parent: "github.com/andeya/fcplug/samples",
        target_crate_dir: None,
    })
        .unwrap();
}
```

#### Step 4: Preliminary Code Generation

- Execute under the current Crate:

```shell
cargo build
# `cargo test` and `cargo install` will also trigger the execution of build.rs to generate code
```

- Attach the generated src/{ffi_name}_ffi mod to Crate, that is, add mod {ffi_name}_ffi to the `lib.rs` file

#### Step 5: Implement the FFI interface

- On the rust side, you need to implement the specific trait RustFfi and trait GoFfi methods in the newly initialized
  file src/{ffi_name}_ffi/mod.rs.
  <br/> The complete sample code of the file is as follows:

```rust
#![allow(unused_variables)]

pub use echo_gen::*;
use fcplug::{GoFfiResult, TryIntoTBytes};
use fcplug::protobuf::PbMessage;

mod echo_gen;

impl RustFfi for FfiImpl {
    fn echo_rs(mut req: ::fcplug::RustFfiArg<Ping>) -> ::fcplug::ABIResult<::fcplug::TBytes<Pong>> {
        let _req = req.try_to_object::<PbMessage<_>>();
        #[cfg(debug_assertions)]
        println!("rust receive req: {:?}", _req);
        Pong {
            msg: "this is pong from rust".to_string(),
        }
            .try_into_tbytes::<PbMessage<_>>()
    }
}

impl GoFfi for FfiImpl {
    unsafe fn echo_go_set_result(mut go_ret: ::fcplug::RustFfiArg<Pong>) -> ::fcplug::GoFfiResult {
        #[cfg(debug_assertions)]
        return GoFfiResult::from_ok(go_ret.try_to_object::<PbMessage<_>>()?);
        #[cfg(not(debug_assertions))]
        return GoFfiResult::from_ok(go_ret.bytes().to_owned());
    }
}
```

- Implement the go GoFfi interface in the one-time generated file ./cgobin/clib_goffi_impl.go.
  <br/> The complete sample code of this file is as follows:

```go
package main

import (
	"fmt"

	"github.com/andeya/fcplug/samples/echo"
	"github.com/andeya/gust"
)

func init() {
	// TODO: Replace with your own implementation, then re-execute `cargo build`
	GlobalGoFfi = GoFfiImpl{}
}

type GoFfiImpl struct{}

func (g GoFfiImpl) EchoGo(req echo.TBytes[echo.Ping]) gust.EnumResult[echo.TBytes[*echo.Pong], ResultMsg] {
	fmt.Printf("go receive req: %v\n", req.PbUnmarshalUnchecked())
	return gust.EnumOk[echo.TBytes[*echo.Pong], ResultMsg](echo.TBytesFromPbUnchecked(&echo.Pong{
		Msg: "this is pong from go",
	}))
}
```

#### Step 6: Generate Final Code

Execute `cargo build` `cargo test` or `cargo install` under the current Crate, trigger the execution of build.rs, and
generate code.

> Note: When GoFfi is defined, after compiling or changing the code for the first time,
> a warning similar to the following will occur,
> and you should execute cargo build twice at this time
>
> *warning: ... to re-execute 'cargo build' to ensure the correctness of 'libgo_echo.a'*

Therefore, it is recommended to repeat cargo build three times directly in the `build.sh` script

```bash
#!/bin/bash

cargo build --release
cargo build --release
cargo build --release
```

#### Step 7: Testing

- Rust calls Go tests, you can add test functions in `lib.rs`,
  <br/>the sample code is as follows:

```rust
#![feature(test)]

extern crate test;

mod echo_ffi;


#[cfg(test)]
mod tests {
    use test::Bencher;

    use fcplug::protobuf::PbMessage;
    use fcplug::TryIntoTBytes;

    use crate::echo_ffi::{FfiImpl, GoFfiCall, Ping, Pong};

    #[test]
    fn test_call_echo_go() {
        let pong = unsafe {
            FfiImpl::echo_go::<Pong>(Ping {
                msg: "this is ping from rust".to_string(),
            }.try_into_tbytes::<PbMessage<_>>().unwrap())
        };
        println!("{:?}", pong);
    }

    #[bench]
    fn bench_call_echo_go(b: &mut Bencher) {
        let req = Ping {
            msg: "this is ping from rust".to_string(),
        }
            .try_into_tbytes::<PbMessage<_>>()
            .unwrap();
        b.iter(|| {
            let pong = unsafe { FfiImpl::echo_go::<Vec<u8>>(req.clone()) };
            let _ = test::black_box(pong);
        });
    }
}
```

- Go calls Rust test, add the file `go_call_rust_test.go` in the root directory,
  <br/>the sample code is as follows:

```go
package echo_test

import (
	"testing"

	"github.com/andeya/fcplug/samples/echo"
)

func TestEcho(t *testing.T) {
	ret := echo.GlobalRustFfi.EchoRs(echo.TBytesFromPbUnchecked[*echo.Ping](&echo.Ping{
		Msg: "this is ping from go",
	}))
	if ret.IsOk() {
		t.Logf("%#v", ret.PbUnmarshalUnchecked())
	} else {
		t.Logf("fail: err=%v", ret.AsError())
	}
	ret.Free()
}

```

## Asynchronous programming

- Rust Tokio asynchronous function calling Go synchronous function

```rust
use fcplug::protobuf::PbMessage;
use fcplug::TryIntoTBytes;
use tokio::task;

use crate::echo_ffi::{FfiImpl, GoFfiCall, Ping, Pong};

let pong = task::spawn_blocking(move | | {
// The opened task runs in a dedicated thread pool. 
// If this task is blocked, it will not affect the completion of other tasks
unsafe {
FfiImpl::echo_go::< Pong > (Ping {
msg: "this is ping from rust".to_string(),
}.try_into_tbytes::< PbMessage < _ > > ().unwrap())
}
}).await?;

```

- Go calls Rust, at least one side is an async function

> in development

## Benchmark

[See benchmark code](https://github.com/andeya/fcplug/blob/HEAD/samples/echo_pb/go_call_rust_test.go)

```text
goos: darwin
goarch: amd64
pkg: github.com/andeya/fcplug/demo
cpu: Intel(R) Core(TM) i7-1068NG7 CPU @ 2.30GHz
```

![Benchmark: fcplug(cgo->rust) vs pure go](https://github.com/andeya/fcplug/raw/HEAD/doc/benchmark.png)
