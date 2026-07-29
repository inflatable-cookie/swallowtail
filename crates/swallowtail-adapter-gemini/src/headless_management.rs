use crate::headless_pump::{contains_exact_session_id, run_management_process};
use crate::{GeminiHeadlessDriver, GeminiHeadlessPreparedIntegration};
use swallowtail_core::{
    AccessRequirement, Capability, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    CredentialState, Diagnostic, DriverRole, EndpointAuthorization, EntitlementState,
    ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation, HostServiceKind,
    OperationRequirements, OperationShape, ProviderSessionActivityEvidence,
    ProviderSessionAffectedScope, ProviderSessionCancellationPosture,
    ProviderSessionDeletionStrength, ProviderSessionEffectTruth,
    ProviderSessionInitialStateRequirement, ProviderSessionManagementAction,
    ProviderSessionManagementEffect, RuntimeReadiness,
};
use swallowtail_runtime::{
    ArchiveProviderSessionRequest, BoxFuture, Deadline, DeleteProviderSessionRequest, HostServices,
    PreparationFailure, PreparationStage, PreparedProviderSessionManagementEvidence,
    ProviderSessionManagementBinding, ProviderSessionManagementDriver,
    ProviderSessionManagementOutcome, ProviderSessionManagementPlan, RequestId,
    RestoreProviderSessionRequest, RuntimeFailure, validate_provider_session_management_request,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeminiHeadlessSessionManagementInput {
    request_id: RequestId,
    binding: ProviderSessionManagementBinding,
    deadline: Deadline,
}

impl GeminiHeadlessSessionManagementInput {
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        binding: ProviderSessionManagementBinding,
        deadline: Deadline,
    ) -> Self {
        Self {
            request_id,
            binding,
            deadline,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GeminiHeadlessPreparedDelete {
    environment: swallowtail_runtime::EnvironmentRef,
    credential: swallowtail_core::CredentialRef,
    evidence: PreparedProviderSessionManagementEvidence,
    request: DeleteProviderSessionRequest,
}

impl GeminiHeadlessPreparedDelete {
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionManagementEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionManagementPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &DeleteProviderSessionRequest {
        &self.request
    }

    pub fn execute(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        let driver = GeminiHeadlessDriver::new(self.environment.clone(), self.credential.clone());
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.delete_session(plan, request, services).await })
    }
}

impl GeminiHeadlessPreparedIntegration {
    pub fn prepare_delete_session(
        &self,
        input: GeminiHeadlessSessionManagementInput,
    ) -> Result<GeminiHeadlessPreparedDelete, PreparationFailure> {
        if !self.observation().is_qualified() {
            return Err(PreparationFailure::new(
                PreparationStage::CompatibilityClassification,
                Diagnostic::new(swallowtail_core::SafeDiagnostic::new(
                    "swallowtail.gemini.headless.management_version_unqualified",
                    "Gemini transcript deletion requires an exact qualified CLI version",
                )),
            ));
        }
        let GeminiHeadlessSessionManagementInput {
            request_id,
            binding,
            deadline,
        } = input;
        let capability = CapabilityRequirement::new(Capability::ProviderSessionDelete, []);
        let base = self.instance();
        let instance = ConfiguredInstance::new(
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
            CapabilityProfile::new([capability.clone()]),
        )
        .with_interface_versions(base.interface_versions().cloned())
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
        let requirements = OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            OperationShape::ProviderSessionManagement,
            DriverRole::ProviderSessionManagement,
            self.instance().execution_host_id().clone(),
            AccessRequirement::new(self.access_profile().id().clone())
                .with_credential_states([CredentialState::Ready])
                .with_entitlement_states([EntitlementState::Available])
                .with_endpoint_authorizations([EndpointAuthorization::Allowed])
                .with_runtime_readiness([RuntimeReadiness::Ready])
                .with_support_authorities([self.access_profile().support_authority()]),
        )
        .with_ownership_modes([self.instance().ownership()])
        .with_host_services([
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::Time,
        ])
        .with_capabilities([capability])
        .with_interface_versions([self.observation().version().clone()])
        .with_harness_isolation(HarnessIsolation::AmbientHost)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
        let preflight = swallowtail_core::preflight(
            &swallowtail_core::PreflightContext::new(
                &crate::gemini_headless_descriptor(),
                &instance,
                self.access_profile(),
                self.access_evidence().status(),
                self.available_host_services(),
            ),
            &requirements,
        )
        .map_err(|error| {
            PreparationFailure::new(
                PreparationStage::Preflight,
                Diagnostic::new(error.diagnostic().clone()),
            )
        })?;
        let action = ProviderSessionManagementAction::Delete(
            ProviderSessionDeletionStrength::HistoryRemoved,
        );
        let agreement = swallowtail_runtime::ProviderSessionManagementAgreement::new(
            binding,
            action,
            ProviderSessionInitialStateRequirement::UnarchivedOrArchived,
            ProviderSessionAffectedScope::ProviderDefinedDescendants,
            ProviderSessionActivityEvidence::CallerAssertedInactive,
            ProviderSessionCancellationPosture::BeforeDispatchOnly,
            Some(deadline),
        );
        let plan = ProviderSessionManagementPlan::new(preflight, agreement).map_err(|error| {
            PreparationFailure::new(
                PreparationStage::Preflight,
                Diagnostic::new(error.diagnostic().clone()),
            )
        })?;
        let request =
            DeleteProviderSessionRequest::from_plan(request_id, &plan).map_err(|error| {
                PreparationFailure::new(
                    PreparationStage::Preflight,
                    Diagnostic::new(error.diagnostic().clone()),
                )
            })?;
        Ok(GeminiHeadlessPreparedDelete {
            environment: self.environment().clone(),
            credential: self
                .access_profile()
                .credential_reference()
                .expect("prepared Gemini headless credential is present")
                .clone(),
            evidence: PreparedProviderSessionManagementEvidence::from_plan(plan)?,
            request,
        })
    }
}

