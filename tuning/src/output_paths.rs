use std::{fs, io, path::{Path, PathBuf}};

/// Ensure the tuning output directory exists and return its path.
pub fn tuning_data_dir() -> io::Result<PathBuf> {
    let dir = std::env::current_dir()?.join("tuning_data");
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// Resolve a file name into the tuning output directory.
pub fn in_tuning_data(file_name: impl AsRef<Path>) -> io::Result<PathBuf> {
    Ok(tuning_data_dir()?.join(file_name))
}

/// Ensure the parent directory of a file path exists before writing.
pub fn ensure_parent_directory(path: impl AsRef<Path>) -> io::Result<()> {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}