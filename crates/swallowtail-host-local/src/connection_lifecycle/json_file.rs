use super::document::{JsonDocument, refuse_secret_byte_fields};
use super::state::StoreState;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use swallowtail_core::{AdmittedInstanceRecord, ConfiguredInstanceId, OverlayMarker};
use swallowtail_runtime::{ConnectionLifecycleStore, ConnectionLifecycleStoreFailure};

/// JSON-file Contract 057 store. The host supplies the path.
///
/// The document persists references, enablement, labels, and overlay markers.
/// It refuses to write secret bytes.
pub struct JsonFileConnectionLifecycleStore {
    path: PathBuf,
    state: Mutex<StoreState>,
}

impl JsonFileConnectionLifecycleStore {
    /// Opens an existing document, or an empty store if the host path is absent.
    pub fn open(path: impl Into<PathBuf>) -> Result<Self, ConnectionLifecycleStoreFailure> {
        let path = path.into();
        let state = load_state(&path)?;
        Ok(Self {
            path,
            state: Mutex::new(state),
        })
    }

    #[must_use]
    /// Returns the host-owned document path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    fn persist(&self, state: &StoreState) -> Result<(), ConnectionLifecycleStoreFailure> {
        write_state(&self.path, state)
    }
}

impl ConnectionLifecycleStore for JsonFileConnectionLifecycleStore {
    fn put_instance(
        &self,
        record: AdmittedInstanceRecord,
    ) -> Result<(), ConnectionLifecycleStoreFailure> {
        let mut state = self.state.lock().expect("json store lock poisoned");
        let mut next = state.clone();
        next.put_instance(record);
        self.persist(&next)?;
        *state = next;
        Ok(())
    }

    fn get_instance(
        &self,
        id: &ConfiguredInstanceId,
    ) -> Result<Option<AdmittedInstanceRecord>, ConnectionLifecycleStoreFailure> {
        Ok(self
            .state
            .lock()
            .expect("json store lock poisoned")
            .get_instance(id))
    }

    fn list_instances(
        &self,
    ) -> Result<Vec<AdmittedInstanceRecord>, ConnectionLifecycleStoreFailure> {
        Ok(self
            .state
            .lock()
            .expect("json store lock poisoned")
            .list_instances())
    }

    fn put_overlay_marker(
        &self,
        marker: OverlayMarker,
    ) -> Result<(), ConnectionLifecycleStoreFailure> {
        let mut state = self.state.lock().expect("json store lock poisoned");
        let mut next = state.clone();
        next.put_overlay_marker(marker);
        self.persist(&next)?;
        *state = next;
        Ok(())
    }

    fn list_overlay_markers(&self) -> Result<Vec<OverlayMarker>, ConnectionLifecycleStoreFailure> {
        Ok(self
            .state
            .lock()
            .expect("json store lock poisoned")
            .list_overlay_markers())
    }
}

impl fmt::Debug for JsonFileConnectionLifecycleStore {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("JsonFileConnectionLifecycleStore")
            .field("path", &self.path)
            .finish_non_exhaustive()
    }
}

fn load_state(path: &Path) -> Result<StoreState, ConnectionLifecycleStoreFailure> {
    match fs::read_to_string(path) {
        Ok(text) if text.trim().is_empty() => Ok(StoreState::default()),
        Ok(text) => {
            let document: JsonDocument = serde_json::from_str(&text).map_err(|_| {
                ConnectionLifecycleStoreFailure::new(
                    "swallowtail.connection_lifecycle.json_invalid",
                    "JSON-file store could not parse the lifecycle document",
                )
            })?;
            document.into_state()
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(StoreState::default()),
        Err(_) => Err(ConnectionLifecycleStoreFailure::new(
            "swallowtail.connection_lifecycle.json_read_failed",
            "JSON-file store could not read the lifecycle document",
        )),
    }
}

fn write_state(path: &Path, state: &StoreState) -> Result<(), ConnectionLifecycleStoreFailure> {
    let document = JsonDocument::from_state(state);
    let value = serde_json::to_value(&document).map_err(|_| {
        ConnectionLifecycleStoreFailure::new(
            "swallowtail.connection_lifecycle.json_write_failed",
            "JSON-file store could not encode the lifecycle document",
        )
    })?;
    refuse_secret_byte_fields(&value)?;
    let text = serde_json::to_string_pretty(&value).map_err(|_| {
        ConnectionLifecycleStoreFailure::new(
            "swallowtail.connection_lifecycle.json_write_failed",
            "JSON-file store could not encode the lifecycle document",
        )
    })?;
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent).map_err(|_| {
            ConnectionLifecycleStoreFailure::new(
                "swallowtail.connection_lifecycle.json_write_failed",
                "JSON-file store could not write the lifecycle document",
            )
        })?;
    }
    let staging = staging_path(path);
    fs::write(&staging, text).map_err(|_| {
        ConnectionLifecycleStoreFailure::new(
            "swallowtail.connection_lifecycle.json_write_failed",
            "JSON-file store could not write the lifecycle document",
        )
    })?;
    fs::rename(&staging, path).map_err(|_| {
        ConnectionLifecycleStoreFailure::new(
            "swallowtail.connection_lifecycle.json_write_failed",
            "JSON-file store could not write the lifecycle document",
        )
    })
}

fn staging_path(path: &Path) -> PathBuf {
    match path.file_name().and_then(|name| name.to_str()) {
        Some(name) => path.with_file_name(format!(".{name}.tmp")),
        None => path.with_extension("tmp"),
    }
}
