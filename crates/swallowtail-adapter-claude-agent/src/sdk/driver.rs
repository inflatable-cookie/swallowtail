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
use std::sync::{Arc, Mutex};
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    BoxFuture, CredentialLease, EnvironmentRef, HostServices, InteractiveSessionDriver,
    InteractiveSessionHandle, JoinedTask, LoadSessionRequest, LoadedSession, OpenSessionRequest,
    ResourceLease, ResumeSessionRequest, RuntimeFailure, RuntimeSessionId,
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

/// One spawned, verified sidecar attachment before readiness.
pub(super) struct PendingSession {
    pub(super) request_id: swallowtail_runtime::RequestId,
    pub(super) connection: Arc<SdkConnection>,
    pub(super) pump_task: Option<Box<dyn JoinedTask>>,
    pub(super) services: HostServices,
    pub(super) resource: Option<ResourceLease>,
    pub(super) credential: Option<CredentialLease>,
    pub(super) leased_cwd: String,
}

impl PendingSession {
    /// Closes the sidecar through host authority and releases leases in
    /// contract order. Used when readiness fails or the open deadline expires.
    ///
    /// LIMIT, stated rather than hidden: escalation runs first, but the join
    /// that follows is not itself bounded by the caller's deadline. That
    /// deadline has already expired by the time abort runs, and no fresh
    /// host-observed bound can be derived without a timing seam, so this path
    /// depends on host termination completing. The caller's deadline therefore
    /// bounds *detection* of a stuck open, not the return of every cleanup
    /// await after it.
    pub(in crate::sdk::driver) async fn abort(mut self) {
        self.connection.begin_close().await;
        let _ = self.connection.escalate().await;
        if let Some(task) = self.pump_task.take() {
            let _ = task.join().await;
        }
        if let (Some(lease), Some(service)) =
            (self.resource.take(), self.services.working_resource())
        {
            let _ = service.release(lease).await;
        }
        if let (Some(lease), Some(service)) = (self.credential.take(), self.services.credential()) {
            let _ = service.release(lease).await;
        }
    }

    pub(in crate::sdk::driver) fn into_handle(
        self,
        plan: &PreflightPlan,
        readiness: startup::SessionReadiness,
    ) -> ClaudeAgentSdkSessionHandle {
        let runtime_id =
            RuntimeSessionId::new(format!("claude-agent-sdk:{}", self.request_id.as_str()))
                .expect("validated request id produces a valid sidecar runtime session id");
        ClaudeAgentSdkSessionHandle {
            request_id: self.request_id,
            runtime_id,
            execution_host_id: plan.execution_host_id().clone(),
            connection: Arc::clone(&self.connection),
            cancellation: handle::SessionCancellation::new(Arc::clone(&self.connection)),
            pump_task: self.pump_task,
            services: self.services,
            resource: self.resource,
            credential: self.credential,
            readiness,
            active: Arc::new(Mutex::new(None)),
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
            let bounded = HostBound::new(
                services
                    .time()
                    .cloned()
                    .expect("validated sidecar time service"),
                request.deadline().expect("validated open deadline"),
            );
            if bounded.expired() {
                return Err(open_deadline_elapsed());
            }
            let pending = self
                .spawn_session(
                    &plan,
                    request.request_id().clone(),
                    request
                        .working_resource()
                        .expect("validated sidecar working resource")
                        .clone(),
                    request.access_policy(),
                    services,
                )
                .await?;
            // Startup is raced against the caller's host deadline: an SDK that
            // never initializes must not hold the open await, and expiry hands
            // the tree to the host's termination authority. See `abort` for the
            // exact limit of that guarantee.
            match bounded
                .run(startup::open(
                    &pending.connection,
                    &plan,
                    &pending.leased_cwd,
                ))
                .await
            {
                Some(Ok(readiness)) => Ok(Box::new(pending.into_handle(&plan, readiness))
                    as Box<dyn InteractiveSessionHandle>),
                Some(Err(error)) => {
                    pending.abort().await;
                    Err(error)
                }
                None => {
                    pending.abort().await;
                    Err(open_deadline_elapsed())
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

fn open_deadline_elapsed() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.open_deadline_elapsed",
        "Claude Agent SDK sidecar session reached its host deadline before readiness",
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
