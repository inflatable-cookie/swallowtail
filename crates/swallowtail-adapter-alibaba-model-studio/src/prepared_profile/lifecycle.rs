use super::input::AlibabaSessionManagementInput;
use super::plan::build_plan_without_route;
use super::retained::lifecycle_management_instance;
use crate::prepared::failure;
use crate::{AlibabaModelStudioDriver, AlibabaModelStudioPreparedIntegration};
use swallowtail_core::{
    ProviderSessionActivityEvidence, ProviderSessionAffectedScope,
    ProviderSessionCancellationPosture, ProviderSessionDeletionStrength,
    ProviderSessionInitialStateRequirement, ProviderSessionManagementAction,
};
use swallowtail_runtime::{
    BoxFuture, DeleteProviderSessionRequest, HostServices, PreparationFailure, PreparationStage,
    PreparedProviderSessionManagementEvidence, ProviderSessionManagementAgreement,
    ProviderSessionManagementDriver, ProviderSessionManagementOutcome,
    ProviderSessionManagementPlan, RuntimeFailure,
};

#[derive(Clone, Debug)]
pub struct AlibabaModelStudioPreparedDelete {
    evidence: PreparedProviderSessionManagementEvidence,
    request: DeleteProviderSessionRequest,
}

impl AlibabaModelStudioPreparedDelete {
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
        let driver = AlibabaModelStudioDriver::new();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.delete_session(plan, request, services).await })
    }
}

impl AlibabaModelStudioPreparedIntegration {
    pub fn prepare_delete_retained_conversation(
        &self,
        input: AlibabaSessionManagementInput,
    ) -> Result<AlibabaModelStudioPreparedDelete, PreparationFailure> {
        let (request_id, binding, deadline) = input.into_parts();
        let instance = lifecycle_management_instance(self);
        let requirements = crate::alibaba_model_studio_management_requirements(
            self.instance().execution_host_id().clone(),
        );
        let preflight = build_plan_without_route(self, &instance, &requirements)?;
        let action = ProviderSessionManagementAction::Delete(
            ProviderSessionDeletionStrength::ProviderDataDeleted,
        );
        let agreement = ProviderSessionManagementAgreement::new(
            binding,
            action,
            ProviderSessionInitialStateRequirement::Unarchived,
            ProviderSessionAffectedScope::ProviderDefinedDescendants,
            ProviderSessionActivityEvidence::CallerAssertedInactive,
            ProviderSessionCancellationPosture::BeforeDispatchOnly,
            deadline,
        );
        let plan = ProviderSessionManagementPlan::new(preflight, agreement).map_err(|_| {
            failure(
                PreparationStage::Preflight,
                "swallowtail.alibaba_model_studio.preparation.lifecycle_binding_mismatch",
                "Alibaba Model Studio deletion binding did not match this prepared integration",
            )
        })?;
        let request = DeleteProviderSessionRequest::from_plan(request_id, &plan).map_err(|_| {
            failure(
                PreparationStage::Preflight,
                "swallowtail.alibaba_model_studio.preparation.lifecycle_request_invalid",
                "Alibaba Model Studio deletion request could not be prepared",
            )
        })?;
        Ok(AlibabaModelStudioPreparedDelete {
            evidence: PreparedProviderSessionManagementEvidence::from_plan(plan)?,
            request,
        })
    }
}
