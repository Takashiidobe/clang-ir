//! Snapshot-tests the parser against every `*.c`/`*.cpp` file found
//! recursively under `tests/` (fixtures, stdlib, gcc-torture, chibicc,
//! fixtures.cxx, ...).
//!
//! Compiling ~2500 files with clang is the dominant cost, so that pass runs
//! in parallel across a worker pool; the actual (cheap, pure-Rust) parse and
//! snapshot assertion happens sequentially afterwards.
//!
//! Override the compiler with `CLANG_OPT` if it's not at the
//! default dev-machine path. `cir-opt` is located the normal way (see
//! [`clang_ir_rs::Toolchain`]), via `CIR_OPT`, the default dev-machine path,
//! or `PATH`.

use std::path::{Path, PathBuf};
use std::process::Command;

fn clang_path() -> PathBuf {
    if let Some(p) = std::env::var_os("CLANG_OPT") {
        return PathBuf::from(p);
    }
    let home = std::env::var("HOME").expect("HOME not set");
    PathBuf::from(home).join("llvm-project/build-cir/bin/clang")
}

fn collect_c_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
    entries.sort_by_key(|e| e.path());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            collect_c_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "c" || e == "cpp") {
            out.push(path);
        }
    }
}

enum Compiled {
    Cir(String),
    Failed(String),
}

fn compile_one(clang: &Path, file: &Path) -> Compiled {
    let mut cmd = Command::new("timeout");
    cmd.arg("20").arg(clang).arg("-emit-cir").arg("-fclangir");
    // C++ fixtures (`fixtures.cxx`) opt into a language mode via extension;
    // everything else is plain C.
    if file.extension().is_some_and(|e| e == "cpp") {
        cmd.arg("-std=c++17");
    }
    let output = cmd
        .arg("-S")
        .arg("-o")
        .arg("-")
        .arg("-c")
        .arg(file)
        .output();
    match output {
        Ok(o) if o.status.success() && !o.stdout.is_empty() => {
            Compiled::Cir(String::from_utf8_lossy(&o.stdout).into_owned())
        }
        Ok(o) => {
            let stderr = String::from_utf8_lossy(&o.stderr);
            let first_line = stderr.lines().find(|l| !l.trim().is_empty()).unwrap_or("");
            Compiled::Failed(format!("clang exit {}: {}", o.status, first_line))
        }
        Err(e) => Compiled::Failed(format!("failed to spawn clang: {e}")),
    }
}

/// Sanitizes a path relative to `tests/` into a valid, unique snapshot name.
fn snapshot_name(tests_dir: &Path, file: &Path) -> String {
    let rel = file.strip_prefix(tests_dir).unwrap_or(file);
    rel.with_extension("")
        .to_string_lossy()
        .replace(['/', '\\'], "__")
        .replace(['.', '-'], "_")
}

#[test]
fn parse_all_fixtures() {
    let tests_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    // Restricts the scan to a single subdirectory, e.g. `fixtures`, for fast
    // iteration while developing the parser without running the full corpus.
    let scan_root = match std::env::var("CLANG_IR_RS_TEST_SUBDIR") {
        Ok(sub) => tests_dir.join(sub),
        Err(_) => tests_dir.clone(),
    };
    let mut files = Vec::new();
    collect_c_files(&scan_root, &mut files);
    files.sort();
    assert!(
        !files.is_empty(),
        "expected to find *.c/*.cpp files under {}",
        tests_dir.display()
    );
    eprintln!("found {} .c/.cpp files", files.len());

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let clang = clang_path();
    let worker_count = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);

    let mut results: Vec<Option<Compiled>> = (0..files.len()).map(|_| None).collect();
    std::thread::scope(|scope| {
        let chunk_size = files.len().div_ceil(worker_count).max(1);
        let file_chunks = files.chunks(chunk_size);
        let result_chunks = results.chunks_mut(chunk_size);
        for (file_chunk, result_chunk) in file_chunks.zip(result_chunks) {
            let clang = &clang;
            scope.spawn(move || {
                for (file, slot) in file_chunk.iter().zip(result_chunk.iter_mut()) {
                    *slot = Some(compile_one(clang, file));
                }
            });
        }
    });

    let toolchain = clang_ir::Toolchain::from_env();
    let manifest_prefix = format!("{}/", manifest_dir.display());
    let mut compile_failed = 0usize;
    let mut parse_failed = 0usize;
    let mut parsed_ok = 0usize;

    for (file, compiled) in files.iter().zip(results) {
        let name = snapshot_name(&tests_dir, file);
        let snapshot = match compiled.expect("every file has a result") {
            Compiled::Failed(msg) => {
                compile_failed += 1;
                format!("CLANG_COMPILE_FAILED: {msg}")
            }
            Compiled::Cir(cir_text) => match clang_ir::parse_with(&toolchain, &cir_text) {
                Ok(module) => {
                    parsed_ok += 1;
                    // clang always embeds the module's source path as an
                    // absolute, canonicalized path regardless of how it was
                    // invoked (confirmed: passing a relative path with a
                    // matching cwd still resolves to absolute), so strip the
                    // checkout-specific prefix here to keep snapshots stable
                    // across clones/renames.
                    format!("{module}").replace(&manifest_prefix, "")
                }
                Err(e) => {
                    parse_failed += 1;
                    format!("PARSE_ERROR: {e}")
                }
            },
        };
        insta::assert_snapshot!(name, snapshot);
    }

    eprintln!(
        "summary: {parsed_ok} parsed OK, {parse_failed} parse errors, {compile_failed} clang compile failures (of {} total)",
        files.len()
    );
}
