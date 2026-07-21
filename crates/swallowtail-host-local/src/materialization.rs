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
            match fs::create_dir(&directory) {
                Ok(()) => return Ok(directory),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
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
