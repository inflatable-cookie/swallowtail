#[path = "profile/plan.rs"]
mod plan;

use super::KimiHeadlessPreparedIntegration;
use super::instance::run_capabilities;
use crate::headless_activity::profile::{activity_profile, with_activity};
use plan::{build_plan, instance_with_capabilities, requirements};
use swallowtail_core::{CapabilityRequirement, ModelRoute, PreflightPlan};
use swallowtail_runtime::{
    BoxFuture, Deadline, HostServices, OperationContent, OperationPolicy, PreparationFailure,
    PreparedOperationEvidence, ProviderRetentionPolicy, RequestId, RunHandle, RuntimeFailure,
    StructuredRunDriver, StructuredRunRequest, WorkingResourceRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer inputs for one prepared Kimi headless run.
pub struct KimiHeadlessRunInput {
    request_id: RequestId,
    model: crate::KimiModelSelection,
    content: OperationContent,
    working_resource: WorkingResourceRef,
    deadline: Deadline,
    managed_recovery_accepted: bool,
}

impl KimiHeadlessRunInput {
    /// Creates a Kimi headless run profile.
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: crate::KimiModelSelection,
        content: OperationContent,
        working_resource: WorkingResourceRef,
        deadline: Deadline,
    ) -> Self {
        Self {
            request_id,
            model,
            content,
            working_resource,
            deadline,
            managed_recovery_accepted: false,
        }
    }

    /// Explicitly accepts provider-managed recovery semantics.
    #[must_use]
    pub const fn accept_managed_recovery(mut self) -> Self {
        self.managed_recovery_accepted = true;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Portable evidence for a prepared Kimi headless run.
pub struct KimiHeadlessPreparedEvidence {
    observation: swallowtail_core::InstalledExecutableObservation,
    environment: swallowtail_runtime::EnvironmentRef,
    credential: swallowtail_core::CredentialRef,
    operation: PreparedOperationEvidence,
}

impl KimiHeadlessPreparedEvidence {
    fn from_prepared(
        prepared: &KimiHeadlessPreparedIntegration,
        plan: PreflightPlan,
        activity_profile: swallowtail_core::ObservableActivityProfile,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            observation: prepared.observation().clone(),
            environment: prepared.environment().clone(),
            credential: prepared
                .access_profile()
                .credential_reference()
                .expect("prepared Kimi headless access has one credential")
                .clone(),
            operation: PreparedOperationEvidence::from_plan_with_activity_profile(
                plan,
                prepared.access_evidence().clone(),
                activity_profile,
            )?,
        })
    }

    /// Returns the qualified installed-executable observation.
    #[must_use]
    pub const fn observation(&self) -> &swallowtail_core::InstalledExecutableObservation {
        &self.observation
    }

    /// Returns the prepared access evidence.
    #[must_use]
    pub const fn access(&self) -> &swallowtail_runtime::PreparedAccessEvidence {
        self.operation.access()
    }

    /// Returns the complete prepared-operation evidence.
    #[must_use]
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    /// Returns the admitted observable-activity profile.
    #[must_use]
    pub const fn observable_activity(&self) -> &swallowtail_core::ObservableActivityProfile {
        self.operation.observable_activity()
    }

    /// Returns the validated preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.operation.plan()
    }

    fn low_level_driver(&self) -> crate::KimiHeadlessDriver {
        crate::KimiHeadlessDriver::new(self.environment.clone(), self.credential.clone())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared one-shot Kimi headless structured run.
pub struct KimiHeadlessPreparedRun {
    evidence: KimiHeadlessPreparedEvidence,
    request: StructuredRunRequest,
}

impl KimiHeadlessPreparedRun {
    /// Returns portable evidence for the prepared run.
    #[must_use]
    pub const fn evidence(&self) -> &KimiHeadlessPreparedEvidence {
        &self.evidence
    }

    /// Returns the validated preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    /// Returns the bound structured-run request.
    #[must_use]
    pub const fn request(&self) -> &StructuredRunRequest {
        &self.request
    }

    /// Creates the low-level headless driver bound to this run.
    #[must_use]
    pub fn low_level_driver(&self) -> crate::KimiHeadlessDriver {
        self.evidence.low_level_driver()
    }

    /// Starts the prepared run with caller-supplied host services.
    pub fn start_run(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.start_run(plan, request, services).await })
    }
}

impl KimiHeadlessPreparedIntegration {
    /// Prepares a structured run through the admitted headless integration.
    pub fn prepare_run(
        &self,
        input: KimiHeadlessRunInput,
    ) -> Result<KimiHeadlessPreparedRun, PreparationFailure> {
        if !input.managed_recovery_accepted {
            return Err(PreparationFailure::new(
                swallowtail_runtime::PreparationStage::Preflight,
                swallowtail_core::Diagnostic::new(swallowtail_core::SafeDiagnostic::new(
                    "swallowtail.kimi.headless.preparation.recovery_agreement_required",
                    "Kimi headless requires explicit managed-recovery acceptance",
                )),
            ));
        }
        let activity = activity_profile(self)?;
        let capabilities = with_activity(run_capabilities(), &activity);
        let instance = instance_with_capabilities(self, capabilities.clone());
        let (route_id, route_revision, model_id) = input.model.into_parts();
        let route = ModelRoute::new(
            route_id,
            route_revision,
            self.instance().id().clone(),
            model_id,
            capabilities.clone(),
        );
        let requirements = requirements(
            self,
            capabilities.iter().map(|(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            }),
        );
        let plan = build_plan(self, &instance, &route, &requirements)?;
        let policy = OperationPolicy::offline()
            .with_provider_retention(ProviderRetentionPolicy::DurableAllowed)
            .with_provider_recovery(swallowtail_runtime::ProviderRecoveryPolicy::ManagedAllowed)
            .with_harness_isolation(swallowtail_core::HarnessIsolation::AmbientHost)
            .with_harness_configuration_posture(
                swallowtail_core::HarnessConfigurationPosture::Ambient,
            );
        let request = StructuredRunRequest::new(input.request_id, input.content, policy)
            .with_working_resource(input.working_resource)
            .with_deadline(input.deadline);
        Ok(KimiHeadlessPreparedRun {
            evidence: KimiHeadlessPreparedEvidence::from_prepared(self, plan, activity)?,
            request,
        })
    }
}
