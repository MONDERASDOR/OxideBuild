use std::path::{Path, PathBuf};
use std::fs;
use glob::glob;
use walkdir::WalkDir;
use crate::error::OxideError;

pub fn ensure_dir(path: &Path) -> anyhow::Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn find_files(pattern: &str) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in glob(pattern)? {
        files.push(entry?);
    }
    Ok(files)
}

pub fn recursive_copy(src: &Path, dest: &Path) -> anyhow::Result<()> {
    for entry in WalkDir::new(src) {
        let entry = entry?;
        let path = entry.path();
        let relative = path.strip_prefix(src)?;
        let dest_path = dest.join(relative);

        if path.is_dir() {
            fs::create_dir_all(dest_path)?;
        } else {
            fs::copy(path, dest_path)?;
        }
    }
    Ok(())
}
