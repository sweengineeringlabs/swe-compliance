use std::fs;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::api::traits::FileScanner;
use crate::api::types::ScanError;

pub struct FileSystemScanner;

impl FileScanner for FileSystemScanner {
    fn scan_files(&self, root: &Path) -> Vec<PathBuf> {
        let mut files = Vec::new();

        for entry in WalkDir::new(root)
            .into_iter()
            .filter_entry(|e| {
                let name = e.file_name().to_string_lossy();
                // Skip hidden directories, target/, node_modules/
                if e.file_type().is_dir()
                    && (name.starts_with('.') || name == "target" || name == "node_modules")
                {
                    return false;
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn make_tmp() -> TempDir {
        tempfile::Builder::new().prefix("test_").tempdir().unwrap()
    }

    #[test]
    fn test_scan_files_basic() {
        let tmp = make_tmp();
        fs::write(tmp.path().join("file.txt"), "content").unwrap();
        fs::create_dir(tmp.path().join("sub")).unwrap();
        fs::write(tmp.path().join("sub").join("nested.txt"), "content").unwrap();

        let scanner = FileSystemScanner;
        let files = scanner.scan_files(tmp.path());
        assert!(files.len() >= 2, "Expected >= 2 files, got {:?}", files);
        let names: Vec<String> = files.iter().map(|f| f.to_string_lossy().replace('\\', "/")).collect();
        assert!(names.iter().any(|n| n == "file.txt"));
        assert!(names.iter().any(|n| n == "sub/nested.txt"));
    }

    #[test]
    fn test_scan_files_relative() {
        let tmp = make_tmp();
        fs::write(tmp.path().join("file.txt"), "content").unwrap();

        let scanner = FileSystemScanner;
        let files = scanner.scan_files(tmp.path());
        for f in &files {
            assert!(f.is_relative());
        }
    }

    #[test]
    fn test_scan_skips_hidden() {
        let tmp = make_tmp();
        fs::create_dir(tmp.path().join(".git")).unwrap();
        fs::write(tmp.path().join(".git").join("config"), "content").unwrap();
        fs::write(tmp.path().join("visible.txt"), "content").unwrap();

        let scanner = FileSystemScanner;
        let files = scanner.scan_files(tmp.path());
        assert!(!files.iter().any(|f| f.to_string_lossy().contains(".git")));
        let names: Vec<String> = files.iter().map(|f| f.to_string_lossy().replace('\\', "/")).collect();
        assert!(names.iter().any(|n| n == "visible.txt"));
    }

    #[test]
    fn test_scan_skips_target() {
        let tmp = make_tmp();
        fs::create_dir(tmp.path().join("target")).unwrap();
        fs::write(tmp.path().join("target").join("debug"), "content").unwrap();
        fs::write(tmp.path().join("visible.txt"), "content").unwrap();

        let scanner = FileSystemScanner;
        let files = scanner.scan_files(tmp.path());
        assert!(!files.iter().any(|f| f.to_string_lossy().contains("target")));
    }

    #[test]
    fn test_scan_skips_node_modules() {
        let tmp = make_tmp();
        fs::create_dir(tmp.path().join("node_modules")).unwrap();
        fs::write(tmp.path().join("node_modules").join("pkg.json"), "{}").unwrap();
        fs::write(tmp.path().join("visible.txt"), "content").unwrap();

        let scanner = FileSystemScanner;
        let files = scanner.scan_files(tmp.path());
        assert!(!files.iter().any(|f| f.to_string_lossy().contains("node_modules")));
    }

    #[test]
    fn test_scan_empty_dir() {
        let tmp = make_tmp();
        let scanner = FileSystemScanner;
        let files = scanner.scan_files(tmp.path());
        assert!(files.is_empty());
    }

    #[test]
    fn test_file_exists_true() {
        let tmp = make_tmp();
        fs::write(tmp.path().join("README.md"), "hi").unwrap();
        let scanner = FileSystemScanner;
        assert!(scanner.file_exists(tmp.path(), "README.md"));
    }

    #[test]
    fn test_file_exists_false() {
        let tmp = make_tmp();
        let scanner = FileSystemScanner;
        assert!(!scanner.file_exists(tmp.path(), "README.md"));
    }

    #[test]
    fn test_read_file_success() {
        let tmp = make_tmp();
        let path = tmp.path().join("test.txt");
        fs::write(&path, "hello world").unwrap();
        let scanner = FileSystemScanner;
        let content = scanner.read_file(&path).unwrap();
        assert_eq!(content, "hello world");
    }

    #[test]
    fn test_read_file_not_found() {
        let tmp = make_tmp();
        let scanner = FileSystemScanner;
        assert!(scanner.read_file(&tmp.path().join("nope.txt")).is_err());
    }
}
