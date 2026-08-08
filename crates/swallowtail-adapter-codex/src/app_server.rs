use crate::rpc::{RpcConnection, failure};
use crate::selection::{CodexAppServerBehavior, classify_app_server_plan, codex_app_server_claim};
use crate::session_access::{CodexSessionAccess, codex_provider_request_extensions};
use crate::session_input::CodexSessionInput;
use crate::session_open::PendingSessionOpen;
use crate::session_replay::{MAXIMUM_REPLAY_BYTES, MAXIMUM_REPLAY_ITEMS};
use serde_json::Value;
use std::future::poll_fn;
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, Capability, CapabilityConstraint, DriverDescriptor,
    DriverRole, ExecutionLayer, HarnessConfigurationPosture, HostServiceKind, IntegrationFamilyId,
    ModelCatalogEntry, ModelId, ModelMetadata, OperationShape, PreflightPlan, ReasoningMetadata,
    ReasoningMode, ResourceAccess, TransportFamilyId,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, EnvironmentRef, ExecutableRef, HostServices,
    InteractiveSessionDriver, InteractiveSessionHandle, JoinedTask, LoadSessionRequest,
    LoadedSession, ModelCatalogDriver, ModelCatalogRequest, OpenSessionRequest, ProcessHandle,
    ProcessRequest, RequestId, ResumeSessionRequest, RuntimeFailure, ScopeId, WorkingResourceRef,
    validate_session_plan_agreement,
};

/// Low-level driver for Codex app-server sessions and thread operations.
pub struct CodexAppServerDriver {
    environment: EnvironmentRef,
}

impl CodexAppServerDriver {
    /// Creates an app-server driver using the approved execution environment.
    #[must_use]
    pub const fn new(environment: EnvironmentRef) -> Self {
        Self { environment }
    }
}

#[must_use]
/// Describes Codex's interactive app-server route.
pub fn codex_app_server_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new("swallowtail.codex.app-server").expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("codex").expect("static family id is valid"),
        TransportFamilyId::new("jsonl-rpc-stdio").expect("static transport id is valid"),
    )
    .with_roles([
        DriverRole::Discovery,
        DriverRole::ModelCatalog,
        DriverRole::InteractiveSession,
        DriverRole::ProviderSessionManagement,
        DriverRole::ProviderSessionCatalogue,
        DriverRole::ProviderSessionImport,
        DriverRole::ProviderSessionReconciliation,
        DriverRole::ProviderSessionHistory,
    ])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([
        OperationShape::InteractiveSession,
        OperationShape::ProviderSessionManagement,
        OperationShape::ProviderSessionCatalogue,
        OperationShape::ProviderSessionImport,
        OperationShape::ProviderSessionReconciliation,
        OperationShape::ProviderSessionHistory,
    ])
    .with_required_host_services(
        DriverRole::ModelCatalog,
        [HostServiceKind::Task, HostServiceKind::Process],
    )
    .with_required_host_services(
        DriverRole::InteractiveSession,
        [HostServiceKind::Task, HostServiceKind::Process],
    )
    .with_required_host_services(
        DriverRole::ProviderSessionManagement,
        [HostServiceKind::Task, HostServiceKind::Process],
    )
    .with_required_host_services(
        DriverRole::ProviderSessionCatalogue,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::WorkingResource,
        ],
    )
    .with_required_host_services(
        DriverRole::ProviderSessionImport,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::WorkingResource,
        ],
    )
    .with_required_host_services(
        DriverRole::ProviderSessionReconciliation,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::WorkingResource,
        ],
    )
    .with_required_host_services(
        DriverRole::ProviderSessionHistory,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::WorkingResource,
        ],
    )
    .with_required_host_services(
        DriverRole::Discovery,
        [
            HostServiceKind::Task,
            HostServiceKind::Time,
            HostServiceKind::Process,
        ],
    )
    .with_discovery_actions([swallowtail_core::DiscoveryAction::Probe])
    .with_extension_namespaces(codex_provider_request_extensions())
    .with_interface_compatibility(codex_app_server_claim())
}

