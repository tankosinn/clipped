use std::path::{Path, PathBuf};

use rustc_hash::{FxHashMap, FxHashSet};

use crate::error::{Error, Result};

pub struct FileMatcher<'a> {
    files: &'a FxHashSet<PathBuf>,
    workspace_root: &'a Path,
    cache: FxHashMap<PathBuf, PathBuf>,
}

impl<'a> FileMatcher<'a> {
    pub fn new(files: &'a FxHashSet<PathBuf>, workspace_root: &'a Path) -> Self {
        FileMatcher { files, workspace_root, cache: FxHashMap::default() }
    }

    pub fn matches(&mut self, file: &Path) -> Result<bool> {
        let canonical_path = if let Some(resolved) = self.cache.get(file) {
            resolved.clone()
        } else {
            let full_path = self.workspace_root.join(file);
            let resolved = dunce::canonicalize(&full_path).map_err(|source| {
                Error::PathResolution { path: full_path.display().to_string(), source }
            })?;
            self.cache.insert(file.to_path_buf(), resolved.clone());
            resolved
        };

        Ok(self.files.contains(&canonical_path))
    }
}
