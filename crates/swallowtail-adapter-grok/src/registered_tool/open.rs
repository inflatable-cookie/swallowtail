//! Session-open binding and turn-scoped lifecycle of one registered courier.
//!
//! The courier process is Grok's child: Swallowtail declares it in the ACP
//! `mcpServers` list and the provider spawns it. This route owns the bridge
//! lease only; it never holds a Swallowtail-spawned courier process handle.
//!
//! The lease is bound to exactly one ACP turn attempt, which is the Contract
//! 063 first-tranche rule of one active provider turn per server lease. Grok
//! spawns the courier from `session/new`, before any turn exists, so the
//! consumer names that turn on the binding and the route refuses to start any
//! other turn while the lease is live. Cancellation, deadline, and turn
//! terminal freeze admission before settling; the close that follows carries
//! the exact cause and its cleanup truth is never discarded.

use super::binding::GrokRegisteredToolBinding;
use super::declaration::GrokAcpMcpServerDeclaration;
use super::projection::GROK_ACP_REGISTERED_TOOL_VERSION_NOT_ADMITTED_CODE;
use crate::failure::failure;
use std::sync::Mutex;
use swallowtail_core::{InterfaceVersion, PreflightPlan, SafeDiagnostic};
use swallowtail_host_local::RegisteredToolProxyLaunch;
use swallowtail_runtime::{
    CleanupOutcome, HostServices, RegisteredToolBridgeLease, RegisteredToolCleanupCause,
    RegisteredToolFailure, RegisteredToolReadiness, RequestId, RuntimeFailure, RuntimeTurnId,
    ScopeId,
};

/// Live registered-tool lease bound to one open ACP session and one turn.
///
/// Settlement is serialized: exactly one caller closes the lease, and every
/// concurrent caller awaits that same completion instead of reading a
/// not-yet-recorded outcome. Cancellation, turn terminal, transport failure,
/// and session close all race for this, and a caller that saw
/// `NotApplicable` while a close was still joining could publish terminal or
/// release the credential beside work the host had not finished.
pub(crate) struct GrokRegisteredToolSession {
    state: Mutex<SettleState>,
    turn: RuntimeTurnId,
}

#[derive(Default)]
struct SettleState {
    lease: Option<RegisteredToolBridgeLease>,
    outcome: Option<CleanupOutcome>,
    settling: bool,
    waiters: Vec<futures_channel::oneshot::Sender<CleanupOutcome>>,
}

/// What one `settle` caller must do to obtain the exact cleanup truth.
///
/// The lease is boxed only to keep the variants comparable in size; exactly
/// one caller ever receives it.
enum SettleStep {
    Recorded(CleanupOutcome),
    Await(futures_channel::oneshot::Receiver<CleanupOutcome>),
    Close(Box<RegisteredToolBridgeLease>),
}

impl GrokRegisteredToolSession {
    fn new(lease: RegisteredToolBridgeLease, turn: RuntimeTurnId) -> Self {
        Self {
            state: Mutex::new(SettleState {
                lease: Some(lease),
                ..SettleState::default()
            }),
            turn,
        }
    }

    /// Returns the exact turn attempt this lease was opened for.
    pub(crate) fn turn(&self) -> &RuntimeTurnId {
        &self.turn
    }

    /// Reports whether the lease settled and released everything it held.
    ///
    /// A settlement still in flight is not clean, and neither is a failed one:
    /// in both cases the host may still hold the lease and work it could not
    /// join, so the operation is not over.
    pub(crate) fn settled_clean(&self) -> bool {
        let state = self.locked();
        !state.settling
            && matches!(
                state.outcome.as_ref(),
                Some(CleanupOutcome::Clean | CleanupOutcome::NotApplicable)
            )
    }

    /// Reports whether a joined cleanup attempt already failed.
    pub(crate) fn cleanup_failed(&self) -> bool {
        matches!(
            self.locked().outcome.as_ref(),
            Some(CleanupOutcome::Failed(_) | CleanupOutcome::Degraded(_))
        )
    }

