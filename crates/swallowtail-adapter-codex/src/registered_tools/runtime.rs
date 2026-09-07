//! Session-scoped registered-tool state for one Codex app-server thread.
//!
//! Binding happens at session open, before the provider thread starts, so a
//! missing port, an absent task service, or a declaration the session never
//! transported fails before any provider work. Opening the lease happens per
//! turn attempt: one Desktop-admitted attempt maps to exactly one Swallowtail
//! operation, lease, and lease generation.

use super::binding::CodexRegisteredToolBinding;
use super::turn::CodexRegisteredTurn;
use crate::rpc::{RpcConnection, failure};
use serde_json::Value;
use std::sync::{Arc, Weak};
use swallowtail_core::ConfiguredInstanceId;
use swallowtail_runtime::{
    Deadline, HostServices, MonotonicInstant, RegisteredToolBridgeHostService,
    RegisteredToolFailure, RegisteredToolReadiness, RuntimeFailure, RuntimeTurnId, ScopeId,
    ScopedTaskService,
};

/// Registered-tool state carried by one prepared and opened Codex session.
pub(crate) struct CodexRegisteredToolRuntime {
    binding: CodexRegisteredToolBinding,
    port: Arc<dyn RegisteredToolBridgeHostService>,
    tasks: Arc<dyn ScopedTaskService>,
}

impl CodexRegisteredToolRuntime {
    /// Binds one qualified selection to the session's host services.
    ///
    /// `transported` is the exact dynamic tool payload this session is about to
    /// send. It must reproduce the qualified registration, and the selection
    /// must pass the typed readiness gate against this exact host topology.
    /// Both checks run before any process, connection, or provider work, so a
    /// substituted schema or an unready topology never reaches the provider.
    pub(crate) fn bind(
        binding: CodexRegisteredToolBinding,
        transported: &[Value],
        services: &HostServices,
    ) -> Result<Self, RuntimeFailure> {
        binding.verify_transported(transported)?;
        RegisteredToolReadiness::evaluate(services, binding.selection())
            .require_ready()
            .map_err(RegisteredToolFailure::into_runtime_failure)?;
        let port = services.registered_tool_bridge().cloned().ok_or_else(|| {
            failure(
                "swallowtail.codex.app_server.registered_bridge_missing",
                "Codex registered dynamic tools require the registered-tool bridge port",
            )
        })?;
        let tasks = services.task().cloned().ok_or_else(|| {
            failure(
                "swallowtail.codex.app_server.registered_task_service_missing",
                "Codex registered dynamic tools require a scoped task service",
            )
        })?;
        Ok(Self {
            binding,
            port,
            tasks,
        })
    }

    /// Opens one lease bound to exactly one Codex turn attempt.
    ///
    /// The lease reaches ready before `turn/start` is sent, so a registered
    /// failure never leaves a provider turn running without its tool seam.
    pub(crate) async fn open_turn(
        &self,
        services: &HostServices,
        configured_instance: ConfiguredInstanceId,
        turn: RuntimeTurnId,
        deadline: Option<Deadline>,
        connection: Weak<RpcConnection>,
    ) -> Result<Arc<CodexRegisteredTurn>, RuntimeFailure> {
        let scope = ScopeId::new(format!(
            "codex-app-server:registered-tool:{}",
            turn.as_str()
        ))
        .expect("runtime turn id produces a valid scope id");
        // Absent a turn deadline the operation stays bounded by the kernel's
        // own maximum call duration; it never becomes unbounded work.
        let deadline =
            deadline.unwrap_or_else(|| Deadline::at(MonotonicInstant::from_ticks(u64::MAX)));
        let prepared = self.binding.preparation().prepare(
            services,
            configured_instance,
            scope.clone(),
            turn,
            deadline,
        )?;
        let lease = prepared.open().await?;
        let port = services
            .registered_tool_bridge()
            .cloned()
            .unwrap_or_else(|| Arc::clone(&self.port));
        Ok(Arc::new(CodexRegisteredTurn::new(
            self.binding.clone(),
            port,
            Arc::clone(&self.tasks),
            connection,
            scope,
            lease,
        )))
    }
}
