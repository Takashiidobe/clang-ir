use std::{env, fs, path::PathBuf};

mod attrs;
mod common;
mod enums;
mod io;
mod ops;
mod types;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let llvm = env::var("LLVM_PROJECT_DIR").unwrap_or_else(|_| "/home/takashi/llvm-project".into());

    let out = "../clang-ir-types";
    let template = "./template";

    let keeper = io::parse_keeper(&llvm)?;
    let out_dir = PathBuf::from(out);
    let template_dir = PathBuf::from(template);
    if out_dir.exists() {
        fs::remove_dir_all(&out_dir)?;
    }
    fs::create_dir_all(&out_dir)?;
    io::copy_dir_all(&template_dir, &out_dir)?;

    io::write_rust_file(
        &out_dir.join("src/enums.rs"),
        enums::generate_enums(&keeper)?,
        "CIR enum attributes generated from TableGen enum definitions.",
    )?;
    io::write_rust_file(
        &out_dir.join("src/types.rs"),
        types::generate_types(&keeper)?,
        "CIR types generated from CIRTypes.td.",
    )?;
    io::write_rust_file(
        &out_dir.join("src/attrs.rs"),
        attrs::generate_attrs(&keeper)?,
        "CIR attributes generated from CIRAttrs.td and related files.",
    )?;

    let (op_modules, op_variants) = ops::collect_ops(&keeper)?;
    ops::write_ops(&out_dir.join("src/ops"), op_modules, op_variants)?;

    let out_dir = out_dir.canonicalize().unwrap_or(out_dir);
    println!("generated clang-ir-types crate in {}", out_dir.display());
    Ok(())
}
