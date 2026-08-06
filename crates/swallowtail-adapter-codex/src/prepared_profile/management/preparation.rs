use super::{CodexPreparedArchive, CodexPreparedDelete, CodexPreparedRestore};
use crate::prepared_profile::input::CodexSessionManagementInput;
use crate::prepared_profile::plan::{
    build_plan, descriptor, failure, instance_with_capabilities, require_driver, requirements,
};
use crate::selection::classify_lifecycle_version;
use crate::{CodexPreparedDriver, CodexPreparedIntegration};
use swallowtail_core::{
    CapabilityProfile, CapabilityRequirement, DriverRole, HarnessConfigurationPosture,
    HostServiceKind, OperationShape, ProviderSessionActivityEvidence,
    ProviderSessionCancellationPosture, ProviderSessionInitialStateRequirement,
    ProviderSessionManagementAction,
};
use swallowtail_runtime::{
    ArchiveProviderSessionRequest, DeleteProviderSessionRequest, PreparationFailure,
    PreparedProviderSessionManagementEvidence, ProviderSessionManagementAgreement,
    ProviderSessionManagementPlan, RestoreProviderSessionRequest,
};

impl CodexPreparedIntegration {
    /// Prepares archival of one exact inactive thread.
    pub fn prepare_archive_session(
        &self,
        input: CodexSessionManagementInput,
    ) -> Result<CodexPreparedArchive, PreparationFailure> {
        let PreparedManagement {
            environment,
            request_id,
            plan,
        } = self.prepare_management(ProviderSessionManagementAction::Archive, input)?;
        let request = ArchiveProviderSessionRequest::from_plan(request_id, &plan)
            .map_err(|_| preparation_error("Archive request could not be prepared"))?;
        Ok(CodexPreparedArchive {
            environment,
            evidence: PreparedProviderSessionManagementEvidence::from_plan(plan)?,
            request,
        })
    }

    /// Prepares restoration of one exact archived thread.
    pub fn prepare_restore_session(
        &self,
        input: CodexSessionManagementInput,
    ) -> Result<CodexPreparedRestore, PreparationFailure> {
        let PreparedManagement {
            environment,
            request_id,
            plan,
        } = self.prepare_management(ProviderSessionManagementAction::Restore, input)?;
        let request = RestoreProviderSessionRequest::from_plan(request_id, &plan)
            .map_err(|_| preparation_error("Restore request could not be prepared"))?;
        Ok(CodexPreparedRestore {
            environment,
            evidence: PreparedProviderSessionManagementEvidence::from_plan(plan)?,
            request,
        })
    }

    /// Prepares the strongest qualified deletion action for one inactive thread.
    pub fn prepare_delete_session(
        &self,
        input: CodexSessionManagementInput,
    ) -> Result<CodexPreparedDelete, PreparationFailure> {
        let behavior = classify_lifecycle_version(self.observation().version().version())
            .ok_or_else(|| {
                failure(
                    "swallowtail.codex.preparation.lifecycle_version_unsupported",
                    "Prepared Codex version does not support thread lifecycle management",
                )
            })?
            .behavior;
        let action = behavior.delete_action().ok_or_else(|| {
            failure(
                "swallowtail.codex.preparation.lifecycle_action_unsupported",
                "Prepared Codex version does not support thread deletion",
            )
        })?;
        let PreparedManagement {
            environment,
            request_id,
            plan,
        } = self.prepare_management(action, input)?;
        let request = DeleteProviderSessionRequest::from_plan(request_id, &plan)
            .map_err(|_| preparation_error("Delete request could not be prepared"))?;
        Ok(CodexPreparedDelete {
            environment,
            evidence: PreparedProviderSessionManagementEvidence::from_plan(plan)?,
            request,
        })
    }

    fn prepare_management(
        &self,
        action: ProviderSessionManagementAction,
        input: CodexSessionManagementInput,
    ) -> Result<PreparedManagement, PreparationFailure> {
        require_driver(self, CodexPreparedDriver::AppServer)?;
        let (request_id, binding, deadline, allow_unverified_newer) = input.into_parts();
        let assessment = lifecycle_behavior(self, allow_unverified_newer)?;
        if !assessment.behavior.supports(action) {
            return Err(failure(
                "swallowtail.codex.preparation.lifecycle_action_unsupported",
                "Prepared Codex version does not support this thread lifecycle action",
            ));
        }

        let capability = CapabilityRequirement::new(action.required_capability(), []);
        let instance =
            instance_with_capabilities(self, CapabilityProfile::new([capability.clone()]));
        let mut host_services = vec![HostServiceKind::Task, HostServiceKind::Process];
        if deadline.is_some() {
            host_services.push(HostServiceKind::Time);
        }
        let requirements = requirements(
            self,
            OperationShape::ProviderSessionManagement,
            DriverRole::ProviderSessionManagement,
            host_services,
            [capability],
        )
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
        let preflight = build_plan(self, &descriptor(self), &instance, None, &requirements)?;
        let agreement = ProviderSessionManagementAgreement::new(
            binding,
            action,
            initial_state(action),
            assessment.behavior.affected_scope(action),
            ProviderSessionActivityEvidence::CallerAssertedInactive,
            ProviderSessionCancellationPosture::BeforeDispatchOnly,
            deadline,
        );
        let plan = ProviderSessionManagementPlan::new(preflight, agreement).map_err(|_| {
            failure(
                "swallowtail.codex.preparation.lifecycle_binding_mismatch",
                "Codex thread management binding does not match this prepared integration",
            )
        })?;
        Ok(PreparedManagement {
            environment: self.environment().clone(),
            request_id,
            plan,
        })
    }
}

struct PreparedManagement {
    environment: swallowtail_runtime::EnvironmentRef,
    request_id: swallowtail_runtime::RequestId,
    plan: ProviderSessionManagementPlan,
}

fn lifecycle_behavior(
    prepared: &CodexPreparedIntegration,
    allow_unverified_newer: bool,
) -> Result<crate::selection::CodexLifecycleAssessment, PreparationFailure> {
    let assessment = classify_lifecycle_version(prepared.observation().version().version())
        .ok_or_else(|| {
            failure(
                "swallowtail.codex.preparation.lifecycle_version_unsupported",
                "Prepared Codex version does not support thread lifecycle management",
            )
        })?;
    if assessment.unverified_newer && !allow_unverified_newer {
        return Err(failure(
            "swallowtail.codex.preparation.lifecycle_unverified_newer",
            "Newer unverified Codex lifecycle execution requires explicit acceptance",
        ));
    }
    Ok(assessment)
}

const fn initial_state(
    action: ProviderSessionManagementAction,
) -> ProviderSessionInitialStateRequirement {
    match action {
        ProviderSessionManagementAction::Archive => {
            ProviderSessionInitialStateRequirement::Unarchived
        }
        ProviderSessionManagementAction::Restore => {
            ProviderSessionInitialStateRequirement::Archived
        }
        ProviderSessionManagementAction::Delete(_) => {
            ProviderSessionInitialStateRequirement::UnarchivedOrArchived
        }
    }
}

fn preparation_error(message: &'static str) -> PreparationFailure {
    failure(
        "swallowtail.codex.preparation.lifecycle_request_invalid",
        message,
    )
}