    /// Freezes admission, joins issued work, and closes with its exact cause.
    ///
    /// The first caller performs the close; every other caller awaits its
    /// result and receives the same cleanup truth. A settled lease reports its
    /// retained outcome rather than closing twice.
    pub(crate) async fn settle(
        &self,
        services: &HostServices,
        cause: RegisteredToolCleanupCause,
    ) -> CleanupOutcome {
        let step = {
            let mut state = self.locked();
            if let Some(outcome) = state.outcome.clone() {
                SettleStep::Recorded(outcome)
            } else if state.settling {
                let (sender, receiver) = futures_channel::oneshot::channel();
                state.waiters.push(sender);
                SettleStep::Await(receiver)
            } else if let Some(lease) = state.lease.take() {
                state.settling = true;
                SettleStep::Close(Box::new(lease))
            } else {
                SettleStep::Recorded(CleanupOutcome::NotApplicable)
            }
        };
        match step {
            SettleStep::Recorded(outcome) => outcome,
            // A settlement whose result cannot be observed is never clean.
            SettleStep::Await(receiver) => receiver.await.unwrap_or_else(|_| {
                CleanupOutcome::Failed(SafeDiagnostic::new(
                    "swallowtail.grok.acp.registered_tool.cleanup_unobserved",
                    "Grok Build registered-tool cleanup result was not observed",
                ))
            }),
            SettleStep::Close(lease) => {
                let outcome = close_lease(*lease, services, cause).await;
                let waiters = {
                    let mut state = self.locked();
                    state.outcome = Some(outcome.clone());
                    state.settling = false;
                    std::mem::take(&mut state.waiters)
                };
                for waiter in waiters {
                    let _ = waiter.send(outcome.clone());
                }
                outcome
            }
        }
    }

    fn locked(&self) -> std::sync::MutexGuard<'_, SettleState> {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

/// Partial registered-tool open that never holds a courier process handle.
pub(crate) struct PendingRegisteredOpen {
    lease: Option<RegisteredToolBridgeLease>,
    launch: Option<RegisteredToolProxyLaunch>,
    declaration: GrokAcpMcpServerDeclaration,
    turn: RuntimeTurnId,
    settled: Option<CleanupOutcome>,
    claimed: bool,
}

impl PendingRegisteredOpen {
    pub(crate) fn declaration(&self) -> &GrokAcpMcpServerDeclaration {
        &self.declaration
    }

    /// Takes the courier launch so its blocking ready barrier can be bounded.
    ///
    /// The barrier is wall-clock and blocking, so open runs it on a scoped task
    /// and races it against the same opening deadline as every other step.
    /// Settling the lease closes the proxy, which releases a waiting barrier,
    /// so the task always joins promptly.
    pub(crate) fn take_launch(&mut self) -> Option<RegisteredToolProxyLaunch> {
        self.launch.take()
    }

    pub(crate) fn claim(mut self) -> GrokRegisteredToolSession {
        self.claimed = true;
        drop(self.launch.take());
        GrokRegisteredToolSession::new(
            self.lease.take().expect("registered lease is present"),
            self.turn.clone(),
        )
    }

    /// Settles the still-unclaimed lease in place, keeping this open usable.
    ///
    /// Used when the ready barrier expires: the close releases the barrier so
    /// its task can join, and the caller then abandons the rest of the open.
    pub(crate) async fn settle_in_place(
        &mut self,
        services: &HostServices,
        cause: RegisteredToolCleanupCause,
    ) -> CleanupOutcome {
        let outcome = close_registered_lease(self.lease.take(), services, cause).await;
        // Retained, not discarded: the abandonment that follows must report
        // this truth so a failed close still retains the route's own leases.
        self.settled = Some(match self.settled.take() {
            Some(recorded) => worst_cleanup(recorded, outcome.clone()),
            None => outcome.clone(),
        });
        outcome
    }

