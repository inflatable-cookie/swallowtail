//! Session-open binding of one qualified registered-tool courier.

use crate::sdk::failure::failure;
use crate::sdk::mcp::OpenStdioMcpServer;
use crate::sdk::registered_tool::ClaudeAgentSdkRegisteredToolBinding;
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    HostServices, OpenSessionRequest, ProcessHandle, ProcessService, RegisteredToolBridgeLease,
    RegisteredToolCleanupCause, RegisteredToolFailure, RegisteredToolReadiness, RuntimeFailure,
    RuntimeTurnId, ScopeId,
};

/// Live registered-tool courier bound to one open sidecar session.
pub(in crate::sdk) struct ClaudeAgentSdkRegisteredToolSession {
    lease: Option<RegisteredToolBridgeLease>,
    courier: Option<Box<dyn ProcessHandle>>,
    services: HostServices,
}

impl ClaudeAgentSdkRegisteredToolSession {
    pub(in crate::sdk) async fn close(mut self, services: &HostServices) {
        close_registered(
            self.courier.take(),
            self.lease.take(),
            services,
            RegisteredToolCleanupCause::ExplicitClose,
        )
        .await;
    }
}

impl Drop for ClaudeAgentSdkRegisteredToolSession {
    fn drop(&mut self) {
        let courier = self.courier.take();
        let lease = self.lease.take();
        if courier.is_none() && lease.is_none() {
            return;
        }
        let services = self.services.clone();
        futures_executor::block_on(close_registered(
            courier,
            lease,
            &services,
            RegisteredToolCleanupCause::ProviderFailure,
        ));
    }
}

async fn close_registered(
    courier: Option<Box<dyn ProcessHandle>>,
    lease: Option<RegisteredToolBridgeLease>,
    services: &HostServices,
    cause: RegisteredToolCleanupCause,
) {
    if let Some(lease) = lease
        && let Some(bridge) = services.registered_tool_bridge()
    {
        let _ = bridge.close(lease, cause).await;
    }
    if let Some(courier) = courier {
        let _ = courier.request_stop().await;
    }
}

/// Partial registered-tool open that closes the lease and courier if dropped
/// before the session claims them.
struct PendingRegisteredOpen {
    services: HostServices,
    lease: Option<RegisteredToolBridgeLease>,
    courier: Option<Box<dyn ProcessHandle>>,
    claimed: bool,
}

impl PendingRegisteredOpen {
    fn claim(mut self) -> ClaudeAgentSdkRegisteredToolSession {
        self.claimed = true;
        ClaudeAgentSdkRegisteredToolSession {
            lease: Some(self.lease.take().expect("registered lease is present")),
            courier: Some(self.courier.take().expect("registered courier is present")),
            services: self.services.clone(),
        }
    }
}

impl Drop for PendingRegisteredOpen {
    fn drop(&mut self) {
        if self.claimed {
            return;
        }
        let lease = self.lease.take();
        let courier = self.courier.take();
        let services = self.services.clone();
        futures_executor::block_on(close_registered(
            courier,
            lease,
            &services,
            RegisteredToolCleanupCause::ProviderFailure,
        ));
    }
}

pub(in crate::sdk) async fn open_registered(
    binding: &ClaudeAgentSdkRegisteredToolBinding,
    plan: &PreflightPlan,
    request: &OpenSessionRequest,
    services: &HostServices,
) -> Result<(ClaudeAgentSdkRegisteredToolSession, OpenStdioMcpServer), RuntimeFailure> {
    let host = binding.host().ok_or_else(|| {
        failure(
            "swallowtail.claude-agent.sdk.registered_tool.host_missing",
            "Claude Agent SDK registered-tool route binding requires the local host composition that mints the courier rendezvous",
        )
    })?;
    RegisteredToolReadiness::evaluate(services, binding.selection())
        .require_ready()
        .map_err(RegisteredToolFailure::into_runtime_failure)?;
    let request_id = request.request_id().as_str();
    let scope = ScopeId::new(format!("claude-agent-sdk:registered-tool:{request_id}"))
        .expect("validated request id produces a valid registered-tool scope");
    let turn = RuntimeTurnId::new(format!("claude-agent-sdk:registered:{request_id}"))
        .expect("validated request id produces a valid registered-tool turn");
    let deadline = request.deadline().ok_or_else(|| {
        failure(
            "swallowtail.claude-agent.sdk.registered_tool.deadline_missing",
            "Claude Agent SDK registered-tool open requires the session deadline",
        )
    })?;
    let prepared = binding.preparation().prepare(
        services,
        plan.instance_id().clone(),
        scope.clone(),
        turn,
        deadline,
    )?;
    let lease = prepared.open().await?;
    let mut launch = host.registered_tool_proxy_launch(&lease)?;
    let process = services.process().cloned().ok_or_else(|| {
        failure(
            "swallowtail.claude-agent.sdk.registered_tool.process_missing",
            "Claude Agent SDK registered-tool open requires the host process service",
        )
    })?;
    let courier =
        ProcessService::start(process.as_ref(), scope, launch.process_request().clone()).await?;
    let pending = PendingRegisteredOpen {
        services: services.clone(),
        lease: Some(lease),
        courier: Some(courier),
        claimed: false,
    };
    launch.wait_until_ready()?;
    let recipe = binding
        .selection()
        .proxy_recipe()
        .expect("qualify requires a proxy recipe");
    let declaration = OpenStdioMcpServer::registered_tool_courier(
        recipe.executable().as_host_value().to_owned(),
        launch
            .process_request()
            .arguments()
            .map(str::to_owned)
            .collect(),
        binding
            .carrier()
            .tools()
            .iter()
            .map(|tool| tool.carrier_tool_name().to_owned())
            .collect(),
    );
    Ok((pending.claim(), declaration))
}
