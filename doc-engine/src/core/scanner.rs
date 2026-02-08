use std::fs;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::spi::traits::FileScanner;
use crate::spi::types::ScanError;

pub struct FileSystemScanner;

impl FileScanner for FileSystemScanner {
    fn scan_files(&self, root: &Path) -> Vec<PathBuf> {
        let mut files = Vec::new();

        for entry in WalkDir::new(root)
            .into_iter()
            .filter_entry(|e| {
                let name = e.file_name().to_string_lossy();
                // Skip hidden directories, target/, node_modules/
                if e.file_type().is_dir() {
                    if name.starts_with('.') || name == "target" || name == "node_modules" {
                        return false;
                    }
                }
                true
            })
        {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            if entry.file_type().is_file() {
                // Return paths relative to root
                if let Ok(rel) = entry.path().strip_prefix(root) {
                    files.push(rel.to_path_buf());
                }
            }
        }

        files
    }

    fn file_exists(&self, root: &Path, relative: &str) -> bool {
        root.join(relative).exists()
    }

    fn read_file(&self, path: &Path) -> Result<String, ScanError> {
        fs::read_to_string(path).map_err(ScanError::Io)
    }
}