impl ModelCatalogDriver for CodexAppServerDriver {
    fn list_models(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        Box::pin(async move {
            let behavior = self.validate_plan(&plan)?;
            let deadline = request
                .deadline()
                .map(|deadline| {
                    services
                        .time()
                        .ok_or_else(|| {
                            failure(
                                "swallowtail.codex.app_server.time_service_missing",
                                "Codex model discovery deadline requires a time service",
                            )
                        })
                        .map(|time| time.wait_until(deadline))
                })
                .transpose()?;
            let scope = scope("catalog", request.request_id());
            let (connection, task) = self
                .start_connection(&plan, behavior, scope, None, false, &services)
                .await?;
            let result = match deadline {
                Some(deadline) => {
                    catalog_before_deadline(self.read_catalog(&connection), deadline).await
                }
                None => self.read_catalog(&connection).await,
            };
            let cleanup = close_connection(&connection, task).await;
            match (result, cleanup) {
                (Ok(models), CleanupOutcome::Clean) => Ok(models),
                (Err(error), _) => Err(error),
                (Ok(_), _) => Err(failure(
                    "swallowtail.codex.app_server.catalog_cleanup_failed",
                    "Codex app-server catalog cleanup failed",
                )),
            }
        })
    }
}

async fn catalog_before_deadline<F>(
    catalog: F,
    mut deadline: BoxFuture<'static, swallowtail_runtime::DeadlineObservation>,
) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure>
where
    F: std::future::Future<Output = Result<Vec<ModelCatalogEntry>, RuntimeFailure>>,
{
    let mut catalog = Box::pin(catalog);
    poll_fn(|context| {
        if let Poll::Ready(result) = catalog.as_mut().poll(context) {
            return Poll::Ready(result);
        }
        if deadline.as_mut().poll(context).is_ready() {
            return Poll::Ready(Err(failure(
                "swallowtail.codex.app_server.catalog_timed_out",
                "Codex model discovery timed out",
            )));
        }
        Poll::Pending
    })
    .await
}

include!("app_server/session_role.rs");

fn validate_session_deadline(has_deadline: bool) -> Result<(), RuntimeFailure> {
    if has_deadline {
        Err(unsupported("session deadlines"))
    } else {
        Ok(())
    }
}

fn validate_workspace_behavior(
    behavior: &CodexAppServerBehavior,
    policy: &swallowtail_core::SessionAccessPolicy,
) -> Result<(), RuntimeFailure> {
    if policy.resource_access() == Some(swallowtail_core::ResourceAccess::ReadWrite)
        && !behavior.supports_workspace_roots()
    {
        Err(unsupported(
            "bounded workspace sessions before Codex 0.131.0",
        ))
    } else {
        Ok(())
    }
}

fn validate_app_server_plan(
    plan: &PreflightPlan,
    behavior: CodexAppServerBehavior,
) -> Result<(), RuntimeFailure> {
    if plan.harness_configuration_posture() != Some(HarnessConfigurationPosture::Ambient) {
        return Err(failure(
            "swallowtail.codex.app_server.preflight_mismatch",
            "Codex app-server requires explicit ambient harness configuration agreement",
        ));
    }
    if !behavior.is_legacy() {
        return Ok(());
    }
    for requirement in plan.requirements().capabilities() {
        if requirement.capability() == Capability::ToolCalls
            || (requirement.capability() == Capability::WorkingResource
                && requirement.constraints().any(|constraint| {
                    constraint == &CapabilityConstraint::ResourceAccess(ResourceAccess::ReadWrite)
                }))
        {
            return Err(unsupported(
                "dynamic tools or bounded workspace roots on legacy Codex app-server",
            ));
        }
    }
    if plan.requirements().extension_namespaces().next().is_some() {
        return Err(unsupported("provider requests on legacy Codex app-server"));
    }
    Ok(())
}

include!("app_server/continuity.rs");
