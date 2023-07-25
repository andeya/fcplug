# fcplug

Foreign-Clang-Plugin solution, such as solving rust and go two-way calls.

## Features

| ⇊Caller \ Callee⇉ |  Go  | Rust |
|-------------------|:----:|:----:|
| Go                |  -   |  ✅   |
| Rust              |  ✅ |  -   |

## Schematic

![Fcplug Schematic](https://github.com/andeya/fcplug/raw/HEAD/doc/fcplug_schematic.png)

## Usage

See the [echo](https://github.com/andeya/fcplug/raw/HEAD/samples/echo)

## Benchmark

[See benchmark code](https://github.com/andeya/fcplug/raw/HEAD/demo/main_test.go)

```text
goos: darwin
goarch: amd64
pkg: github.com/andeya/fcplug/demo
cpu: Intel(R) Core(TM) i7-1068NG7 CPU @ 2.30GHz
```

![Benchmark: fcplug(cgo->rust) vs pure go](https://github.com/andeya/fcplug/raw/HEAD/doc/benchmark.png)
