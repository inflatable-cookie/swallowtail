#![allow(unused_mut)]
#![deny(missing_docs)]

include!("roles/imports.rs");
include!("roles/drivers/provider_run_reconciliation.rs");
include!("roles/drivers/provider_recovered_resource_cleanup.rs");

include!("roles/api/basic.rs");
include!("roles/api/sessions.rs");
include!("roles/api/turn_and_serving.rs");

include!("roles/requests/basic.rs");
include!("roles/requests/sessions.rs");
include!("roles/requests/turn_and_serving.rs");

include!("roles/drivers/discovery.rs");
include!("roles/drivers/catalogue.rs");
include!("roles/drivers/structured_run.rs");
include!("roles/drivers/interactive_session.rs");
include!("roles/drivers/realtime.rs");
include!("roles/drivers/provider_session_management.rs");
include!("roles/drivers/provider_session_catalogue.rs");
include!("roles/drivers/provider_session_import.rs");
include!("roles/drivers/provider_session_reconciliation.rs");
include!("roles/drivers/provider_session_history.rs");
include!("roles/drivers/serving.rs");

/// Discovers provider instances without starting an inference operation.
pub trait DiscoveryDriver: Send + Sync {
    discovery_driver_items!();
}

/// Lists models available through one prepared provider instance.
pub trait ModelCatalogDriver: Send + Sync {
    catalogue_driver_items!();
}

/// Starts one bounded provider run from an admitted preflight plan.
pub trait StructuredRunDriver: Send + Sync {
    structured_run_driver_items!();
}

/// Opens, resumes, loads, or recovers reusable provider sessions.
pub trait InteractiveSessionDriver: Send + Sync {
    interactive_session_driver_items!();
}

/// Opens one realtime duplex media session.
pub trait RealtimeMediaSessionDriver: Send + Sync {
    realtime_driver_items!();
}

/// Low-level role for one explicitly bound inactive provider session.
///
/// Implementations must finish all scoped work and preserve uncertain
/// after-dispatch truth before resolving the returned future.
pub trait ProviderSessionManagementDriver: Send + Sync {
    provider_session_management_driver_items!();
}

/// Read-only discovery of provider-owned sessions within one prepared scope.
pub trait ProviderSessionCatalogueDriver: Send + Sync {
    provider_session_catalogue_driver_items!();
}

/// Read-only revalidation and binding issue for one explicitly selected session.
pub trait ProviderSessionImportDriver: Send + Sync {
    provider_session_import_driver_items!();
}

/// Read-only observation of provider work left attached to a durable session.
///
/// This role grants no cancellation, callback, continuation, or session-import
/// authority.
pub trait ProviderSessionReconciliationDriver: Send + Sync {
    provider_session_reconciliation_driver_items!();
}

/// Read-only newest-first pages of provider-owned session history.
///
/// This role grants no turn start, resume, load, import, archive, delete, or
/// callback authority, and returns no live session handle.
pub trait ProviderSessionHistoryDriver: Send + Sync {
    provider_session_history_driver_items!();
}

/// Read-only observation of one exact provider-owned structured run.
///
/// This role grants no create, retry, stream attachment, cancellation,
/// callback, deletion, or provider-session authority.
pub trait ProviderRunReconciliationDriver: Send + Sync {
    provider_run_reconciliation_driver_items!();
}

/// Destructive cleanup of exact inactive resources from one recovered run.
///
/// This role accepts only the separately persisted cleanup binding. It grants
/// no interruption, retry, callback, or provider-specific ordering authority.
pub trait ProviderRecoveredResourceCleanupDriver: Send + Sync {
    provider_recovered_resource_cleanup_driver_items!();
}

/// Attaches to or starts a model-serving instance.
pub trait ServingInstanceDriver: Send + Sync {
    serving_driver_items!();
}
