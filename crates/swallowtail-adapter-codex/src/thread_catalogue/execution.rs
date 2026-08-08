#[path = "execution/catalogue.rs"]
mod catalogue;
#[path = "execution/control.rs"]
mod control;
#[path = "execution/history.rs"]
mod history;
#[path = "execution/import.rs"]
mod import;
#[path = "execution/reconciliation.rs"]
mod reconciliation;
#[path = "execution/resource.rs"]
mod resource;

use crate::app_server::close_connection;
use crate::rpc::RpcConnection;
use crate::selection::supports_thread_catalogue_version;
use resource::ScopedResource;
use std::sync::Arc;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, Deadline, DeadlineObservation, HostServices, JoinedTask,
    ProviderSessionOperationFailure, ProviderSessionOperationFailureStage, RuntimeFailure,
};

pub(super) fn require_catalogue_version(
    plan: &swallowtail_core::PreflightPlan,
) -> Result<(), ProviderSessionOperationFailure> {
    let version = plan.interface_versions().next().ok_or_else(|| {
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

async fn close_and_release(
    connection: &Arc<RpcConnection>,
    task: Box<dyn JoinedTask>,
    resource: ScopedResource,
) -> CleanupOutcome {
    let connection = close_connection(connection, task).await;
    merge_cleanup(connection, resource.release().await)
}

async fn cancel_and_release(
    connection: &Arc<RpcConnection>,
    task: Box<dyn JoinedTask>,
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
    merge_cleanup(connection_cleanup, resource.release().await)
}

pub(super) fn merge_cleanup(current: CleanupOutcome, next: CleanupOutcome) -> CleanupOutcome {
    match (&current, &next) {
        (CleanupOutcome::Failed(_), _) => current,
        (_, CleanupOutcome::Failed(_)) => next,
        (CleanupOutcome::Degraded(_), _) => current,
        (_, CleanupOutcome::Degraded(_)) => next,
        (CleanupOutcome::Clean, CleanupOutcome::NotApplicable) => current,
        (CleanupOutcome::NotApplicable, CleanupOutcome::Clean) => next,
        _ => current,
    }
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
