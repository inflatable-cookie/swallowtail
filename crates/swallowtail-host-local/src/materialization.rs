use crate::host::LocalProcessHostBuilder;
use crate::output::failure;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use swallowtail_runtime::{
    CleanupOutcome, MaterializedFileRef, RuntimeFailure, ScopeId, WorkingResourceRef,
};

impl LocalProcessHostBuilder {
    /// Replaces the root used for operation-scoped temporary materialization.
    #[must_use]
    pub fn with_temporary_root(mut self, path: impl Into<PathBuf>) -> Self {
        self.temporary_root = path.into();
        self
    }
}

#[derive(Clone)]
struct ScopedPath {
    scope: ScopeId,
    materialized_path: PathBuf,
    cleanup_path: PathBuf,
}

pub(crate) struct LocalMaterializationState {
    temporary_root: PathBuf,
    sequence: AtomicU64,
    working_resources: Mutex<HashMap<WorkingResourceRef, ScopedPath>>,
    files: Mutex<HashMap<MaterializedFileRef, ScopedPath>>,
}

impl LocalMaterializationState {
    pub(crate) fn new(temporary_root: PathBuf) -> Self {
        Self {
            temporary_root,
            sequence: AtomicU64::new(0),
            working_resources: Mutex::new(HashMap::new()),
            files: Mutex::new(HashMap::new()),
        }
    }

    pub(crate) fn working_resource_path(
        &self,
        scope: &ScopeId,
        reference: &WorkingResourceRef,
    ) -> Option<PathBuf> {
        let resources = self.working_resources.lock().ok()?;
        let entry = resources.get(reference)?;
        (entry.scope == *scope).then(|| entry.materialized_path.clone())
    }

    pub(crate) fn create_directory(&self, kind: &str) -> Result<PathBuf, RuntimeFailure> {
        fs::create_dir_all(&self.temporary_root).map_err(|_| {
            failure(
                "swallowtail.local_materialization.root_unavailable",
                "Local materialization root is unavailable",
            )
        })?;
        for _ in 0..32 {
            let sequence = self.sequence.fetch_add(1, Ordering::Relaxed);
            let directory = self.temporary_root.join(format!(
                "swallowtail-{kind}-{}-{sequence}",
                std::process::id()
            ));
            match mkdir_private(&directory) {
                Ok(()) => return Ok(directory),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(error) if error.kind() == std::io::ErrorKind::Unsupported => {
                    return Err(privacy_unsupported());
                }
                Err(_) => {
                    return Err(failure(
                        "swallowtail.local_materialization.create_failed",
                        "Local temporary materialization could not be created",
                    ));
                }
            }
        }
        Err(failure(
            "swallowtail.local_materialization.create_failed",
            "Local temporary materialization could not be created",
        ))
    }

    pub(crate) fn insert_working_resource(
        &self,
        scope: ScopeId,
        path: PathBuf,
    ) -> Result<WorkingResourceRef, RuntimeFailure> {
        let sequence = self.sequence.fetch_add(1, Ordering::Relaxed);
        let reference = WorkingResourceRef::new(format!(
            "local.temporary-resource.{}.{}",
            std::process::id(),
            sequence
        ))
        .map_err(|_| invalid_local_reference())?;
        self.working_resources
            .lock()
            .map_err(|_| materialization_state_failed())?
            .insert(
                reference.clone(),
                ScopedPath {
                    scope,
                    materialized_path: path.clone(),
                    cleanup_path: path,
                },
            );
        Ok(reference)
    }

    pub(crate) fn insert_file(
        &self,
        scope: ScopeId,
        path: PathBuf,
        cleanup_path: PathBuf,
    ) -> Result<MaterializedFileRef, RuntimeFailure> {
        let value = path.to_str().ok_or_else(|| {
            failure(
                "swallowtail.local_materialization.path_unrepresentable",
                "Local materialized path cannot be represented for the driver",
            )
        })?;
        let reference = MaterializedFileRef::new(value).map_err(|_| invalid_local_reference())?;
        self.files
            .lock()
            .map_err(|_| materialization_state_failed())?
            .insert(
                reference.clone(),
                ScopedPath {
                    scope,
                    materialized_path: path,
                    cleanup_path,
                },
            );
        Ok(reference)
    }