impl ProviderSessionManagementDriver for GeminiHeadlessDriver {
    fn archive_session(
        &self,
        _plan: ProviderSessionManagementPlan,
        _request: ArchiveProviderSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        Box::pin(async { Err(unsupported_management()) })
    }

    fn restore_session(
        &self,
        _plan: ProviderSessionManagementPlan,
        _request: RestoreProviderSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        Box::pin(async { Err(unsupported_management()) })
    }

    fn delete_session(
        &self,
        plan: ProviderSessionManagementPlan,
        request: DeleteProviderSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        Box::pin(async move {
            validate_provider_session_management_request(&plan, request.agreement(), &services)?;
            crate::headless::validate_headless_plan(plan.preflight(), &self.credential)?;
            let agreement = request.agreement();
            let action = agreement.action();
            let binding = agreement.binding();
            let provider_id = binding.provider_session_ref().as_provider_value();
            let working_resource = binding.working_resource().ok_or_else(|| {
                RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                    "swallowtail.gemini.headless.management_resource_missing",
                    "Gemini transcript deletion requires its bound working resource",
                ))
            })?;
            let time = services.time().expect("validated time service");
            let deadline = agreement.deadline().expect("prepared deadline is present");
            if request.cancellation().is_requested() || time.now() >= deadline.instant() {
                return Ok(ProviderSessionManagementOutcome::new(
                    binding.clone(),
                    ProviderSessionManagementEffect::failed_before_effect(action),
                ));
            }
            let process = services.process().expect("validated process service");
            let executable = swallowtail_runtime::ExecutableRef::from_instance_target(
                plan.preflight().instance_target_ref(),
            );
            let mut deadline_wait = Some(time.wait_until(deadline));
            let delete = run_management_process(
                process.as_ref(),
                &executable,
                &self.environment,
                working_resource,
                crate::headless_command::delete_session_arguments(provider_id),
                "delete",
                &mut deadline_wait,
            )
            .await;
            let list = run_management_process(
                process.as_ref(),
                &executable,
                &self.environment,
                working_resource,
                crate::headless_command::list_sessions_arguments(),
                "reconcile",
                &mut deadline_wait,
            )
            .await;
            let effect = classify(action, provider_id, delete.as_ref(), list.as_ref());
            let mut outcome = ProviderSessionManagementOutcome::new(binding.clone(), effect);
            if effect.truth() == ProviderSessionEffectTruth::UnconfirmedAfterEffect {
                outcome = outcome.with_diagnostic(swallowtail_core::SafeDiagnostic::new(
                    "swallowtail.gemini.headless.transcript_deletion_unconfirmed",
                    "Gemini stored-transcript deletion could not be confirmed",
                ));
            }
            Ok(outcome)
        })
    }
}

