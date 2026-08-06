use super::{
    KimiLocalServerPreparedIntegration, KimiLocalServerSessionManagementInput, preparation_failure,
};
use swallowtail_core::{
    AccessRequirement, Capability, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    CredentialState, Diagnostic, DriverRole, EndpointAuthorization, EntitlementState,
    ExecutionLayer, OperationRequirements, OperationShape, PreflightContext,
    ProviderSessionActivityEvidence, ProviderSessionAffectedScope,
    ProviderSessionCancellationPosture, ProviderSessionInitialStateRequirement,
    ProviderSessionManagementAction, RuntimeReadiness, preflight,
};
use swallowtail_runtime::{
    ArchiveProviderSessionRequest, BoxFuture, HostServices, PreparationFailure, PreparationStage,
    PreparedProviderSessionManagementEvidence, ProviderSessionManagementAgreement,
    ProviderSessionManagementDriver, ProviderSessionManagementOutcome,
    ProviderSessionManagementPlan, RestoreProviderSessionRequest, RuntimeFailure,
};

#[derive(Clone, Debug)]
/// Prepared archive operation for one inactive Kimi local-server session.
pub struct KimiLocalServerPreparedArchive {
    evidence: PreparedProviderSessionManagementEvidence,
    request: ArchiveProviderSessionRequest,
}

impl KimiLocalServerPreparedArchive {
    /// Returns portable evidence for the prepared management operation.
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionManagementEvidence {
        &self.evidence
    }

    /// Returns the exact management plan.
    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionManagementPlan {
        self.evidence.plan()
    }

    /// Returns the bound archive request.
    #[must_use]
    pub const fn request(&self) -> &ArchiveProviderSessionRequest {
        &self.request
    }

    /// Executes the prepared archive operation.
    pub fn execute(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        let driver = super::super::KimiLocalServerDriver::new();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.archive_session(plan, request, services).await })
    }
}

#[derive(Clone, Debug)]
/// Prepared restore operation for one inactive Kimi local-server session.
pub struct KimiLocalServerPreparedRestore {
    evidence: PreparedProviderSessionManagementEvidence,
    request: RestoreProviderSessionRequest,
}

impl KimiLocalServerPreparedRestore {
    /// Returns portable evidence for the prepared management operation.
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionManagementEvidence {
        &self.evidence
    }

    /// Returns the exact management plan.
    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionManagementPlan {
        self.evidence.plan()
    }

    /// Returns the bound restore request.
    #[must_use]
    pub const fn request(&self) -> &RestoreProviderSessionRequest {
        &self.request
    }

    /// Executes the prepared restore operation.
    pub fn execute(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        let driver = super::super::KimiLocalServerDriver::new();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.restore_session(plan, request, services).await })
    }
}

impl KimiLocalServerPreparedIntegration {
    /// Prepares archival of one exact inactive local-server session.
    pub fn prepare_archive_session(
        &self,
        input: KimiLocalServerSessionManagementInput,
    ) -> Result<KimiLocalServerPreparedArchive, PreparationFailure> {
        let (plan, request_id) =
            prepare_action(self, input, ProviderSessionManagementAction::Archive)?;
        let request =
            ArchiveProviderSessionRequest::from_plan(request_id, &plan).map_err(|_| {
                preparation_failure(
                    PreparationStage::Preflight,
                    "swallowtail.kimi.local_server.preparation.request_invalid",
                    "Kimi local-server archive request could not be prepared",
                )
            })?;
        Ok(KimiLocalServerPreparedArchive {
            evidence: PreparedProviderSessionManagementEvidence::from_plan(plan)?,
            request,
        })
    }

    /// Prepares restoration of one exact archived local-server session.
    pub fn prepare_restore_session(
        &self,
        input: KimiLocalServerSessionManagementInput,
    ) -> Result<KimiLocalServerPreparedRestore, PreparationFailure> {
        let (plan, request_id) =
            prepare_action(self, input, ProviderSessionManagementAction::Restore)?;
        let request =
            RestoreProviderSessionRequest::from_plan(request_id, &plan).map_err(|_| {
                preparation_failure(
                    PreparationStage::Preflight,
                    "swallowtail.kimi.local_server.preparation.request_invalid",
                    "Kimi local-server restore request could not be prepared",
                )
            })?;
        Ok(KimiLocalServerPreparedRestore {
            evidence: PreparedProviderSessionManagementEvidence::from_plan(plan)?,
            request,
        })
    }
}

