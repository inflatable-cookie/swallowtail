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
    /// contract order. Used when readiness fails.
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
            if request.deadline().is_some_and(|deadline| {
                services
                    .time()
                    .expect("validated sidecar time service")
                    .now()
                    >= deadline.instant()
            }) {
                return Err(failure(
                    "swallowtail.claude-agent.sdk.open_deadline_elapsed",
                    "Claude Agent SDK sidecar session deadline elapsed before startup",
                ));
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
            match startup::open(&pending.connection, &plan, &pending.leased_cwd).await {
                Ok(readiness) => Ok(Box::new(pending.into_handle(&plan, readiness))
                    as Box<dyn InteractiveSessionHandle>),
                Err(error) => {
                    pending.abort().await;
                    Err(error)
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

fn scope_invalid() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.scope_invalid",
        "Claude Agent SDK sidecar scope was invalid",
    )
}

pub use descriptor::claude_agent_sdk_descriptor;

pub(crate) use validation::{ACCESS_NAMESPACE, ENDPOINT_AUDIENCE};