    /// Closes a lease that never reached an open session, with its truth.
    pub(crate) async fn abandon(
        mut self,
        services: &HostServices,
        cause: RegisteredToolCleanupCause,
    ) -> CleanupOutcome {
        self.claimed = true;
        drop(self.launch.take());
        let closed = close_registered_lease(self.lease.take(), services, cause).await;
        match self.settled.take() {
            Some(recorded) => worst_cleanup(recorded, closed),
            None => closed,
        }
    }
}

impl Drop for PendingRegisteredOpen {
    fn drop(&mut self) {
        if self.claimed {
            return;
        }
        drop(self.launch.take());
        drop(self.lease.take());
    }
}

/// Closes one minted lease and reports its exact cleanup truth.
pub(crate) async fn close_registered_lease(
    lease: Option<RegisteredToolBridgeLease>,
    services: &HostServices,
    cause: RegisteredToolCleanupCause,
) -> CleanupOutcome {
    let Some(lease) = lease else {
        return CleanupOutcome::NotApplicable;
    };
    close_lease(lease, services, cause).await
}

/// Reports the more serious of two cleanup truths.
fn worst_cleanup(left: CleanupOutcome, right: CleanupOutcome) -> CleanupOutcome {
    match (left, right) {
        (CleanupOutcome::Failed(diagnostic), _) | (_, CleanupOutcome::Failed(diagnostic)) => {
            CleanupOutcome::Failed(diagnostic)
        }
        (CleanupOutcome::Degraded(diagnostic), _) | (_, CleanupOutcome::Degraded(diagnostic)) => {
            CleanupOutcome::Degraded(diagnostic)
        }
        (CleanupOutcome::Clean, _) | (_, CleanupOutcome::Clean) => CleanupOutcome::Clean,
        (CleanupOutcome::NotApplicable, CleanupOutcome::NotApplicable) => {
            CleanupOutcome::NotApplicable
        }
    }
}

async fn close_lease(
    lease: RegisteredToolBridgeLease,
    services: &HostServices,
    cause: RegisteredToolCleanupCause,
) -> CleanupOutcome {
    match services.registered_tool_bridge() {
        Some(bridge) => {
            let _ = bridge.completion_gate(&lease).await;
            match bridge.close(lease, cause).await {
                Ok(outcome) => outcome,
                Err(error) => CleanupOutcome::Failed(error.diagnostic().clone()),
            }
        }
        None => CleanupOutcome::Failed(SafeDiagnostic::new(
            "swallowtail.grok.acp.registered_tool.bridge_missing",
            "Grok Build registered-tool bridge disappeared during cleanup",
        )),
    }
}

pub(crate) async fn prepare_registered(
    binding: &GrokRegisteredToolBinding,
    version: &InterfaceVersion,
    plan: &PreflightPlan,
    request_id: &RequestId,
    services: &HostServices,
) -> Result<PendingRegisteredOpen, RuntimeFailure> {
    if !matches!(
        super::projection::grok_build_acp_registered_tool_qualification(version),
        swallowtail_runtime::RegisteredToolRouteQualification::Qualified(_)
    ) {
        return Err(failure(
            GROK_ACP_REGISTERED_TOOL_VERSION_NOT_ADMITTED_CODE,
            "Grok Build registered-tool open requires an executable version inside the accepted live segments (exact maintained 1.0.4..=1.0.5)",
        ));
    }
    let host = binding.require_host()?;
    let deadline = binding.require_deadline()?;
    let turn = binding.require_turn()?.clone();
    let recipe = binding
        .selection()
        .proxy_recipe()
        .expect("qualify requires a proxy recipe");
    let command = host
        .approved_executable_path(recipe.executable())
        .and_then(|path| path.to_str().map(str::to_owned))
        .filter(|command| !command.is_empty())
        .ok_or_else(|| {
            failure(
                "swallowtail.grok.acp.registered_tool.command_unresolved",
                "Grok Build ACP registered-tool open requires a host-approved native courier path",
            )
        })?;
    if !std::path::Path::new(&command).is_file() {
        return Err(failure(
            "swallowtail.grok.acp.registered_tool.command_unspawnable",
            "Grok Build ACP registered-tool courier command is not a spawnable filesystem path",
        ));
    }
    let environment = declared_environment(
        host.approved_environment(recipe.environment())
            .ok_or_else(|| {
                failure(
                    "swallowtail.grok.acp.registered_tool.environment_unresolved",
                    "Grok Build ACP registered-tool open requires a host-approved courier environment",
                )
            })?,
    )?;
    RegisteredToolReadiness::evaluate(services, binding.selection())
        .require_ready()
        .map_err(RegisteredToolFailure::into_runtime_failure)?;
    let scope = ScopeId::new(format!("grok-acp:registered-tool:{}", request_id.as_str()))
        .map_err(|_| registered_identity_rejected())?;
    let prepared = binding.preparation().prepare(
        services,
        plan.instance_id().clone(),
        scope,
        turn.clone(),
        deadline,
    )?;
    let lease = prepared.open().await?;
    // Every failure after the lease is minted closes it explicitly and reports
    // its cleanup truth. Drop alone is defensive, never close evidence.
    let launch = match host.registered_tool_proxy_launch(&lease) {
        Ok(launch) => launch,
        Err(error) => {
            return Err(merge_open_failure(
                error,
                close_registered_lease(
                    Some(lease),
                    services,
                    RegisteredToolCleanupCause::ProviderFailure,
                )
                .await,
            ));
        }
    };
    let declaration = GrokAcpMcpServerDeclaration::courier(
        binding.carrier().server_name(),
        command,
        launch
            .process_request()
            .arguments()
            .map(str::to_owned)
            .collect(),
        environment,
    );
    Ok(PendingRegisteredOpen {
        lease: Some(lease),
        launch: Some(launch),
        declaration,
        turn,
        settled: None,
        claimed: false,
    })
}

/// Reports the original open failure, or the cleanup failure that hid it.
///
/// A failed close is never invisible: when both happen the cleanup failure is
/// surfaced, because a retained lease is the more serious truth.
fn merge_open_failure(error: RuntimeFailure, cleanup: CleanupOutcome) -> RuntimeFailure {
    match cleanup {
        CleanupOutcome::Failed(diagnostic) | CleanupOutcome::Degraded(diagnostic) => {
            RuntimeFailure::new(diagnostic)
        }
        CleanupOutcome::Clean | CleanupOutcome::NotApplicable => error,
    }
}

/// Renders the host-approved recipe environment for the ACP declaration.
///
/// Swallowtail-private bindings are dropped: the rendezvous file carries the
/// endpoint, bearer, and generations, so nothing named `SWALLOWTAIL_` needs to
/// cross an ACP wire the provider can read.
fn declared_environment(
    bindings: &[(std::ffi::OsString, std::ffi::OsString)],
) -> Result<Vec<(String, String)>, RuntimeFailure> {
    let mut environment = Vec::new();
    for (key, value) in bindings {
        let (Some(key), Some(value)) = (key.to_str(), value.to_str()) else {
            return Err(failure(
                "swallowtail.grok.acp.registered_tool.environment_unresolved",
                "Grok Build ACP registered-tool environment must be UTF-8",
            ));
        };
        if key.starts_with("SWALLOWTAIL_") {
            continue;
        }
        environment.push((key.to_owned(), value.to_owned()));
    }
    Ok(environment)
}

fn registered_identity_rejected() -> RuntimeFailure {
    failure(
        "swallowtail.grok.acp.registered_tool.identity_rejected",
        "Grok Build ACP registered-tool open could not bind its scope and turn identity",
    )
}

#[cfg(test)]
mod tests_support {
    use super::*;
    use std::sync::Arc;
    use swallowtail_core::{ConfiguredInstanceId, ExecutionHostId};
    use swallowtail_runtime::{
        BoxFuture, CleanupOutcome, Deadline, MonotonicInstant, RegisteredServerId,
        RegisteredServerRevision, RegisteredToolBounds, RegisteredToolCall,
        RegisteredToolDeclaration, RegisteredToolDispatchContext, RegisteredToolDispatcher,
        RegisteredToolEffectPosture, RegisteredToolExecutionKind, RegisteredToolId,
        RegisteredToolLimits, RegisteredToolLocalName, RegisteredToolNamespace,
        RegisteredToolOutcome, RegisteredToolPreparation, RegisteredToolProtocolVersion,
        RegisteredToolRetryPosture, RegisteredToolSchema, RegisteredToolSchemaDialect,
        RegisteredToolSchemaDigest, RegisteredToolSchemaDocument, RegisteredToolSchemaMediaType,
        RegisteredToolSchemaNamespace, RegisteredToolSelection, RegisteredToolSnapshot,
        RegisteredToolSnapshotInput, RegisteredToolSource, RegisteredToolSourceId,
        RegisteredToolTransport, RegisteredToolTransportSupport, RuntimeFailure, RuntimeTurnId,
        ScopeId,
    };

