use std::{env, fs, path::PathBuf};

mod attrs;
mod common;
mod enums;
mod io;
mod ops;
mod types;
mod version;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_args()?;
    let keeper = io::parse_keeper(&config.llvm_dir.to_string_lossy())?;

    let base_version = version::template_version(&config.template_dir)?;
    let counter = version::read_counter(&config.counter_file)?;
    let llvm_commit = version::llvm_short_commit(&config.llvm_dir)?;
    let crate_version =
        version::types_crate_version(&base_version, counter, &llvm_commit);

    if config.out_dir.exists() {
        fs::remove_dir_all(&config.out_dir)?;
    }
    fs::create_dir_all(&config.out_dir)?;
    io::copy_dir_all(&config.template_dir, &config.out_dir)?;
    version::stamp_cargo_toml(
        &config.template_dir,
        &config.out_dir.join("Cargo.toml"),
        &crate_version,
    )?;

    io::write_rust_file(
        &config.out_dir.join("src/enums.rs"),
        enums::generate_enums(&keeper)?,
        "CIR enum attributes generated from TableGen enum definitions.",
    )?;
    io::write_rust_file(
        &config.out_dir.join("src/types.rs"),
        types::generate_types(&keeper)?,
        "CIR types generated from CIRTypes.td.",
    )?;
    io::write_rust_file(
        &config.out_dir.join("src/attrs.rs"),
        attrs::generate_attrs(&keeper)?,
        "CIR attributes generated from CIRAttrs.td and related files.",
    )?;

    let (op_modules, op_variants) = ops::collect_ops(&keeper)?;
    ops::write_ops(
        &config.out_dir.join("src/ops"),
        op_modules,
        op_variants,
    )?;

    let out_dir = config.out_dir.canonicalize().unwrap_or(config.out_dir);
    println!(
        "generated clang-ir-types {crate_version} in {}",
        out_dir.display()
    );
    Ok(())
}

struct Config {
    llvm_dir: PathBuf,
    out_dir: PathBuf,
    template_dir: PathBuf,
    counter_file: PathBuf,
}

impl Config {
    fn from_args() -> Result<Self, Box<dyn std::error::Error>> {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut llvm_dir = PathBuf::from(
            env::var("LLVM_PROJECT_DIR")
                .unwrap_or_else(|_| "/home/takashi/llvm-project".into()),
        );
        let mut out_dir = manifest_dir.join("../clang-ir-types");
        let mut template_dir = manifest_dir.join("template");
        let mut counter_file = manifest_dir.join("counter");

        let args = env::args().skip(1).collect::<Vec<_>>();
        let mut i = 0;
        while i < args.len() {
            let arg = args[i].as_str();
            let value_for = |i: &mut usize| -> Result<String, Box<dyn std::error::Error>> {
                *i += 1;
                args.get(*i).cloned().ok_or_else(|| {
                    format!("missing value for {}", args[*i - 1])
                        .into()
                })
            };

            if let Some(value) = arg.strip_prefix("--llvm-dir=") {
                llvm_dir = PathBuf::from(value);
            } else if arg == "--llvm-dir" {
                llvm_dir = PathBuf::from(value_for(&mut i)?);
            } else if let Some(value) = arg.strip_prefix("--out=") {
                out_dir = PathBuf::from(value);
            } else if arg == "--out" {
                out_dir = PathBuf::from(value_for(&mut i)?);
            } else if let Some(value) = arg.strip_prefix("--template=") {
                template_dir = PathBuf::from(value);
            } else if arg == "--template" {
                template_dir = PathBuf::from(value_for(&mut i)?);
            } else if let Some(value) = arg.strip_prefix("--counter-file=") {
                counter_file = PathBuf::from(value);
            } else if arg == "--counter-file" {
                counter_file = PathBuf::from(value_for(&mut i)?);
            } else {
                return Err(format!("unknown argument: {arg}").into());
            }
            i += 1;
        }

        Ok(Self {
            llvm_dir,
            out_dir,
            template_dir,
            counter_file,
        })
    }
}
