//! The lifecycle every controlled thread RPC shares.
//!
//! Catalogue, history, import, and reconciliation each validate their plan,
//! take a working-resource lease, then run one RPC on a fresh app-server
//! connection under cancellation and a deadline. Only the codes they report,
//! the stage a dispatch failure belongs to, and how a response projects differ
//! between them, so that lifecycle lives here once.

use super::resource::ScopedResource;
use super::{
    cancel_and_release, cleanup_or, control_before_dispatch, control_failure, deadline_wait,
    from_runtime, request_controlled, settle,
};
use crate::CodexAppServerDriver;
use crate::app_server::{INITIALIZE_TIMED_OUT, InitBound, InitControl, initialize_control};
use crate::control_wait::Controlled;
use crate::process_close::close_budget;
use serde_json::Value;
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    CleanupOutcome, Deadline, HostServices, ImmediateCancellation, ProviderSessionOperationFailure,
    ProviderSessionOperationFailureStage, ScopeId,
};

/// The diagnostics one thread operation reports for its controls and dispatch.
#[derive(Clone, Copy)]
pub(super) struct Operation {
    cancelled: (&'static str, &'static str),
    timed_out: (&'static str, &'static str),
    dispatch_stage: ProviderSessionOperationFailureStage,
}

impl Operation {
    pub(super) const fn new(
        cancelled: (&'static str, &'static str),
        timed_out: (&'static str, &'static str),
        dispatch_stage: ProviderSessionOperationFailureStage,
    ) -> Self {
        Self {
            cancelled,
            timed_out,
            dispatch_stage,
        }
    }

    pub(super) fn cancelled(self) -> ProviderSessionOperationFailure {
        control_failure(
            ProviderSessionOperationFailureStage::Cancelled,
            self.cancelled.0,
            self.cancelled.1,
        )
    }

    pub(super) fn timed_out(self) -> ProviderSessionOperationFailure {
        control_failure(
            ProviderSessionOperationFailureStage::TimedOut,
            self.timed_out.0,
            self.timed_out.1,
        )
    }

    /// Maps a runtime error from dispatch or projection to this operation's stage.
    pub(super) fn runtime(
        self,
        error: swallowtail_runtime::RuntimeFailure,
    ) -> ProviderSessionOperationFailure {
        from_runtime(self.dispatch_stage, error)
    }
}

/// Cancellation and deadline that bound one controlled RPC.
#[derive(Clone, Copy)]
pub(super) struct Controls<'a> {
    pub(super) operation: Operation,
    pub(super) cancellation: &'a ImmediateCancellation,
    pub(super) deadline: Option<Deadline>,
}

/// One completed RPC: its projected value and the cleanup that followed.
pub(super) struct Settled<T> {
    pub(super) projected: Result<T, ProviderSessionOperationFailure>,
    pub(super) cleanup: CleanupOutcome,
}

/// Starts a connection for `resource`, sends one RPC, projects its response,
/// and settles the connection and lease.
///
/// Errors before the RPC carry the cleanup already merged in, so a failed
/// release is never lost. `project` runs only on a completed success.
#[allow(clippy::too_many_arguments)]
pub(super) async fn run_controlled_rpc<T>(
    driver: &CodexAppServerDriver,
    plan: &PreflightPlan,
    behavior: crate::selection::CodexAppServerBehavior,
    operation_scope: ScopeId,
    resource: ScopedResource,
    controls: Controls<'_>,
    services: &HostServices,
    method: &str,
    params: Value,
    project: impl FnOnce(&Value) -> Result<T, ProviderSessionOperationFailure>,
) -> Result<Settled<T>, ProviderSessionOperationFailure> {
    let Controls {
        operation,
        cancellation,
        deadline,
    } = controls;
    let started = driver
        .start_connection(
            plan,
            behavior,
            operation_scope,
            Some(resource.reference().clone()),
            services,
            InitBound {
                experimental_api: false,
                cancellation: Some(cancellation),
                deadline,
                timed_out: INITIALIZE_TIMED_OUT,
            },
        )
        .await;
    let (connection, task) = match started {
        Ok(started) => started,
        Err(error) => {
            let cleanup = resource.release().await;
            let failure = match initialize_control(&error) {
                Some(InitControl::Cancelled) => operation.cancelled(),
                Some(InitControl::TimedOut) => operation.timed_out(),
                None => operation.runtime(error),
            };
            return Err(cleanup_or(cleanup, failure));
        }
    };
    if let Err(control) = control_before_dispatch(operation, cancellation, deadline, services) {
        let cleanup = cancel_and_release(&connection, task, resource).await;
        return Err(cleanup_or(cleanup, control));
    }
    let deadline_wait = match deadline_wait(deadline, services) {
        Ok(wait) => wait,
        Err(error) => {
            let cleanup = cancel_and_release(&connection, task, resource).await;
            return Err(cleanup_or(cleanup, error));
        }
    };
    let response =
        request_controlled(&connection, method, params, cancellation, deadline_wait).await;
    let interrupted = matches!(&response, Controlled::Cancelled | Controlled::Deadline);
    let projected = match response {
        Controlled::Completed(Ok(value)) => project(&value),
        Controlled::Completed(Err(error)) => Err(operation.runtime(error)),
        Controlled::Cancelled => Err(operation.cancelled()),
        Controlled::Deadline => Err(operation.timed_out()),
    };
    let cleanup = settle(
        &connection,
        task,
        resource,
        interrupted,
        close_budget(services, deadline),
    )
    .await;
    Ok(Settled { projected, cleanup })
}
