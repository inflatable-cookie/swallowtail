use super::plan::failure;
use crate::OpenCodePreparedIntegration;
use swallowtail_core::{
    AccessRequirement, CapabilityRequirement, CredentialState, DriverRole, EndpointAuthorization,
    EntitlementState, ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation, HostServiceKind,
    OperationRequirements, OperationShape, RuntimeReadiness, SessionAccessPolicy,
    SessionProviderStatePolicy,
};
use swallowtail_runtime::PreparationFailure;

mod catalogue;
mod import;
mod reconciliation;

pub use catalogue::OpenCodePreparedSessionCatalogue;
pub use import::OpenCodePreparedSessionImport;
pub use reconciliation::OpenCodePreparedSessionReconciliation;

pub(super) fn require_qualified(prepared: &OpenCodePreparedIntegration) -> Result<(), PreparationFailure> {
    if prepared.server().is_qualified() {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.opencode.preparation.session_catalogue_version_unsupported",
            "OpenCode session catalogue and import require a qualified server version",
        ))
    }
}

pub(super) fn require_reconciliation_qualified(
    prepared: &OpenCodePreparedIntegration,
) -> Result<(), PreparationFailure> {
    if prepared.server().is_qualified() {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.opencode.preparation.session_reconciliation_version_unsupported",
            "OpenCode session reconciliation requires a qualified server version",
        ))
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn provider_session_requirements(
    prepared: &OpenCodePreparedIntegration,
    shape: OperationShape,
    role: DriverRole,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
    require_model: bool,
    include_time: bool,
    access: Option<SessionAccessPolicy>,
) -> OperationRequirements {
    let mut services = vec![
        HostServiceKind::Task,
        HostServiceKind::BlockingWork,
        HostServiceKind::Network,
        HostServiceKind::Credential,
        HostServiceKind::WorkingResource,
    ];
    if include_time {
        services.push(HostServiceKind::Time);
    }
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        shape,
        role,
        prepared.instance().execution_host_id().clone(),
        AccessRequirement::new(prepared.access_profile().id().clone())
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([prepared.access_profile().support_authority()]),
    )
    .with_ownership_modes([prepared.instance().ownership()])
    .with_host_services(services)
    .with_capabilities(capabilities)
    .with_interface_versions([prepared.server().binding().clone()])
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let requirements = access.map_or(requirements.clone(), |policy| {
        requirements
            .with_session_access_policy(policy)
            .with_session_provider_state_policy(
                SessionProviderStatePolicy::DurableProviderSessionPreserved,
            )
    });
    if require_model {
        requirements.require_model_route()
    } else {
        requirements
    }
}
