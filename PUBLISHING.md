# Publishing

Release checklist:

- [ ] Bump the parent version in `clang-ir-types-gen/template/Cargo.toml`.
- [ ] Set the same parent version in `clang-ir/Cargo.toml`.
- [ ] Regenerate `clang-ir-types` from the intended LLVM checkout; its generated
  version will include the counter and LLVM commit, such as
  `0.1.5-16-6bcb0a8b3d94`.
- [ ] Set the `clang-ir-types` dependency in `clang-ir/Cargo.toml` to that exact
  generated version.
- [ ] Push one tag matching the `clang-ir` version, such as `v0.1.5` (the `v`
  prefix is optional).

The release workflow publishes `clang-ir-types`, waits for it to appear on
crates.io, then publishes `clang-ir`.
