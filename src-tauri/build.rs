use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn collect_starter_files(root: &Path, path: &Path, files: &mut Vec<(String, PathBuf)>) {
    let Ok(entries) = fs::read_dir(path) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_starter_files(root, &path, files);
            continue;
        }

        let is_supported = path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| matches!(extension, "md" | "csv"));
        if !is_supported {
            continue;
        }

        let relative = path
            .strip_prefix(root)
            .expect("starter file should be inside starter root")
            .to_string_lossy()
            .replace('\\', "/");
        files.push((relative, path));
    }
}

fn main() {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is required"));
    let starter_root = manifest_dir.join("..").join("starter-knowledge");
    let mut files = Vec::new();
    collect_starter_files(&starter_root, &starter_root, &mut files);
    files.sort_by(|left, right| left.0.cmp(&right.0));

    let mut generated = String::from("const STARTER_FILES: &[(&str, &str)] = &[\n");
    for (relative, absolute) in files {
        generated.push_str(&format!(
            "    ({relative:?}, include_str!({absolute:?})),\n",
            absolute = absolute.to_string_lossy()
        ));
    }
    generated.push_str("];\n");

    let output =
        PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is required")).join("starter_files.rs");
    fs::write(output, generated).expect("starter file manifest should be generated");
    println!("cargo:rerun-if-changed={}", starter_root.display());

    tauri_build::build()
}
