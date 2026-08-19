use std::{env, fs, path::Path, process::Command};

pub(crate) fn types_crate_version(base_version: &str, counter: u64, llvm_commit: &str) -> String {
    format!("{base_version}-{counter}-{llvm_commit}")
}

pub(crate) fn template_version(template_dir: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let cargo_toml = fs::read_to_string(template_dir.join("Cargo.toml"))?;
    parse_cargo_version(&cargo_toml)
}

pub(crate) fn read_counter(counter_file: &Path) -> Result<u64, Box<dyn std::error::Error>> {
    let value = fs::read_to_string(counter_file)?;
    let value = value.trim();
    if value.is_empty() {
        return Err(format!("counter file {} is empty", counter_file.display()).into());
    }
    value.parse::<u64>().map_err(|_| {
        format!(
            "counter file {} must contain an unsigned integer",
            counter_file.display()
        )
        .into()
    })
}

pub(crate) fn llvm_short_commit(llvm_dir: &Path) -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(hash) = env::var("CIR_TYPES_LLVM_COMMIT")
        && !hash.trim().is_empty()
    {
        return Ok(hash.trim().to_string());
    }

    let output = Command::new("git")
        .arg("-C")
        .arg(llvm_dir)
        .args(["rev-parse", "--short", "HEAD"])
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "failed to read LLVM commit hash from {}: {}",
            llvm_dir.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        )
        .into());
    }

    let hash = String::from_utf8(output.stdout)?.trim().to_string();
    if hash.is_empty() {
        return Err(format!(
            "git rev-parse --short HEAD returned an empty commit hash for {}",
            llvm_dir.display()
        )
        .into());
    }
    Ok(hash)
}

pub(crate) fn stamp_cargo_toml(
    template_dir: &Path,
    out_cargo_toml: &Path,
    version: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let template = fs::read_to_string(template_dir.join("Cargo.toml"))?;
    let mut stamped = Vec::new();
    let mut replaced = false;

    for line in template.lines() {
        if !replaced && line.trim_start().starts_with("version") {
            let indent = &line[..line.len() - line.trim_start().len()];
            stamped.push(format!("{indent}version = \"{version}\""));
            replaced = true;
        } else {
            stamped.push(line.to_string());
        }
    }

    if !replaced {
        return Err("template Cargo.toml does not contain a version field".into());
    }

    fs::write(out_cargo_toml, stamped.join("\n") + "\n")?;
    Ok(())
}

fn parse_cargo_version(cargo_toml: &str) -> Result<String, Box<dyn std::error::Error>> {
    for line in cargo_toml.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix("version") else {
            continue;
        };
        let Some(value) = rest.trim_start().strip_prefix('=') else {
            continue;
        };
        let value = value.trim();
        if value.len() >= 2 && value.starts_with('"') && value.ends_with('"') {
            return Ok(value[1..value.len() - 1].to_string());
        }
    }
    Err("template Cargo.toml does not contain a version field".into())
}