    pub(crate) fn release_working_resource(
        &self,
        scope: &ScopeId,
        reference: &WorkingResourceRef,
    ) -> CleanupOutcome {
        let result = (|| {
            let mut resources = self
                .working_resources
                .lock()
                .map_err(|_| materialization_state_failed())?;
            let entry = resources.get(reference).ok_or_else(lease_not_owned)?;
            if entry.scope != *scope {
                return Err(lease_scope_mismatch());
            }
            remove_path(&entry.cleanup_path)?;
            resources.remove(reference);
            Ok(())
        })();
        cleanup_outcome(result)
    }

    pub(crate) fn release_file(
        &self,
        scope: &ScopeId,
        reference: &MaterializedFileRef,
    ) -> CleanupOutcome {
        let result = (|| {
            let mut files = self
                .files
                .lock()
                .map_err(|_| materialization_state_failed())?;
            let entry = files.get(reference).ok_or_else(lease_not_owned)?;
            if entry.scope != *scope {
                return Err(lease_scope_mismatch());
            }
            remove_path(&entry.cleanup_path)?;
            files.remove(reference);
            Ok(())
        })();
        cleanup_outcome(result)
    }
}

impl Drop for LocalMaterializationState {
    fn drop(&mut self) {
        let mut cleanup_paths = HashSet::new();
        if let Ok(resources) = self.working_resources.get_mut() {
            cleanup_paths.extend(resources.values().map(|entry| entry.cleanup_path.clone()));
        }
        if let Ok(files) = self.files.get_mut() {
            cleanup_paths.extend(files.values().map(|entry| entry.cleanup_path.clone()));
        }
        for path in cleanup_paths {
            let _ = remove_path(&path);
        }
    }
}

pub(crate) fn create_private_directory(path: &Path) -> Result<(), RuntimeFailure> {
    match mkdir_private(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::Unsupported => Err(privacy_unsupported()),
        Err(_) => Err(failure(
            "swallowtail.local_materialization.privacy_failed",
            "Local temporary directory could not be created privately",
        )),
    }
}

pub(crate) fn create_or_replace_private_file(
    path: &Path,
    content: &[u8],
) -> Result<(), RuntimeFailure> {
    match write_private_file(path, content) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::Unsupported => Err(privacy_unsupported()),
        Err(_) => Err(failure(
            "swallowtail.local_materialization.privacy_failed",
            "Local temporary file could not be created privately",
        )),
    }
}

fn mkdir_private(path: &Path) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        fs::DirBuilder::new().mode(0o700).create(path)
    }
    #[cfg(not(unix))]
    {
        let _ = path;
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "private directories require Unix file-mode control",
        ))
    }
}

fn write_private_file(path: &Path, content: &[u8]) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        use std::io::Write;
        use std::os::unix::fs::OpenOptionsExt;
        for _ in 0..32 {
            match fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .open(path)
            {
                Ok(mut file) => return file.write_all(content),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                    match fs::remove_file(path) {
                        Ok(()) => {}
                        Err(remove_error)
                            if remove_error.kind() == std::io::ErrorKind::NotFound => {}
                        Err(remove_error) => return Err(remove_error),
                    }
                }
                Err(error) => return Err(error),
            }
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "private file could not be created exclusively",
        ))
    }
    #[cfg(not(unix))]
    {
        let _ = (path, content);
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "private files require Unix file-mode control",
        ))
    }
}

fn privacy_unsupported() -> RuntimeFailure {
    failure(
        "swallowtail.local_materialization.privacy_unsupported",
        "Local private materialization requires Unix file-mode control",
    )
}

fn remove_path(path: &Path) -> Result<(), RuntimeFailure> {
    let result = if path.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    };
    result.map_err(|_| {
        failure(
            "swallowtail.local_materialization.cleanup_failed",
            "Local temporary materialization could not be removed",
        )
    })
}

fn cleanup_outcome(result: Result<(), RuntimeFailure>) -> CleanupOutcome {
    match result {
        Ok(()) => CleanupOutcome::Clean,
        Err(error) => CleanupOutcome::Failed(error.diagnostic().clone()),
    }
}

