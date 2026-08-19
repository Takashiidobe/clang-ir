# clang-ir

[![crates.io](https://img.shields.io/crates/v/clang-ir.svg)](https://crates.io/crates/clang-ir)
[![docs.rs](https://docs.rs/clang-ir/badge.svg)](https://docs.rs/clang-ir)

A parser for Clang IR (CIR) textual output, produced by `clang -emit-cir -fclangir`
into a generic, dialect-agnostic operation tree, with an optional typed model on top
(functions, globals, and their bodies as instructions).

## Usage

```rust
let module = clang_ir::parse(source)?;
```

Parsing normalizes the input through `cir-opt --mlir-print-op-generic` first, so `cir-opt`
must be available: on `PATH`, or overriden by the `CIR_OPT` environment variable.

Use [`parse_str`]/[`parse_file`] for the generic operation tree, or [`parse`]/[`parse_module_file`]
for the typed model. `_with` variants accept an explicit [`Toolchain`] instead of resolving one
from the environment.

[`parse_str`]: https://docs.rs/clang-ir/latest/clang_ir/fn.parse_str.html
[`parse_file`]: https://docs.rs/clang-ir/latest/clang_ir/fn.parse_file.html
[`parse`]: https://docs.rs/clang-ir/latest/clang_ir/fn.parse.html
[`parse_module_file`]: https://docs.rs/clang-ir/latest/clang_ir/fn.parse_module_file.html
[`Toolchain`]: https://docs.rs/clang-ir/latest/clang_ir/struct.Toolchain.html

## Features

- `serde` for serialization/deserialization

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.
