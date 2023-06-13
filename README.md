# fcplug

Foreign Clang Plugin solution.

## Usage

See the [demo](./demo)

## Benchmark

```text
goos: darwin
goarch: amd64
pkg: github.com/andeya/fcplug/demo
cpu: Intel(R) Core(TM) i7-1068NG7 CPU @ 2.30GHz
BenchmarkEcho_Rust_fb
BenchmarkEcho_Rust_fb-8   	  947886	      1215 ns/op
BenchmarkEcho_Rust_pb
BenchmarkEcho_Rust_pb-8   	  932937	      1216 ns/op
BenchmarkEcho_Go_pb
BenchmarkEcho_Go_pb-8     	  525447	      2246 ns/op
```
