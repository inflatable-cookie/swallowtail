//! Fresh-session driver for the source-tagged Claude Agent SDK Node sidecar.
//!
//! The driver owns no launch argv: the application-approved launch recipe
//! (host-local interpreted Node plus the sidecar entry point) comes from the
//! bound instance target, and the application-provisioned SDK module, native
//! binary, and shipped manifest arrive through the approved environment. The
//! sidecar starts inside the host's descendant-tree authority, so the native
//! binary and everything it spawns stay enrolled in one host-owned tree.

use self::session::ClaudeAgentSdkSessionHandle;
use self::validation::validate_open;
use crate::sdk::bounded::HostBound;
use crate::sdk::connection::SdkConnection;
use crate::sdk::failure::failure;
use crate::sdk::guardian::OpenGuard;
use std::sync::{Arc, Mutex};
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    BoxFuture, EnvironmentRef, HostServices, InteractiveSessionDriver, InteractiveSessionHandle,
    LoadSessionRequest, LoadedSession, OpenSessionRequest, ResumeSessionRequest, RuntimeFailure,
    RuntimeSessionId,
};

mod descriptor;
mod handle;
mod launch;
mod session;
mod startup;
mod validation;

pub(super) const SDK_DRIVER_ID: &str = "swallowtail.claude-agent.sdk";

/// Low-level driver for fresh Claude Agent SDK sidecar sessions.
pub struct ClaudeAgentSdkDriver {
    environment: EnvironmentRef,
    credential: swallowtail_core::CredentialRef,
}

impl ClaudeAgentSdkDriver {
    /// Binds the host-private environment and delegated subscription
    /// credential reference. No credential value is ever held here.
    #[must_use]
    pub const fn new(
        environment: EnvironmentRef,
        credential: swallowtail_core::CredentialRef,
    ) -> Self {
        Self {
            environment,
            credential,
        }
    }
}

/// One spawned sidecar attachment before readiness.
///
/// The leases, process, and pump task are not held here: the open guard holds
/// them until the success path claims them, so a dropped open future cannot
/// strand a partial acquisition.
pub(super) struct PendingSession {
    pub(super) request_id: swallowtail_runtime::RequestId,
    pub(super) connection: Arc<SdkConnection>,
    pub(super) services: HostServices,
    pub(super) leased_cwd: String,
}

impl PendingSession {
    pub(in crate::sdk::driver) fn into_handle(
        self,
        plan: &PreflightPlan,
        readiness: startup::SessionReadiness,
        acquired: crate::sdk::guardian::Acquisitions,
    ) -> ClaudeAgentSdkSessionHandle {
        let runtime_id =
            RuntimeSessionId::new(format!("claude-agent-sdk:{}", self.request_id.as_str()))
                .expect("validated request id produces a valid sidecar runtime session id");
        let active: crate::sdk::driver::session::ActiveSlot = Arc::new(Mutex::new(None));
        ClaudeAgentSdkSessionHandle {
            request_id: self.request_id,
            runtime_id,
            execution_host_id: plan.execution_host_id().clone(),
            connection: Arc::clone(&self.connection),
            cancellation: handle::SessionCancellation::new(Arc::clone(&active)),
            pump_task: acquired.pump,
            services: self.services,
            resource: acquired.resource,
            credential: acquired.credential,
            readiness,
            active,
        }
    }
}

impl InteractiveSessionDriver for ClaudeAgentSdkDriver {
    fn open_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            validate_open(&plan, &request, &services, &self.credential)?;
            let deadline = request.deadline().expect("validated open deadline");
            let bounded = HostBound::new(
                services
                    .time()
                    .cloned()
                    .expect("validated sidecar time service"),
                deadline,
            );
            if bounded.expired() {
                return Err(open_deadline_elapsed());
            }
            // Armed before the first acquisition. From here on, every lease,
            // process, and task the open path takes is recorded in the guard,
            // so the caller's deadline can drop this future at any point
            // without stranding what was already acquired.
            let guard = OpenGuard::arm(&services, request.request_id().as_str(), deadline)?;
            let opened = bounded
                .run(self.acquire_and_start(&plan, &request, services.clone(), &guard))
                .await;
            match opened {
                Some(Ok((pending, readiness))) => {
                    let acquired = guard.claim();
                    Ok(Box::new(pending.into_handle(&plan, readiness, acquired))
                        as Box<dyn InteractiveSessionHandle>)
                }
                Some(Err(error)) => {
                    // A failure that only happened because the guard already
                    // terminated at the deadline is reported as the deadline,
                    // not as whatever the collapsing connection said next.
                    let expired = bounded.expired() || guard.deadline_fired();
                    let cleaned = guard.fire(&bounded).await;
                    Err(match (expired, cleaned) {
                        (_, false) => open_cleanup_unconfirmed(),
                        (true, true) => open_deadline_elapsed(),
                        (false, true) => error,
                    })
                }
                None => {
                    // The bound expired inside acquisition or startup. The
                    // guard still terminates and releases under host ownership;
                    // this future returns now either way.
                    let cleaned = guard.fire(&bounded).await;
                    Err(if cleaned {
                        open_deadline_elapsed()
                    } else {
                        open_cleanup_unconfirmed()
                    })
                }
            }
        })
    }

    fn load_session(
        &self,
        _plan: PreflightPlan,
        _request: LoadSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<LoadedSession, RuntimeFailure>> {
        Box::pin(async { Err(crate::sdk::failure::unsupported("session load")) })
    }

    fn resume_session(
        &self,
        _plan: PreflightPlan,
        _request: ResumeSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async { Err(crate::sdk::failure::unsupported("session resume")) })
    }
}

impl ClaudeAgentSdkDriver {
    /// The whole provider-facing open: acquisition, launch, and readiness.
    ///
    /// It is one future so the caller's deadline covers all of it, and every
    /// acquisition inside is recorded in the guard before the next await.
    async fn acquire_and_start(
        &self,
        plan: &PreflightPlan,
        request: &OpenSessionRequest,
        services: HostServices,
        guard: &OpenGuard,
    ) -> Result<(PendingSession, startup::SessionReadiness), RuntimeFailure> {
        let pending = self
            .spawn_session(
                plan,
                request.request_id().clone(),
                request
                    .working_resource()
                    .expect("validated sidecar working resource")
                    .clone(),
                request.access_policy(),
                services,
                guard,
            )
            .await?;
        let readiness = startup::open(&pending.connection, plan, &pending.leased_cwd).await?;
        Ok((pending, readiness))
    }
}

fn open_deadline_elapsed() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.open_deadline_elapsed",
        "Claude Agent SDK sidecar session reached its host deadline before readiness",
    )
}

/// The open failed and its cleanup could not finish inside the same caller
/// bound. Termination was requested; completion is unconfirmed, and saying so
/// is the honest report.
fn open_cleanup_unconfirmed() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.open_cleanup_unconfirmed",
        "Claude Agent SDK sidecar termination was requested, but cleanup did not complete inside \
         the caller's open deadline",
    )
}

fn scope_invalid() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.scope_invalid",
        "Claude Agent SDK sidecar scope was invalid",
    )
}

pub use descriptor::claude_agent_sdk_descriptor;

pub(crate) use startup::EXPECTED_TOOLS;
pub(crate) use validation::{ACCESS_NAMESPACE, ENDPOINT_AUDIENCE};
