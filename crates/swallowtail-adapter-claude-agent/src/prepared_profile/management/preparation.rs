use super::ClaudeAgentPreparedDelete;
use crate::ClaudeAgentPreparedIntegration;
use crate::prepared_profile::input::ClaudeAgentSessionManagementInput;
use crate::prepared_profile::plan::{build_plan, failure, instance_with_capabilities};
use swallowtail_core::{
    AccessRequirement, Capability, CapabilityProfile, CapabilityRequirement, CredentialState,
    DriverRole, EndpointAuthorization, EntitlementState, ExecutionLayer,
    HarnessConfigurationPosture, HarnessIsolation, HostServiceKind, OperationRequirements,
    OperationShape, ProviderSessionActivityEvidence, ProviderSessionAffectedScope,
    ProviderSessionCancellationPosture, ProviderSessionDeletionStrength,
    ProviderSessionInitialStateRequirement, ProviderSessionManagementAction, RuntimeReadiness,
};
use swallowtail_runtime::{
    DeleteProviderSessionRequest, PreparationFailure, PreparedProviderSessionManagementEvidence,
    ProviderSessionManagementAgreement, ProviderSessionManagementPlan,
};

impl ClaudeAgentPreparedIntegration {
    pub fn prepare_delete_session(
        &self,
        input: ClaudeAgentSessionManagementInput,
    ) -> Result<ClaudeAgentPreparedDelete, PreparationFailure> {
        let (request_id, binding, deadline, allow_unverified_newer) = input.into_parts();
        if !self.observation().is_qualified() && !allow_unverified_newer {
            return Err(failure(
                "swallowtail.claude_agent.preparation.lifecycle_unverified_newer",
                "Newer unverified Claude Agent deletion requires explicit acceptance",
            ));
        }

        let action = ProviderSessionManagementAction::Delete(
            ProviderSessionDeletionStrength::ProviderDataDeleted,
        );
        let capability = CapabilityRequirement::new(Capability::ProviderSessionDelete, []);
        let instance =
            instance_with_capabilities(self, CapabilityProfile::new([capability.clone()]));
        let mut host_services = vec![
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::Credential,
            HostServiceKind::WorkingResource,
            HostServiceKind::WorkingResourceIo,
        ];
        if deadline.is_some() {
            host_services.push(HostServiceKind::Time);
        }
        let requirements = OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            OperationShape::ProviderSessionManagement,
            DriverRole::ProviderSessionManagement,
            self.instance().execution_host_id().clone(),
            access_requirement(self),
        )
        .with_ownership_modes([self.instance().ownership()])
        .with_host_services(host_services)
        .with_capabilities([capability])
        .with_interface_versions([self.observation().version().clone()])
        .with_harness_isolation(HarnessIsolation::AmbientHost)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
        let preflight = build_plan(self, &instance, None, &requirements)?;
        let agreement = ProviderSessionManagementAgreement::new(
            binding,
            action,
            ProviderSessionInitialStateRequirement::UnarchivedOrArchived,
            ProviderSessionAffectedScope::ProviderDefinedDescendants,
            ProviderSessionActivityEvidence::CallerAssertedInactive,
            ProviderSessionCancellationPosture::BeforeDispatchOnly,
            deadline,
        );
        let plan = ProviderSessionManagementPlan::new(preflight, agreement).map_err(|_| {
            failure(
                "swallowtail.claude_agent.preparation.lifecycle_binding_mismatch",
                "Claude Agent session-management binding does not match this prepared integration",
            )
        })?;
        let request = DeleteProviderSessionRequest::from_plan(request_id, &plan).map_err(|_| {
            failure(
                "swallowtail.claude_agent.preparation.lifecycle_request_invalid",
                "Claude Agent delete request could not be prepared",
            )
        })?;
        Ok(ClaudeAgentPreparedDelete {
            environment: self.environment().clone(),
            credential: self
                .access_profile()
                .credential_reference()
                .expect("prepared Claude Agent access has one credential reference")
                .clone(),
            evidence: PreparedProviderSessionManagementEvidence::from_plan(plan)?,
            request,
        })
    }
}

fn access_requirement(prepared: &ClaudeAgentPreparedIntegration) -> AccessRequirement {
    AccessRequirement::new(prepared.access_profile().id().clone())
        .with_credential_states([CredentialState::Ready])
        .with_entitlement_states([EntitlementState::Available])
        .with_endpoint_authorizations([EndpointAuthorization::Allowed])
        .with_runtime_readiness([RuntimeReadiness::Ready])
        .with_support_authorities([prepared.access_profile().support_authority()])
}