fn prepare_action(
    prepared: &KimiLocalServerPreparedIntegration,
    input: KimiLocalServerSessionManagementInput,
    action: ProviderSessionManagementAction,
) -> Result<
    (
        ProviderSessionManagementPlan,
        swallowtail_runtime::RequestId,
    ),
    PreparationFailure,
> {
    let KimiLocalServerSessionManagementInput {
        request_id,
        binding,
        deadline,
        allow_unverified_newer,
    } = input;
    if !prepared.server().is_qualified() && !allow_unverified_newer {
        return Err(preparation_failure(
            PreparationStage::CompatibilityClassification,
            "swallowtail.kimi.local_server.preparation.lifecycle_unverified_newer",
            "Newer unverified Kimi local-server lifecycle requires explicit acceptance",
        ));
    }
    let capability = CapabilityRequirement::new(action.required_capability(), []);
    let instance =
        instance_with_capabilities(prepared, CapabilityProfile::new([capability.clone()]));
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::ProviderSessionManagement,
        DriverRole::ProviderSessionManagement,
        prepared.instance().execution_host_id().clone(),
        AccessRequirement::new(prepared.access_profile().id().clone())
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([prepared.access_profile().support_authority()]),
    )
    .with_ownership_modes([prepared.instance().ownership()])
    .with_host_services(
        crate::kimi_local_server_descriptor()
            .required_host_services(DriverRole::ProviderSessionManagement),
    )
    .with_capabilities([capability])
    .with_interface_versions([prepared.server().binding().clone()]);
    let descriptor = crate::kimi_local_server_descriptor();
    let preflight = preflight(
        &PreflightContext::new(
            &descriptor,
            &instance,
            prepared.access_profile(),
            prepared.access_evidence().status(),
            prepared.available_host_services(),
        ),
        &requirements,
    )
    .map_err(|error| {
        PreparationFailure::new(
            PreparationStage::Preflight,
            Diagnostic::new(error.diagnostic().clone()),
        )
    })?;
    let initial_state = match action {
        ProviderSessionManagementAction::Archive => {
            ProviderSessionInitialStateRequirement::Unarchived
        }
        ProviderSessionManagementAction::Restore => {
            ProviderSessionInitialStateRequirement::Archived
        }
        ProviderSessionManagementAction::Delete(_) => unreachable!("Kimi delete is unsupported"),
    };
    let agreement = ProviderSessionManagementAgreement::new(
        binding,
        action,
        initial_state,
        ProviderSessionAffectedScope::TargetOnly,
        ProviderSessionActivityEvidence::CallerAssertedInactive,
        ProviderSessionCancellationPosture::BeforeDispatchOnly,
        deadline,
    );
    let plan = ProviderSessionManagementPlan::new(preflight, agreement).map_err(|_| {
        preparation_failure(
            PreparationStage::Preflight,
            "swallowtail.kimi.local_server.preparation.lifecycle_binding_mismatch",
            "Kimi local-server lifecycle binding does not match this prepared integration",
        )
    })?;
    Ok((plan, request_id))
}

pub(super) fn instance_with_capabilities(
    prepared: &KimiLocalServerPreparedIntegration,
    capabilities: CapabilityProfile,
) -> ConfiguredInstance {
    let base = prepared.instance();
    ConfiguredInstance::new(
        base.id().clone(),
        base.revision().clone(),
        base.driver_id().clone(),
        base.execution_host_id().clone(),
        base.target_reference().clone(),
        base.ownership(),
        base.access_profile_id().clone(),
        base.support_authority(),
        base.protocol_facade_id().clone(),
        base.policy_id().clone(),
        capabilities,
    )
    .with_interface_versions(base.interface_versions().cloned())
}

pub(crate) fn lifecycle_capabilities() -> CapabilityProfile {
    CapabilityProfile::new([
        CapabilityRequirement::new(Capability::ProviderSessionArchive, []),
        CapabilityRequirement::new(Capability::ProviderSessionRestore, []),
    ])
}
