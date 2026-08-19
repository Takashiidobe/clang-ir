use std::{fs, path::Path};

use proc_macro2::TokenStream;
use syn::parse2;
use tblgen::{RecordKeeper, TableGenParser};

pub(crate) fn parse_keeper(
    llvm: &str,
) -> Result<RecordKeeper<'static>, Box<dyn std::error::Error>> {
    let keeper = TableGenParser::new()
        .add_source_file("clang/CIR/Dialect/IR/CIROps.td")
        .add_include_directory(&format!("{llvm}/clang/include"))
        .add_include_directory(&format!("{llvm}/mlir/include"))
        .parse()?;
    Ok(keeper)
}

pub(crate) fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if src.is_dir() {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());
            if entry.file_name() == "target" || entry.file_name() == "Cargo.lock" {
                continue;
            }
            if src_path.is_dir() {
                copy_dir_all(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }
    } else {
        if let Some(parent) = dst.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(src, dst)?;
    }
    Ok(())
}

pub(crate) fn write_rust_file(
    path: &Path,
    tokens: TokenStream,
    file_doc: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file: syn::File = parse2(tokens)?;
    let body = prettyplease::unparse(&file);
    let body = normalize_doc_comments(&body);
    let text = format!("//! {file_doc}\n\n{body}");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, text)?;
    Ok(())
}

fn normalize_doc_comments(text: &str) -> String {
    let mut out = Vec::new();
    let mut in_block = false;
    let mut block_indent = String::new();

    for line in text.lines() {
        if in_block {
            if let Some(end) = line.find("*/") {
                let content = line[..end].trim().trim_start_matches('*').trim();
                if content.is_empty() {
                    out.push(format!("{block_indent}///"));
                } else {
                    out.push(format!("{block_indent}/// {content}"));
                }
                let rest = line[end + 2..].trim();
                if !rest.is_empty() {
                    out.push(rest.to_string());
                }
                in_block = false;
            } else {
                let content = line.trim().trim_start_matches('*').trim();
                if content.is_empty() {
                    out.push(format!("{block_indent}///"));
                } else {
                    out.push(format!("{block_indent}/// {content}"));
                }
            }
            continue;
        }

        if let Some(start) = line.find("/**") {
            let before = &line[..start];
            let indent_len = before.chars().count();
            block_indent = " ".repeat(indent_len);
            if !before.trim().is_empty() {
                out.push(before.to_string());
            }
            let after = &line[start + 3..];
            if let Some(end) = after.find("*/") {
                let content = after[..end].trim().trim_start_matches('*').trim();
                if content.is_empty() {
                    out.push(format!("{block_indent}///"));
                } else {
                    out.push(format!("{block_indent}/// {content}"));
                }
                let rest = after[end + 2..].trim();
                if !rest.is_empty() {
                    out.push(rest.to_string());
                }
            } else {
                let content = after.trim().trim_start_matches('*').trim();
                if content.is_empty() {
                    out.push(format!("{block_indent}///"));
                } else {
                    out.push(format!("{block_indent}/// {content}"));
                }
                in_block = true;
            }
        } else {
            out.push(line.to_string());
        }
    }

    out.join("\n")
}