fn invalid_local_reference() -> RuntimeFailure {
    failure(
        "swallowtail.local_materialization.reference_failed",
        "Local materialization reference could not be created",
    )
}

fn materialization_state_failed() -> RuntimeFailure {
    failure(
        "swallowtail.local_materialization.state_failed",
        "Local materialization state is unavailable",
    )
}

fn lease_not_owned() -> RuntimeFailure {
    failure(
        "swallowtail.local_materialization.lease_not_owned",
        "Local materialization lease is not owned by this host",
    )
}

fn lease_scope_mismatch() -> RuntimeFailure {
    failure(
        "swallowtail.local_materialization.scope_mismatch",
        "Local materialization lease belongs to a different operation scope",
    )
}

#[cfg(all(test, unix))]
mod tests {
    use super::{create_or_replace_private_file, create_private_directory};
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::sync::atomic::{AtomicU64, Ordering};

    static SEQUENCE: AtomicU64 = AtomicU64::new(1);

    fn fixture_path(label: &str) -> std::path::PathBuf {
        let sequence = SEQUENCE.fetch_add(1, Ordering::SeqCst);
        std::env::temp_dir().join(format!(
            "swallowtail-private-create-{label}-{}-{sequence}",
            std::process::id()
        ))
    }

    #[test]
    fn private_helpers_create_directories_and_files_with_exact_modes() {
        let directory = fixture_path("dir");
        let file = directory.join("mcp.json");
        create_private_directory(&directory).expect("private directory is created");
        create_or_replace_private_file(&file, b"secret-bearer").expect("private file is created");
        let dir_mode = fs::metadata(&directory)
            .expect("directory metadata")
            .permissions()
            .mode()
            & 0o777;
        let file_mode = fs::metadata(&file)
            .expect("file metadata")
            .permissions()
            .mode()
            & 0o777;
        let _ = fs::remove_dir_all(&directory);
        assert_eq!(dir_mode, 0o700);
        assert_eq!(file_mode, 0o600);
    }

    #[test]
    fn private_file_create_does_not_write_into_a_preexisting_shared_inode() {
        use std::io::Read;
        let parent = fixture_path("shared-parent");
        fs::create_dir(&parent).expect("shared parent is created");
        fs::set_permissions(&parent, fs::Permissions::from_mode(0o777))
            .expect("shared parent is world-accessible");
        let file = parent.join("mcp.json");
        fs::write(&file, b"attacker-held").expect("preexisting file is planted");
        fs::set_permissions(&file, fs::Permissions::from_mode(0o666))
            .expect("preexisting file is world-readable");
        let mut held = fs::File::open(&file).expect("attacker keeps an open descriptor");
        create_or_replace_private_file(&file, b"secret-bearer")
            .expect("exclusive private create replaces the name");
        let file_mode = fs::metadata(&file)
            .expect("replaced file metadata")
            .permissions()
            .mode()
            & 0o777;
        let new_content = fs::read(&file).expect("replaced file content");
        let mut held_content = Vec::new();
        held.read_to_end(&mut held_content)
            .expect("held descriptor is still readable");
        let _ = fs::remove_dir_all(&parent);
        assert_eq!(file_mode, 0o600);
        assert_eq!(new_content, b"secret-bearer");
        assert_eq!(held_content, b"attacker-held");
    }

    #[test]
    fn private_directory_create_does_not_adopt_a_preexisting_shared_directory() {
        let directory = fixture_path("shared-dir");
        fs::create_dir(&directory).expect("shared directory is planted");
        fs::set_permissions(&directory, fs::Permissions::from_mode(0o777))
            .expect("shared directory is world-accessible");
        let error = create_private_directory(&directory)
            .expect_err("private mkdir must not adopt an existing directory");
        let dir_mode = fs::metadata(&directory)
            .expect("planted directory metadata")
            .permissions()
            .mode()
            & 0o777;
        let _ = fs::remove_dir_all(&directory);
        assert_eq!(dir_mode, 0o777);
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.local_materialization.privacy_failed"
        );
    }
}
