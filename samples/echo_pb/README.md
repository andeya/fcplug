# echo_pb

Protobuf IDL codec sample.

## Compile go into a C dynamic library

Set build parameters: `config.use_goffi_cdylib=true`

```shell
cargo build -Z unstable-options --out-dir .
DYLD_LIBRARY_PATH=$DYLD_LIBRARY_PATH:{DIR}/fcplug/target/debug ./echo_pb
```