fn classify(
    action: ProviderSessionManagementAction,
    provider_id: &str,
    delete: Result<&crate::headless_pump::ManagementProcessResult, &RuntimeFailure>,
    list: Result<&crate::headless_pump::ManagementProcessResult, &RuntimeFailure>,
) -> ProviderSessionManagementEffect {
    let absent = list.is_ok_and(|result| {
        result.exit.success() && !contains_exact_session_id(&result.combined(), provider_id)
    });
    let delete_completed = delete.is_ok();
    let delete_stderr = delete
        .as_ref()
        .ok()
        .map(|result| result.stderr.as_slice())
        .unwrap_or_default();
    if delete_completed
        && absent
        && delete_stderr
            .windows(b"Invalid session identifier".len())
            .any(|window| window == b"Invalid session identifier")
    {
        ProviderSessionManagementEffect::target_already_absent(
            ProviderSessionDeletionStrength::HistoryRemoved,
            ProviderSessionAffectedScope::ProviderDefinedDescendants,
        )
    } else if delete_completed && absent {
        ProviderSessionManagementEffect::applied(
            action,
            ProviderSessionAffectedScope::ProviderDefinedDescendants,
        )
    } else if delete_stderr
        .windows(b"Cannot delete the current active session".len())
        .any(|window| window == b"Cannot delete the current active session")
    {
        ProviderSessionManagementEffect::failed_before_effect(action)
    } else {
        ProviderSessionManagementEffect::unconfirmed_after_effect(action)
    }
}

fn unsupported_management() -> RuntimeFailure {
    RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
        "swallowtail.gemini.headless.management_unsupported",
        "Gemini headless supports only stored-transcript deletion",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::headless_pump::ManagementProcessResult;
    use swallowtail_runtime::ProcessExit;

    const SESSION: &str = "swallowtail-66697874757265";

    fn result(stdout: &str, stderr: &str, success: bool) -> ManagementProcessResult {
        ManagementProcessResult {
            exit: ProcessExit::new(success, Some(if success { 0 } else { 1 })),
            stdout: stdout.as_bytes().to_vec(),
            stderr: stderr.as_bytes().to_vec(),
        }
    }

    fn action() -> ProviderSessionManagementAction {
        ProviderSessionManagementAction::Delete(ProviderSessionDeletionStrength::HistoryRemoved)
    }

    #[test]
    fn reconciliation_controls_gemini_history_removal_truth() {
        let deleted = result("Deleted session 1: private", "", true);
        let absent = result("No sessions found", "", true);
        assert_eq!(
            classify(action(), SESSION, Ok(&deleted), Ok(&absent)).truth(),
            ProviderSessionEffectTruth::Applied
        );

        let already = result("", "Invalid session identifier private", true);
        assert_eq!(
            classify(action(), SESSION, Ok(&already), Ok(&absent)).truth(),
            ProviderSessionEffectTruth::TargetAlreadyAbsent
        );

        let active = result("", "Cannot delete the current active session.", true);
        let present = result(&format!("1 {SESSION} private"), "", true);
        assert_eq!(
            classify(action(), SESSION, Ok(&active), Ok(&present)).truth(),
            ProviderSessionEffectTruth::FailedBeforeEffect
        );

        assert_eq!(
            classify(action(), SESSION, Ok(&deleted), Ok(&present)).truth(),
            ProviderSessionEffectTruth::UnconfirmedAfterEffect
        );
        assert_eq!(
            classify(
                action(),
                SESSION,
                Ok(&deleted),
                Err(&RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                    "fixture.failed",
                    "Fixture reconciliation failed",
                ),)),
            )
            .truth(),
            ProviderSessionEffectTruth::UnconfirmedAfterEffect
        );
    }
}
