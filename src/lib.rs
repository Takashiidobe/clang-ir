pub mod ast;
mod error;
mod lexer;
pub mod model;
mod parser;
mod toolchain;

use std::path::Path;

pub use ast::GenericModule;
pub use error::{Error, Result};
pub use toolchain::Toolchain;

/// Parses clang IR (CIR) textual output — as produced by `clang -emit-cir
/// -fclangir -S`, either directly or via [`parse_str`] on a file's
/// contents — into the generic, dialect-agnostic operation tree.
///
/// Internally normalizes the input through `cir-opt --mlir-print-op-generic`
/// (see [`Toolchain`]) before parsing, so `cir-opt` must be available (set
/// `CLANG_OPT` to point at it if it's not on `PATH`).
pub fn parse_str(source: &str) -> Result<GenericModule> {
    parse_str_with(&Toolchain::from_env(), source)
}

pub fn parse_str_with(toolchain: &Toolchain, source: &str) -> Result<GenericModule> {
    let generic = toolchain.normalize_to_generic(source)?;
    parser::parse_generic_module(&generic)
}

pub fn parse_file(path: impl AsRef<Path>) -> Result<GenericModule> {
    parse_file_with(&Toolchain::from_env(), path)
}

pub fn parse_file_with(toolchain: &Toolchain, path: impl AsRef<Path>) -> Result<GenericModule> {
    let source = std::fs::read_to_string(path)?;
    parse_str_with(toolchain, &source)
}

/// Parses clang IR text directly into the typed [`model::Module`]
/// representation (functions, globals, and their bodies as [`model::Instruction`]s).
pub fn parse(source: &str) -> Result<model::Module> {
    parse_str(source).map(model::Module::from_generic)
}

pub fn parse_with(toolchain: &Toolchain, source: &str) -> Result<model::Module> {
    parse_str_with(toolchain, source).map(model::Module::from_generic)
}

pub fn parse_module_file(path: impl AsRef<Path>) -> Result<model::Module> {
    parse_file(path).map(model::Module::from_generic)
}

pub fn parse_module_file_with(
    toolchain: &Toolchain,
    path: impl AsRef<Path>,
) -> Result<model::Module> {
    parse_file_with(toolchain, path).map(model::Module::from_generic)
}
