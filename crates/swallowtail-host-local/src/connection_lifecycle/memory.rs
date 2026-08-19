use super::state::StoreState;
use std::fmt;
use std::sync::Mutex;
use swallowtail_core::{AdmittedInstanceRecord, ConfiguredInstanceId, OverlayMarker};
use swallowtail_runtime::{ConnectionLifecycleStore, ConnectionLifecycleStoreFailure};

/// In-memory Contract 057 store for tests and small apps.
#[derive(Default)]
pub struct MemoryConnectionLifecycleStore {
    state: Mutex<StoreState>,
}

impl MemoryConnectionLifecycleStore {
    /// Creates an empty in-memory store.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl ConnectionLifecycleStore for MemoryConnectionLifecycleStore {
    fn put_instance(
        &self,
        record: AdmittedInstanceRecord,
    ) -> Result<(), ConnectionLifecycleStoreFailure> {
        self.state
            .lock()
            .expect("memory store lock poisoned")
            .put_instance(record);
        Ok(())
    }

    fn get_instance(
        &self,
        id: &ConfiguredInstanceId,
    ) -> Result<Option<AdmittedInstanceRecord>, ConnectionLifecycleStoreFailure> {
        Ok(self
            .state
            .lock()
            .expect("memory store lock poisoned")
            .get_instance(id))
    }

    fn list_instances(
        &self,
    ) -> Result<Vec<AdmittedInstanceRecord>, ConnectionLifecycleStoreFailure> {
        Ok(self
            .state
            .lock()
            .expect("memory store lock poisoned")
            .list_instances())
    }

    fn put_overlay_marker(
        &self,
        marker: OverlayMarker,
    ) -> Result<(), ConnectionLifecycleStoreFailure> {
        self.state
            .lock()
            .expect("memory store lock poisoned")
            .put_overlay_marker(marker);
        Ok(())
    }

    fn list_overlay_markers(&self) -> Result<Vec<OverlayMarker>, ConnectionLifecycleStoreFailure> {
        Ok(self
            .state
            .lock()
            .expect("memory store lock poisoned")
            .list_overlay_markers())
    }
}

impl fmt::Debug for MemoryConnectionLifecycleStore {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MemoryConnectionLifecycleStore")
            .finish_non_exhaustive()
    }
}
