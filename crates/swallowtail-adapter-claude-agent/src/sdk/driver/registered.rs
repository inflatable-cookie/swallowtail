//! Session-open binding of one qualified registered-tool courier.

use crate::sdk::failure::failure;
use crate::sdk::mcp::{OpenStdioMcpServer, env_key_allowed};
use crate::sdk::registered_tool::ClaudeAgentSdkRegisteredToolBinding;
use swallowtail_core::{PreflightPlan, SafeDiagnostic};
use swallowtail_host_local::RegisteredToolProxyLaunch;
use swallowtail_runtime::{
    CleanupOutcome, HostServices, OpenSessionRequest, RegisteredToolBridgeLease,
    RegisteredToolCleanupCause, RegisteredToolFailure, RegisteredToolReadiness, RuntimeFailure,
    RuntimeTurnId,
};

/// Live registered-tool lease bound to one open sidecar session.
///
/// The courier process is the provider's child. This session owns the bridge
/// lease only; it never holds a Swallowtail-spawned process handle.
pub(in crate::sdk) struct ClaudeAgentSdkRegisteredToolSession {
    lease: Option<RegisteredToolBridgeLease>,
}

impl ClaudeAgentSdkRegisteredToolSession {
    pub(in crate::sdk) fn take_lease(&mut self) -> Option<RegisteredToolBridgeLease> {
        self.lease.take()
    }
}

/// Partial registered-tool open that never holds a courier process handle.
pub(in crate::sdk) struct PendingRegisteredOpen {
    lease: Option<RegisteredToolBridgeLease>,
    launch: Option<RegisteredToolProxyLaunch>,
    declaration: OpenStdioMcpServer,
    claimed: bool,
}

impl PendingRegisteredOpen {
    pub(in crate::sdk) fn declaration(&self) -> OpenStdioMcpServer {
        self.declaration.clone()
    }

    pub(in crate::sdk) fn wait_until_ready(&mut self) -> Result<(), RuntimeFailure> {
        self.launch
            .as_mut()
            .expect("registered launch is present until claimed")
            .wait_until_ready()
    }

    pub(in crate::sdk) fn claim(mut self) -> ClaudeAgentSdkRegisteredToolSession {
        self.claimed = true;
        drop(self.launch.take());
        ClaudeAgentSdkRegisteredToolSession {
            lease: Some(self.lease.take().expect("registered lease is present")),
        }
    }

    pub(in crate::sdk) fn into_unclaimed_lease(mut self) -> Option<RegisteredToolBridgeLease> {
        self.claimed = true;
        drop(self.launch.take());
        self.lease.take()
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

pub(in crate::sdk) async fn close_registered_lease(
    lease: Option<RegisteredToolBridgeLease>,
    services: &HostServices,
    cause: RegisteredToolCleanupCause,
) -> Option<CleanupOutcome> {
    match lease {
        Some(lease) => match services.registered_tool_bridge() {
            Some(bridge) => Some(match bridge.close(lease, cause).await {
                Ok(outcome) => outcome,
                Err(error) => CleanupOutcome::Failed(error.diagnostic().clone()),
            }),
            // A lease without its bridge service can never report a release:
            // saying so is the honest observation.
            None => Some(CleanupOutcome::Failed(SafeDiagnostic::new(
                "swallowtail.claude-agent.sdk.registered_tool_bridge_unavailable",
                "Claude Agent SDK registered-tool bridge service was unavailable during cleanup",
            ))),
        },
        None => None,
    }
}

pub(in crate::sdk) async fn prepare_registered(
    binding: &ClaudeAgentSdkRegisteredToolBinding,
    plan: &PreflightPlan,
    request: &OpenSessionRequest,
    services: &HostServices,
) -> Result<PendingRegisteredOpen, RuntimeFailure> {
    let host = binding.host().ok_or_else(|| {
        failure(
            "swallowtail.claude-agent.sdk.registered_tool.host_missing",
            "Claude Agent SDK registered-tool route binding requires the local host composition that mints the courier rendezvous",
        )
    })?;
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
                "swallowtail.claude-agent.sdk.registered_tool.command_unresolved",
                "Claude Agent SDK registered-tool open requires a host-approved native courier path",
            )
        })?;
    if !std::path::Path::new(&command).is_file() {
        return Err(failure(
            "swallowtail.claude-agent.sdk.registered_tool.command_unspawnable",
            "Claude Agent SDK registered-tool courier command is not a spawnable filesystem path",
        ));
    }
    let env = allowlisted_recipe_environment(
        host.approved_environment(recipe.environment())
            .ok_or_else(|| {
                failure(
                    "swallowtail.claude-agent.sdk.registered_tool.environment_unresolved",
                    "Claude Agent SDK registered-tool open requires a host-approved card 084 environment",
                )
            })?,
    )?;
    RegisteredToolReadiness::evaluate(services, binding.selection())
        .require_ready()
        .map_err(RegisteredToolFailure::into_runtime_failure)?;
    let request_id = request.request_id().as_str();
    let scope =
        swallowtail_runtime::ScopeId::new(format!("claude-agent-sdk:registered-tool:{request_id}"))
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
        scope,
        turn,
        deadline,
    )?;
    let lease = prepared.open().await?;
    let launch = host.registered_tool_proxy_launch(&lease)?;
    let declaration = OpenStdioMcpServer::registered_tool_courier(
        command,
        launch
            .process_request()
            .arguments()
            .map(str::to_owned)
            .collect(),
        env,
        binding
            .carrier()
            .tools()
            .iter()
            .map(|tool| tool.carrier_tool_name().to_owned())
            .collect(),
    );
    Ok(PendingRegisteredOpen {
        lease: Some(lease),
        launch: Some(launch),
        declaration,
        claimed: false,
    })
}

fn allowlisted_recipe_environment(
    bindings: &[(std::ffi::OsString, std::ffi::OsString)],
) -> Result<Vec<(String, String)>, RuntimeFailure> {
    let mut env = Vec::new();
    for (key, value) in bindings {
        let key = key.to_str().ok_or_else(|| {
            failure(
                "swallowtail.claude-agent.sdk.registered_tool.environment_unresolved",
                "Claude Agent SDK registered-tool environment keys must be UTF-8",
            )
        })?;
        if key.starts_with("SWALLOWTAIL_") || !env_key_allowed(key) {
            continue;
        }
        let value = value.to_str().ok_or_else(|| {
            failure(
                "swallowtail.claude-agent.sdk.registered_tool.environment_unresolved",
                "Claude Agent SDK registered-tool environment values must be UTF-8",
            )
        })?;
        env.push((key.to_owned(), value.to_owned()));
    }
    Ok(env)
}
