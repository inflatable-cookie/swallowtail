use super::retained::{AlibabaModelStudioPreparedRetainedConversation, load_retained_session};
use crate::AlibabaModelStudioDriver;
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    BoxFuture, HostServices, LoadSessionRequest, PreparationFailure, PreparedAccessEvidence,
    PreparedWorkingStateRestoration, ProviderSessionContinuationRecoveryOutcome, RuntimeFailure,
    RuntimeTurnId, SessionResumeBinding, WorkingStateRestorationMethod,
    WorkingStateRestorationOperation, WorkingStateRestorationOutcome,
};

impl AlibabaModelStudioPreparedRetainedConversation {
    pub fn prepare_working_state_restoration(
        &self,
        request_id: swallowtail_runtime::RequestId,
        binding: SessionResumeBinding,
        interrupted_turn_id: RuntimeTurnId,
    ) -> Result<PreparedWorkingStateRestoration, PreparationFailure> {
        let request = self.load_request(request_id, binding)?;
        Ok(PreparedWorkingStateRestoration::new(
            AlibabaRetainedConversationRecovery {
                driver: self.low_level_driver(),
                plan: self.plan().clone(),
                request,
                management_instance: self.management_instance().clone(),
                access: self.evidence().access().clone(),
                interrupted_turn_id,
            },
        ))
    }
}

struct AlibabaRetainedConversationRecovery {
    driver: AlibabaModelStudioDriver,
    plan: PreflightPlan,
    request: LoadSessionRequest,
    management_instance: swallowtail_core::ConfiguredInstance,
    access: PreparedAccessEvidence,
    interrupted_turn_id: RuntimeTurnId,
}

impl WorkingStateRestorationOperation for AlibabaRetainedConversationRecovery {
    fn method(&self) -> WorkingStateRestorationMethod {
        WorkingStateRestorationMethod::ProviderSessionContinuationRecovery
    }

    fn restore(
        self: Box<Self>,
        services: HostServices,
    ) -> BoxFuture<'static, Result<WorkingStateRestorationOutcome, RuntimeFailure>> {
        let Self {
            driver,
            plan,
            request,
            management_instance,
            access,
            interrupted_turn_id,
        } = *self;
        Box::pin(async move {
            let loaded =
                load_retained_session(driver, plan, request, management_instance, access, services)
                    .await?;
            Ok(WorkingStateRestorationOutcome::SessionRecovered(
                ProviderSessionContinuationRecoveryOutcome::new(interrupted_turn_id, loaded),
            ))
        })
    }
}
