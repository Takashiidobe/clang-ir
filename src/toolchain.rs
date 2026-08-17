use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::error::{Error, Result};

/// Locates and invokes `cir-opt` to normalize pretty-printed CIR text into
/// the dialect-agnostic generic MLIR op syntax our parser consumes.
#[derive(Debug, Clone)]
pub struct Toolchain {
    cir_opt: PathBuf,
}

impl Default for Toolchain {
    fn default() -> Self {
        Self::from_env()
    }
}

impl Toolchain {
    /// Resolves `cir-opt` from the `CIR_OPT` environment variable if set;
    /// otherwise falls back to `~/llvm-project/build-cir/bin/cir-opt` (the
    /// conventional dev-tree build location, mirroring how
    /// `tests/snapshot_fixtures.rs` locates `clang` via `CLANG_OPT`) if
    /// `HOME` is set, or bare `cir-opt` on `PATH` otherwise.
    pub fn from_env() -> Self {
        let cir_opt = std::env::var_os("CIR_OPT")
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var_os("HOME")
                    .map(|home| PathBuf::from(home).join("llvm-project/build-cir/bin/cir-opt"))
            })
            .unwrap_or_else(|| PathBuf::from("cir-opt"));
        Toolchain { cir_opt }
    }

    pub fn with_cir_opt(path: impl Into<PathBuf>) -> Self {
        Toolchain {
            cir_opt: path.into(),
        }
    }

    /// Runs `cir-opt --mlir-print-op-generic` on `source`, returning the
    /// normalized text (dialect-agnostic op syntax; CIR types/attributes
    /// keep their pretty printed form).
    pub fn normalize_to_generic(&self, source: &str) -> Result<String> {
        let mut child = Command::new(&self.cir_opt)
            .arg("-")
            .arg("--mlir-print-op-generic")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| {
                Error::Toolchain(format!("failed to spawn `{}`: {e}", self.cir_opt.display()))
            })?;

        child
            .stdin
            .take()
            .expect("piped stdin")
            .write_all(source.as_bytes())
            .map_err(|e| Error::Toolchain(format!("failed to write to cir-opt stdin: {e}")))?;

        let output = child
            .wait_with_output()
            .map_err(|e| Error::Toolchain(format!("failed to wait on cir-opt: {e}")))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::Toolchain(format!(
                "cir-opt exited with {}: {}",
                output.status,
                stderr.trim()
            )));
        }

        String::from_utf8(output.stdout)
            .map_err(|e| Error::Toolchain(format!("cir-opt produced non-UTF8 output: {e}")))
    }
}