    struct UnusedDispatcher;

    impl RegisteredToolDispatcher for UnusedDispatcher {
        fn dispatch(
            &self,
            _call: RegisteredToolCall,
            _context: RegisteredToolDispatchContext,
        ) -> BoxFuture<'_, Result<RegisteredToolOutcome, RuntimeFailure>> {
            Box::pin(async { Err(super::tests::fixture_failure()) })
        }
    }

    fn schema() -> RegisteredToolSchema {
        RegisteredToolSchema::new(
            RegisteredToolSchemaNamespace::new("desktop.schema").expect("namespace"),
            RegisteredToolSchemaMediaType::new("application/json").expect("media type"),
            RegisteredToolSchemaDialect::new("json-schema-2020-12").expect("dialect"),
            RegisteredServerRevision::new("1").expect("revision"),
            RegisteredToolSchemaDigest::new("sha256:fixture").expect("digest"),
            RegisteredToolSchemaDocument::new("{\"type\":\"object\"}").expect("document"),
        )
    }

    /// Opens one real lease, then binds it to a bridge the test can hold open.
    ///
    /// Only the kernel can mint a lease, so the lease is genuine; the close it
    /// is settled through is the held fixture bridge.
    pub(super) fn held_session() -> (
        GrokRegisteredToolSession,
        HostServices,
        futures_channel::oneshot::Sender<CleanupOutcome>,
    ) {
        let host = ExecutionHostId::new("grok.unit.registered-host").expect("host");
        let tool = RegisteredToolId::new(
            RegisteredToolNamespace::new("desktop").expect("namespace"),
            RegisteredToolLocalName::new("reconcile").expect("local name"),
        );
        let snapshot = Arc::new(
            RegisteredToolSnapshot::new(RegisteredToolSnapshotInput {
                server_id: RegisteredServerId::new("desktop.registered-tools").expect("server"),
                revision: RegisteredServerRevision::new("1").expect("revision"),
                execution_host_id: host.clone(),
                declarations: vec![
                    RegisteredToolDeclaration::new(
                        tool.clone(),
                        RegisteredToolExecutionKind::Mcp,
                        schema(),
                        schema(),
                        RegisteredToolEffectPosture::ReadOnly,
                        RegisteredToolRetryPosture::ConsumerRetryable,
                        RegisteredToolBounds::ceiling(),
                    )
                    .expect("declaration"),
                ],
                transports: vec![
                    RegisteredToolTransportSupport::new(
                        RegisteredToolTransport::HostMediatedCallback,
                        [RegisteredToolProtocolVersion::new(
                            swallowtail_runtime::REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION,
                        )
                        .expect("protocol")],
                    )
                    .expect("transport"),
                ],
                required_services: [].into_iter().collect(),
                credential_references: Vec::new(),
                executable_recipes: Vec::new(),
                environment_recipes: Vec::new(),
                bounds: RegisteredToolBounds::ceiling(),
                source: RegisteredToolSource::new(
                    RegisteredToolSourceId::new("desktop.registration.1").expect("source"),
                    MonotonicInstant::from_ticks(1),
                ),
            })
            .expect("snapshot"),
        );
        let selection = RegisteredToolSelection::new(
            Arc::clone(&snapshot),
            [tool],
            RegisteredToolTransport::HostMediatedCallback,
            RegisteredToolProtocolVersion::new(
                swallowtail_runtime::REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION,
            )
            .expect("protocol"),
        )
        .expect("selection");
        let preparation = RegisteredToolPreparation::new(
            snapshot,
            selection,
            swallowtail_testkit::fixture_admission(Arc::new(
                swallowtail_testkit::ScriptedAdmissionPort::current(),
            )),
            RegisteredToolLimits::ceiling(),
        );
        let local = swallowtail_host_local::LocalProcessHost::builder(
            swallowtail_host_local::LocalProcessLimits::default(),
        )
        .with_registered_tool_dispatcher(Arc::new(UnusedDispatcher))
        .build_services(host.clone());
        let lease = futures_executor::block_on(
            preparation
                .prepare(
                    local.services(),
                    ConfiguredInstanceId::new("grok.unit.instance").expect("instance"),
                    ScopeId::new("grok.unit.scope").expect("scope"),
                    RuntimeTurnId::new("grok.unit.turn").expect("turn"),
                    Deadline::at(MonotonicInstant::from_ticks(10_000_000_000)),
                )
                .expect("prepared registration")
                .open(),
        )
        .expect("lease opens");
        let (sender, receiver) = futures_channel::oneshot::channel();
        let services = HostServices::new(host).with_registered_tool_bridge(Arc::new(
            super::tests::HeldBridge {
                release: std::sync::Mutex::new(Some(receiver)),
            },
        ));
        (
            GrokRegisteredToolSession::new(
                lease,
                RuntimeTurnId::new("grok.unit.turn").expect("turn"),
            ),
            services,
            sender,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::future::Future;
    use std::task::Context;
    use swallowtail_runtime::{
        BoxFuture, CleanupOutcome, RegisteredToolAdmissionState, RegisteredToolBridgeHostService,
        RegisteredToolCompletionState, RegisteredToolLifecycleState, RegisteredToolOpenRequest,
        RuntimeFailure,
    };

    /// A bridge whose close finishes only when the test releases it.
    ///
    /// This is what makes the overlap a construction rather than a race: the
    /// first settlement provably cannot complete until the second caller has
    /// already been polled into its waiting path.
    pub(super) struct HeldBridge {
        pub(super) release:
            std::sync::Mutex<Option<futures_channel::oneshot::Receiver<CleanupOutcome>>>,
    }

    impl RegisteredToolBridgeHostService for HeldBridge {
        fn open(
            &self,
            _request: RegisteredToolOpenRequest,
        ) -> BoxFuture<'_, Result<RegisteredToolBridgeLease, RuntimeFailure>> {
            Box::pin(async { Err(fixture_failure()) })
        }

        fn completion_gate(
            &self,
            _lease: &RegisteredToolBridgeLease,
        ) -> BoxFuture<'_, Result<RegisteredToolCompletionState, RuntimeFailure>> {
            Box::pin(async {
                Ok(RegisteredToolCompletionState::new(
                    RegisteredToolAdmissionState::Frozen,
                    RegisteredToolLifecycleState::Frozen,
                    1,
                    false,
                ))
            })
        }

        fn close(
            &self,
            _lease: RegisteredToolBridgeLease,
            _cause: RegisteredToolCleanupCause,
        ) -> BoxFuture<'_, Result<CleanupOutcome, RuntimeFailure>> {
            let receiver = self
                .release
                .lock()
                .expect("held bridge lock")
                .take()
                .expect("close is called once");
            Box::pin(async move { Ok(receiver.await.unwrap_or(CleanupOutcome::NotApplicable)) })
        }
    }

    pub(super) fn fixture_failure() -> RuntimeFailure {
        RuntimeFailure::new(SafeDiagnostic::new(
            "fixture.grok.registered_tool.unused",
            "fixture bridge entry point is not exercised",
        ))
    }

    /// Two settlers overlap by construction, not by timing.
    ///
    /// The second caller is polled while the first is provably still inside
    /// its close, so it must take the waiting path. Both must then observe the
    /// same cleanup truth: a caller that read a not-yet-recorded outcome could
    /// publish a clean terminal or release a credential beside retained work.
    #[test]
    fn a_second_settler_awaits_the_first_and_sees_the_same_truth() {
        let (session, services, release) = super::tests_support::held_session();
        let waker = futures_util::task::noop_waker();
        let mut context = Context::from_waker(&waker);
        let mut first =
            Box::pin(session.settle(&services, RegisteredToolCleanupCause::Cancellation));
        let mut second =
            Box::pin(session.settle(&services, RegisteredToolCleanupCause::ExplicitClose));
        assert!(
            first.as_mut().poll(&mut context).is_pending(),
            "the first settler enters the held close"
        );
        assert!(
            second.as_mut().poll(&mut context).is_pending(),
            "the second settler must wait while the first close is in flight"
        );
        let failed = CleanupOutcome::Failed(SafeDiagnostic::new(
            "fixture.grok.registered_tool.retained",
            "fixture close retained its lease",
        ));
        release.send(failed.clone()).expect("release is delivered");
        let first = futures_executor::block_on(first);
        let second = futures_executor::block_on(second);
        assert_eq!(first, failed);
        assert_eq!(
            second, failed,
            "a concurrent settler must observe the first close's truth, never a default"
        );
        assert!(!session.settled_clean());
        assert!(session.cleanup_failed());
    }
}
