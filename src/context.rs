use std::path::{Path, PathBuf};

use crate::{
    cli::Cli,
    config::Config,
    diagnostics::Level,
    error::{Error, Result},
};

use cargo_metadata::Metadata;
use log::debug;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rustc_hash::FxHashSet;

#[derive(Debug)]
pub struct Context {
    pub workspace_root: PathBuf,
    pub workspace_packages: Vec<String>,
    pub level: Level,
    pub files: FxHashSet<PathBuf>,
    pub clippy_args: Vec<String>,
}

impl Context {
    pub fn new(cli: &Cli) -> Result<Self> {
        let metadata = Self::get_metadata()?;

        let workspace_root =
            dunce::canonicalize(&metadata.workspace_root).map_err(|e| Error::PathResolution {
                path: metadata.workspace_root.clone().into_std_path_buf().display().to_string(),
                source: e,
            })?;
        debug!("workspace root: `{}`", workspace_root.display());

        let config = Config::new(cli, &workspace_root)?;

        let files = Self::resolve_files(&cli.files, &workspace_root)?;
        debug!("resolved `{}` files", files.len());

        let workspace_packages = (metadata.workspace_members.len() > 1 && !files.is_empty())
            .then(|| Self::resolve_packages(&files, &metadata))
            .transpose()?
            .unwrap_or_default();
        debug!("resolved `{}` workspace packages", workspace_packages.len());

        Ok(Self {
            workspace_root,
            workspace_packages,
            level: config.level,
            files,
            clippy_args: config.clippy_args,
        })
    }

    fn get_metadata() -> Result<Metadata> {
        let metadata = cargo_metadata::MetadataCommand::new().no_deps().exec().map_err(Box::new)?;
        Ok(metadata)
    }

    fn resolve_files(files: &[PathBuf], workspace_root: &Path) -> Result<FxHashSet<PathBuf>> {
        files
            .into_par_iter()
            .filter_map(|file| {
                let path = workspace_root.join(file);
                match dunce::canonicalize(&path) {
                    Ok(resolved) => {
                        if resolved.starts_with(workspace_root) {
                            if !resolved.is_file() {
                                eprintln!(
                                    "warning: ignoring file '{}' - not a regular file",
                                    resolved.display()
                                );
                                return None;
                            }
                            debug!(
                                "file '{}' resolved to '{}'",
                                file.display(),
                                resolved.display()
                            );
                            Some(Ok(resolved))
                        } else {
                            let path_info = if resolved == path {
                                format!("'{}'", file.display())
                            } else {
                                format!(
                                    "'{}' - resolved to '{}'",
                                    file.display(),
                                    resolved.display()
                                )
                            };

                            eprintln!(
                                "warning: ignoring file {} - outside of workspace root '{}'",
                                path_info,
                                workspace_root.display()
                            );
                            None
                        }
                    }
                    Err(e) => Some(Err(Error::PathResolution {
                        path: path.display().to_string(),
                        source: e,
                    })),
                }
            })
            .collect()
    }

    fn resolve_packages(files: &FxHashSet<PathBuf>, metadata: &Metadata) -> Result<Vec<String>> {
        let mut packages: Vec<(String, PathBuf)> = metadata
            .workspace_packages()
            .iter()
            .filter_map(|p| {
                let manifest_dir: PathBuf = p.manifest_path.parent().map(Into::into)?;
                let path = dunce::canonicalize(&manifest_dir).map_err(|e| Error::PathResolution {
                    path: manifest_dir.display().to_string(),
                    source: e,
                });
                match path {
                    Ok(path) => Some(Ok((p.name.to_string(), path))),
                    Err(e) => Some(Err(e)),
                }
            })
            .collect::<Result<Vec<_>>>()?;

        packages.sort_unstable_by(|a, b| a.1.cmp(&b.1));

        let resolved_packages: FxHashSet<String> = files
            .par_iter()
            .filter_map(|file| {
                let idx = packages.partition_point(|(_, p)| p <= file);

                if idx > 0 {
                    let (candidate_name, candidate_path) = &packages[idx - 1];
                    if file.starts_with(candidate_path) {
                        return Some(candidate_name.clone());
                    }
                }

                None
            })
            .collect();

        // produce deterministic output
        // useful for testing and reproducibility
        let mut result: Vec<String> = resolved_packages.into_iter().collect();
        result.sort_unstable();
        Ok(result)
    }
}
