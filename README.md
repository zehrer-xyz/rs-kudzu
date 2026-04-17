# rs-kudzu

`rs-kudzu` is a fork of [kuzudb/kuzu](https://github.com/kuzudb/kuzu). The codebase here still tracks the Kuzu database engine and its language bindings, but this repository is hosted separately at [zehrer-xyz/rs-kudzu](https://github.com/zehrer-xyz/rs-kudzu).

## What This Repository Contains

Kuzu is an embedded property graph database built for fast analytical queries on large datasets. The upstream engine and API surface are still present in this fork, including:

- the core C++ database engine in `src/`
- the Rust bindings in `tools/rust_api/`
- examples in `examples/`
- additional APIs in `tools/` for Python, Node.js, Java, and WebAssembly

## Quick Start

Build the default release target:

```bash
make release
```

Run the main C++ test suite:

```bash
make test
```

Run the Rust binding tests:

```bash
make rusttest
```

Build the examples:

```bash
make example
```

## Rust Bindings

The Rust crate lives in `tools/rust_api/`. There is also a small runnable example in `examples/rust/`.

To run the Rust example against a local database path:

```bash
cd examples/rust
cargo run -- /tmp/kuzu-example
```

## Build Notes

This fork keeps the upstream CMake and Makefile-based build flow. The main entry points are:

- `make release` for a release build
- `make debug` for a debug build
- `make test` for the main test suite
- `make rusttest` for Rust bindings
- `make all` to build the larger optional surface area

## Upstream References

If you need broader product documentation, API guides, or background on Kuzu itself, the upstream project is still the best reference point:

- Upstream repository: [kuzudb/kuzu](https://github.com/kuzudb/kuzu)
- Documentation: [kuzudb.github.io/docs](https://kuzudb.github.io/docs)
- Blog: [kuzudb.github.io/blog](https://kuzudb.github.io/blog)

## License

This repository is distributed under the [MIT License](LICENSE).
