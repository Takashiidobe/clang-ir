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

    /// Runs `cir-opt --mlir-print-op-generic --mlir-print-debuginfo` on
    /// `source`, returning the normalized text (dialect-agnostic op syntax;
    /// CIR types/attributes keep their pretty printed form). Without
    /// `--mlir-print-debuginfo`, `cir-opt` silently drops every `loc(...)` -
    /// including from input that already carries it - so it's required
    /// unconditionally, not just when the caller wants locations.
    pub fn normalize_to_generic(&self, source: &str) -> Result<String> {
        self.normalize_to_generic_with_flags(source, &[])
    }

    pub fn normalize_to_generic_with_flags(&self, source: &str, flags: &[&str]) -> Result<String> {
        let mut child = Command::new(&self.cir_opt)
            .arg("-")
            .args(flags)
            .arg("--mlir-print-op-generic")
            .arg("--mlir-print-debuginfo")
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

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn normalization_accepts_caller_supplied_flags() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("clang-ir-toolchain-{unique}"));
        std::fs::create_dir(&dir).unwrap();
        let tool = dir.join("cir-opt");
        std::fs::write(
            &tool,
            "#!/bin/sh\n[ \"$1\" = \"-\" ] || exit 11\n[ \"$2\" = \"--cir-flatten-cfg\" ] || exit 12\n[ \"$3\" = \"--cir-goto-solver\" ] || exit 13\n[ \"$4\" = \"--mlir-print-op-generic\" ] || exit 14\n[ \"$5\" = \"--mlir-print-debuginfo\" ] || exit 15\ncat\n",
        )
        .unwrap();
        let mut permissions = std::fs::metadata(&tool).unwrap().permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(&tool, permissions).unwrap();

        let normalized = Toolchain::with_cir_opt(&tool)
            .normalize_to_generic_with_flags(
                "builtin.module {}",
                &["--cir-flatten-cfg", "--cir-goto-solver"],
            )
            .unwrap();

        std::fs::remove_dir_all(dir).unwrap();
        assert_eq!(normalized, "builtin.module {}");
    }
}
