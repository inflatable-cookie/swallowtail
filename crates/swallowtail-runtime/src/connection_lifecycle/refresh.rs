use swallowtail_core::{AccessStatus, AdmittedInstanceRecord, ConfiguredInstanceId};

use super::{ConnectionLifecycleStore, ReadinessRefreshFailure};

/// Host-supplied access dimensions for one admitted instance.
///
/// Refresh writes [`AccessStatus`] onto the stored record. It does not write
/// enablement or invent an aggregate ready boolean.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadinessRefreshRequest {
    instance_id: ConfiguredInstanceId,
    access_status: AccessStatus,
}

impl ReadinessRefreshRequest {
    /// Creates a refresh request for one admitted instance.
    #[must_use]
    pub const fn new(instance_id: ConfiguredInstanceId, access_status: AccessStatus) -> Self {
        Self {
            instance_id,
            access_status,
        }
    }

    #[must_use]
    /// Returns the configured-instance identity to refresh.
    pub const fn instance_id(&self) -> &ConfiguredInstanceId {
        &self.instance_id
    }

    #[must_use]
    /// Returns the observed Contract 006 / 008 access dimensions.
    pub const fn access_status(&self) -> &AccessStatus {
        &self.access_status
    }
}

/// Re-observes access dimensions for one admitted instance.
///
/// Enablement is left unchanged. Unrelated instances are not probed. Contract
/// 047 snapshots are not written or mutated.
pub fn refresh_readiness(
    store: &dyn ConnectionLifecycleStore,
    request: ReadinessRefreshRequest,
) -> Result<AdmittedInstanceRecord, ReadinessRefreshFailure> {
    let record = store
        .get_instance(request.instance_id())
        .map_err(ReadinessRefreshFailure::from_store)?
        .ok_or_else(ReadinessRefreshFailure::instance_absent)?;
    let refreshed = record.with_access_status(request.access_status);
    store
        .put_instance(refreshed.clone())
        .map_err(ReadinessRefreshFailure::from_store)?;
    Ok(refreshed)
}
