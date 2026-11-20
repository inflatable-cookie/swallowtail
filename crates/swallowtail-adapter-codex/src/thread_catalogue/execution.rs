#[path = "execution/catalogue.rs"]
mod catalogue;
#[path = "execution/history.rs"]
mod history;
#[path = "execution/import.rs"]
mod import;
#[path = "execution/reconciliation.rs"]
mod reconciliation;
#[path = "execution/resource.rs"]
mod resource;

use crate::app_server::close_connection;
use crate::control_wait::{Controlled, wait_controlled};
use crate::process_close::{AppServerTask, CloseBudget};
use crate::rpc::RpcConnection;
use crate::selection::supports_thread_catalogue_version;
use resource::ScopedResource;
use serde_json::Value;
use std::sync::Arc;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, Deadline, DeadlineObservation, HostServices, ImmediateCancellation,
    ProviderSessionOperationFailure, ProviderSessionOperationFailureStage, RuntimeFailure,
};

pub(super) fn require_catalogue_version(
    plan: &swallowtail_core::PreflightPlan,
) -> Result<(), ProviderSessionOperationFailure> {
    let version = crate::cli_version::bound_codex_cli_version(plan).ok_or_else(|| {
        control_failure(
            ProviderSessionOperationFailureStage::BeforeDispatch,
            "swallowtail.codex.thread_catalogue.version_missing",
            "Codex thread catalogue requires an exact executable version",
        )
    })?;
    if supports_thread_catalogue_version(version.version()) {
        Ok(())
    } else {
        Err(control_failure(
            ProviderSessionOperationFailureStage::BeforeDispatch,
            "swallowtail.codex.thread_catalogue.version_unsupported",
            "Codex executable version does not support the qualified thread catalogue",
        ))
    }
}

pub(super) fn control_before_dispatch(
    cancellation: &swallowtail_runtime::ImmediateCancellation,
    deadline: Option<Deadline>,
    services: &HostServices,
) -> Result<(), ProviderSessionOperationFailure> {
    if cancellation.is_requested() {
        return Err(control_failure(
            ProviderSessionOperationFailureStage::Cancelled,
            "swallowtail.codex.thread_catalogue.cancelled",
            "Codex provider-session operation was cancelled",
        ));
    }
    if deadline.is_some_and(|deadline| {
        services
            .time()
            .is_some_and(|time| time.now() >= deadline.instant())
    }) {
        return Err(control_failure(
            ProviderSessionOperationFailureStage::TimedOut,
            "swallowtail.codex.thread_catalogue.timed_out",
            "Codex provider-session operation timed out",
        ));
    }
    Ok(())
}

pub(super) fn deadline_wait(
    deadline: Option<Deadline>,
    services: &HostServices,
) -> Result<Option<BoxFuture<'static, DeadlineObservation>>, ProviderSessionOperationFailure> {
    deadline
        .map(|deadline| {
            services
                .time()
                .ok_or_else(|| {
                    control_failure(
                        ProviderSessionOperationFailureStage::BeforeDispatch,
                        "swallowtail.codex.thread_catalogue.time_service_missing",
                        "Deadline-bound Codex provider-session operation requires a time service",
                    )
                })
                .map(|time| time.wait_until(deadline))
        })
        .transpose()
}

/// Sends one RPC and waits for its response, honouring cancellation and the
/// caller's deadline. A failed dispatch is reported as a completed error.
pub(super) async fn request_controlled(
    connection: &RpcConnection,
    method: &str,
    params: Value,
    cancellation: &ImmediateCancellation,
    deadline: Option<BoxFuture<'static, DeadlineObservation>>,
) -> Controlled<Result<Value, RuntimeFailure>> {
    match connection.dispatch_request(method, params).await {
        Ok(response) => wait_controlled(response, cancellation, deadline).await,
        Err(error) => Controlled::Completed(Err(error)),
    }
}

/// The one teardown every RPC operation shares: an interrupted run cancels,
/// a completed run closes within its budget, and the lease is released last.
async fn settle(
    connection: &Arc<RpcConnection>,
    task: AppServerTask,
    resource: ScopedResource,
    interrupted: bool,
    budget: CloseBudget,
) -> CleanupOutcome {
    if interrupted {
        cancel_and_release(connection, task, resource).await
    } else {
        close_and_release(connection, task, resource, budget).await
    }
}

async fn close_and_release(
    connection: &Arc<RpcConnection>,
    task: AppServerTask,
    resource: ScopedResource,
    budget: CloseBudget,
) -> CleanupOutcome {
    let connection = close_connection(connection, task, budget).await;
    CleanupOutcome::merge(connection, resource.release().await)
}

async fn cancel_and_release(
    connection: &Arc<RpcConnection>,
    task: AppServerTask,
    resource: ScopedResource,
) -> CleanupOutcome {
    let stop = connection.cancel_session().await;
    let join = task.join().await;
    let connection_cleanup = if stop.is_err() || join.is_err() {
        CleanupOutcome::Failed(SafeDiagnostic::new(
            "swallowtail.codex.thread_catalogue.cancel_cleanup_failed",
            "Codex provider-session cancellation cleanup failed",
        ))
    } else {
        connection.cleanup_outcome()
    };
    CleanupOutcome::merge(connection_cleanup, resource.release().await)
}

