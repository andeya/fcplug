# echo_pb

Protobuf IDL codec sample.

## Compile go into a C dynamic library

Set build parameters `Config::use_goffi_cdylib` to `true`

```shell
cargo build -Z unstable-options --out-dir .
DYLD_LIBRARY_PATH=$DYLD_LIBRARY_PATH:{DIR}/fcplug/target/debug ./echo_pb
```
