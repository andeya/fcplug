# fcplug

Foreign Clang Plugin solution.

## Features

| ⇊Caller \ Callee⇉ |  Go  | Rust |
|-------------------|:----:|:----:|
| Go                |  -   |  ✅   |
| Rust              | TODO |  -   |

## Usage

See the [demo](./demo)

## Benchmark

[See benchmark code](./demo/main_test.go)

```text
goos: darwin
goarch: amd64
pkg: github.com/andeya/fcplug/demo
cpu: Intel(R) Core(TM) i7-1068NG7 CPU @ 2.30GHz
```

![Benchmark: fcplug(cgo->rust) vs pure go](doc/benchmark.png)
