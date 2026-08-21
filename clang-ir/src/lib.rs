pub use clang_ir_types::ast;
pub use clang_ir_types::{attrs, enums, ops, types};

pub mod model;

mod error;
mod lexer;
mod parser;
mod toolchain;

use std::path::Path;

pub use ast::Module;
pub use error::{Error, Result};
pub use lexer::decode_escaped_bytes;
pub use toolchain::Toolchain;

pub fn parse_str(source: &str) -> Result<Module> {
    parse_str_with(&Toolchain::from_env(), source)
}

pub fn parse_str_with(toolchain: &Toolchain, source: &str) -> Result<Module> {
    let generic = toolchain.normalize_to_generic(source)?;
    parser::parse_generic_module(&generic)
}

pub fn parse_generic_str(source: &str) -> Result<Module> {
    parser::parse_generic_module(source)
}

pub fn parse_generic_str_model(source: &str) -> Result<model::Module> {
    parse_generic_str(source).map(model::Module::from_generic)
}

pub fn parse_file(path: impl AsRef<Path>) -> Result<Module> {
    parse_file_with(&Toolchain::from_env(), path)
}

pub fn parse_file_with(toolchain: &Toolchain, path: impl AsRef<Path>) -> Result<Module> {
    let source = std::fs::read_to_string(path)?;
    parse_str_with(toolchain, &source)
}

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