pub(super) fn finish<T>(
    result: Result<T, ProviderSessionOperationFailure>,
    cleanup: CleanupOutcome,
) -> Result<T, ProviderSessionOperationFailure> {
    match cleanup {
        CleanupOutcome::Clean | CleanupOutcome::NotApplicable => result,
        CleanupOutcome::Degraded(diagnostic) | CleanupOutcome::Failed(diagnostic) => {
            Err(ProviderSessionOperationFailure::new(
                ProviderSessionOperationFailureStage::Cleanup,
                diagnostic,
            ))
        }
    }
}

pub(super) fn cleanup_or(
    cleanup: CleanupOutcome,
    otherwise: ProviderSessionOperationFailure,
) -> ProviderSessionOperationFailure {
    finish::<()>(Err(otherwise.clone()), cleanup).unwrap_err()
}

/// Runtime-error twin of [`finish`]: a cleanup failure or degradation replaces
/// the projected result, even when that result is itself an error.
pub(super) fn finish_runtime<T>(
    result: Result<T, RuntimeFailure>,
    cleanup: &CleanupOutcome,
) -> Result<T, RuntimeFailure> {
    match cleanup {
        CleanupOutcome::Clean | CleanupOutcome::NotApplicable => result,
        CleanupOutcome::Degraded(diagnostic) | CleanupOutcome::Failed(diagnostic) => {
            Err(RuntimeFailure::new(diagnostic.clone()))
        }
    }
}

pub(super) fn cleanup_or_runtime(
    cleanup: CleanupOutcome,
    otherwise: RuntimeFailure,
) -> RuntimeFailure {
    finish_runtime::<()>(Err(otherwise), &cleanup).unwrap_err()
}

/// Releases the lease after a failure before any connection exists.
async fn release_runtime(resource: ScopedResource, error: RuntimeFailure) -> RuntimeFailure {
    cleanup_or_runtime(resource.release().await, error)
}

/// Cancels and releases after a failure once a connection exists.
async fn abort_runtime(
    connection: &Arc<RpcConnection>,
    task: AppServerTask,
    resource: ScopedResource,
    error: RuntimeFailure,
) -> RuntimeFailure {
    cleanup_or_runtime(cancel_and_release(connection, task, resource).await, error)
}

pub(super) fn from_runtime(
    stage: ProviderSessionOperationFailureStage,
    error: RuntimeFailure,
) -> ProviderSessionOperationFailure {
    ProviderSessionOperationFailure::new(stage, error.diagnostic().clone())
}

pub(super) fn runtime_from_operation(error: ProviderSessionOperationFailure) -> RuntimeFailure {
    RuntimeFailure::new(error.diagnostic().clone())
}

pub(super) fn control_failure(
    stage: ProviderSessionOperationFailureStage,
    code: &'static str,
    message: &'static str,
) -> ProviderSessionOperationFailure {
    ProviderSessionOperationFailure::new(stage, SafeDiagnostic::new(code, message))
}

#[cfg(test)]
mod tests {
    use super::{cleanup_or_runtime, finish_runtime};
    use swallowtail_core::SafeDiagnostic;
    use swallowtail_runtime::{CleanupOutcome, RuntimeFailure};

    fn runtime_failure(code: &'static str) -> RuntimeFailure {
        RuntimeFailure::new(SafeDiagnostic::new(code, "message"))
    }

    fn cleanup_failed() -> CleanupOutcome {
        CleanupOutcome::Failed(SafeDiagnostic::new("cleanup.failed", "cleanup failed"))
    }

    #[test]
    fn failed_cleanup_replaces_a_projected_error() {
        let error =
            finish_runtime::<()>(Err(runtime_failure("projection.failed")), &cleanup_failed())
                .unwrap_err();
        assert_eq!(error.diagnostic().code(), "cleanup.failed");
    }

    #[test]
    fn degraded_cleanup_replaces_a_projected_error() {
        let cleanup = CleanupOutcome::Degraded(SafeDiagnostic::new("cleanup.degraded", "degraded"));
        let error =
            finish_runtime::<()>(Err(runtime_failure("projection.failed")), &cleanup).unwrap_err();
        assert_eq!(error.diagnostic().code(), "cleanup.degraded");
    }

    #[test]
    fn degraded_cleanup_replaces_a_projected_success() {
        let cleanup = CleanupOutcome::Degraded(SafeDiagnostic::new("cleanup.degraded", "degraded"));
        let error = finish_runtime(Ok(7_u8), &cleanup).unwrap_err();
        assert_eq!(error.diagnostic().code(), "cleanup.degraded");
    }

    #[test]
    fn clean_cleanup_keeps_the_projected_result() {
        let error = finish_runtime::<()>(
            Err(runtime_failure("projection.failed")),
            &CleanupOutcome::Clean,
        )
        .unwrap_err();
        assert_eq!(error.diagnostic().code(), "projection.failed");
        assert_eq!(
            finish_runtime(Ok(7_u8), &CleanupOutcome::NotApplicable).ok(),
            Some(7)
        );
    }

    #[test]
    fn pre_dispatch_error_is_replaced_by_failed_cleanup() {
        let error = cleanup_or_runtime(cleanup_failed(), runtime_failure("start.failed"));
        assert_eq!(error.diagnostic().code(), "cleanup.failed");
    }
}
