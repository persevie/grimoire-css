//! File change tracking and cleanup for generated CSS files.
//!
//! This module tracks which CSS files are generated during builds and cleans up
//! old files that are no longer needed. Uses a lock file to maintain the state.
//!
//! # Example
//! ```ignore
//! FileTracker::track(cwd, &[path1, path2])?; // Tracks new files and removes old ones
//! ```

use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fs, path::Path};

use super::{Filesystem, GrimoireCssError};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GrimoireLock {
    paths: Vec<String>,
}

pub struct FileTracker;

impl FileTracker {
    pub fn track<'a>(
        cwd: &Path,
        builded_files: impl IntoIterator<Item = &'a Path>,
    ) -> Result<(), GrimoireCssError> {
        let prev_lock_path =
            Filesystem::get_or_create_grimoire_path(cwd)?.join("grimoire.lock.json");

        let current_files_set: HashSet<String> = builded_files
            .into_iter()
            .map(|path| path.to_string_lossy().into_owned())
            .collect();

        if prev_lock_path.exists() {
            let content = fs::read_to_string(&prev_lock_path)?;
            let lock_json: GrimoireLock = serde_json::from_str(&content)?;

            let prev_files_set: HashSet<String> = lock_json.paths.into_iter().collect();
            let files_to_delete = prev_files_set.difference(&current_files_set);

            for file in files_to_delete {
                let file_path = cwd.join(file);
                if file_path.exists() {
                    fs::remove_file(&file_path)?;
                } else {
                    eprintln!("Warning: File {file} does not exist and cannot be deleted.");
                }
            }
        }

        let lock = GrimoireLock {
            paths: current_files_set.into_iter().collect(),
        };
        let new_lock_content = serde_json::to_string_pretty(&lock)?;
        fs::write(&prev_lock_path, new_lock_content)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::tempdir;

    #[test]
    fn test_track_creates_lock_file() {
        let temp_dir = tempdir().unwrap();
        let cwd = temp_dir.path();

        let file1 = cwd.join("file1.css");
        let file2 = cwd.join("file2.css");
        File::create(&file1).unwrap();
        File::create(&file2).unwrap();

        let builded_files = vec![file1.as_path(), file2.as_path()];

        FileTracker::track(cwd, builded_files).expect("Failed to track files");

        let lock_file_path = cwd.join("grimoire/grimoire.lock.json");
        assert!(
            lock_file_path.exists(),
            "Lock file was not created at expected path: {lock_file_path:?}"
        );
    }

    #[test]
    fn test_track_removes_old_files() {
        let temp_dir = tempdir().unwrap();
        let cwd = temp_dir.path();

        let old_file1 = cwd.join("old_file1.css");
        let old_file2 = cwd.join("old_file2.css");
        File::create(&old_file1).unwrap();
        File::create(&old_file2).unwrap();

        let lock = GrimoireLock {
            paths: vec!["old_file1.css".to_string(), "old_file2.css".to_string()],
        };
        let lock_file_path = cwd.join("grimoire/grimoire.lock.json");
        fs::create_dir_all(lock_file_path.parent().unwrap()).unwrap();
        fs::write(
            &lock_file_path,
            serde_json::to_string_pretty(&lock).unwrap(),
        )
        .unwrap();

        let new_file = cwd.join("new_file.css");
        File::create(&new_file).unwrap();
        let builded_files = vec![new_file.as_path()];

        FileTracker::track(cwd, builded_files).expect("Failed to track files");

        assert!(
            !old_file1.exists(),
            "Old file was not deleted: {old_file1:?}"
        );
        assert!(
            !old_file2.exists(),
            "Old file was not deleted: {old_file2:?}"
        );

        assert!(
            new_file.exists(),
            "New file was unexpectedly deleted: {new_file:?}"
        );
    }

    #[test]
    fn test_track_handles_missing_lock_file_gracefully() {
        let temp_dir = tempdir().unwrap();
        let cwd = temp_dir.path();

        let file1 = cwd.join("file1.css");
        File::create(&file1).unwrap();
        let builded_files = vec![file1.as_path()];

        FileTracker::track(cwd, builded_files).expect("Failed to track files");

        let lock_file_path = cwd.join("grimoire/grimoire.lock.json");
        assert!(
            lock_file_path.exists(),
            "Lock file was not created at expected path: {lock_file_path:?}"
        );
    }
}
