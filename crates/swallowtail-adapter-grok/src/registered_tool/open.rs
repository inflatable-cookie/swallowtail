//! Session-open binding of one qualified registered-tool courier.
//!
//! The courier process is Grok's child: Swallowtail declares it in the ACP
//! `mcpServers` list and the provider spawns it. This route owns the bridge
//! lease only; it never holds a Swallowtail-spawned courier process handle.

use super::binding::GrokRegisteredToolBinding;
use super::declaration::GrokAcpMcpServerDeclaration;
use crate::failure::failure;
use swallowtail_core::PreflightPlan;
use swallowtail_host_local::RegisteredToolProxyLaunch;
use swallowtail_runtime::{
    HostServices, RegisteredToolBridgeLease, RegisteredToolCleanupCause, RegisteredToolFailure,
    RegisteredToolReadiness, RequestId, RuntimeFailure, RuntimeTurnId, ScopeId,
};

/// Live registered-tool lease bound to one open ACP session.
pub(crate) struct GrokRegisteredToolSession {
    lease: Option<RegisteredToolBridgeLease>,
}

impl GrokRegisteredToolSession {
    pub(crate) fn take_lease(&mut self) -> Option<RegisteredToolBridgeLease> {
        self.lease.take()
    }
}

/// Partial registered-tool open that never holds a courier process handle.
pub(crate) struct PendingRegisteredOpen {
    lease: Option<RegisteredToolBridgeLease>,
    launch: Option<RegisteredToolProxyLaunch>,
    declaration: GrokAcpMcpServerDeclaration,
    claimed: bool,
}

impl PendingRegisteredOpen {
    pub(crate) fn declaration(&self) -> &GrokAcpMcpServerDeclaration {
        &self.declaration
    }

    pub(crate) fn wait_until_ready(&mut self) -> Result<(), RuntimeFailure> {
        self.launch
            .as_mut()
            .expect("registered launch is present until claimed")
            .wait_until_ready()
    }

    pub(crate) fn claim(mut self) -> GrokRegisteredToolSession {
        self.claimed = true;
        drop(self.launch.take());
        GrokRegisteredToolSession {
            lease: Some(self.lease.take().expect("registered lease is present")),
        }
    }

    pub(crate) fn into_unclaimed_lease(mut self) -> Option<RegisteredToolBridgeLease> {
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

pub(crate) async fn close_registered_lease(
    lease: Option<RegisteredToolBridgeLease>,
    services: &HostServices,
    cause: RegisteredToolCleanupCause,
) {
    if let Some(lease) = lease
        && let Some(bridge) = services.registered_tool_bridge()
    {
        let _ = bridge.close(lease, cause).await;
    }
}

pub(crate) async fn prepare_registered(
    binding: &GrokRegisteredToolBinding,
    plan: &PreflightPlan,
    request_id: &RequestId,
    services: &HostServices,
) -> Result<PendingRegisteredOpen, RuntimeFailure> {
    let host = binding.require_host()?;
    let deadline = binding.require_deadline()?;
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
    let environment = declared_environment(host.approved_environment(recipe.environment()
    ).ok_or_else(|| {
        failure(
            "swallowtail.grok.acp.registered_tool.environment_unresolved",
            "Grok Build ACP registered-tool open requires a host-approved courier environment",
        )
    })?)?;
    RegisteredToolReadiness::evaluate(services, binding.selection())
        .require_ready()
        .map_err(RegisteredToolFailure::into_runtime_failure)?;
    let scope = ScopeId::new(format!("grok-acp:registered-tool:{}", request_id.as_str()))
        .map_err(|_| registered_identity_rejected())?;
    let turn = RuntimeTurnId::new(format!("grok-acp:registered:{}", request_id.as_str()))
        .map_err(|_| registered_identity_rejected())?;
    let prepared = binding.preparation().prepare(
        services,
        plan.instance_id().clone(),
        scope,
        turn,
        deadline,
    )?;
    let lease = prepared.open().await?;
    let launch = host.registered_tool_proxy_launch(&lease)?;
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
        claimed: false,
    })
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
