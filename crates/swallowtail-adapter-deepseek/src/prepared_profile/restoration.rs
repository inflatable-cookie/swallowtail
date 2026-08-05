use super::session::DeepSeekPreparedSession;
use crate::DeepSeekDirectDriver;
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    BoxFuture, FreshSessionReplacementOutcome, HostServices, InteractiveSessionDriver,
    OpenDirectContinuationSessionRequest, PreparedWorkingStateRestoration, RuntimeFailure,
    RuntimeTurnId, WorkingStateRestorationMethod, WorkingStateRestorationOperation,
    WorkingStateRestorationOutcome,
};

impl DeepSeekPreparedSession {
    #[must_use]
    pub fn prepare_working_state_restoration(
        &self,
        interrupted_turn_id: RuntimeTurnId,
    ) -> PreparedWorkingStateRestoration {
        PreparedWorkingStateRestoration::new(DeepSeekFreshSessionReplacement {
            interrupted_turn_id,
            plan: self.plan().clone(),
            request: self.request().clone(),
        })
    }
}

struct DeepSeekFreshSessionReplacement {
    interrupted_turn_id: RuntimeTurnId,
    plan: PreflightPlan,
    request: OpenDirectContinuationSessionRequest,
}

impl WorkingStateRestorationOperation for DeepSeekFreshSessionReplacement {
    fn method(&self) -> WorkingStateRestorationMethod {
        WorkingStateRestorationMethod::FreshSessionReplacement
    }

    fn restore(
        self: Box<Self>,
        services: HostServices,
    ) -> BoxFuture<'static, Result<WorkingStateRestorationOutcome, RuntimeFailure>> {
        Box::pin(async move {
            let session = DeepSeekDirectDriver::new()
                .open_direct_continuation_session(self.plan, self.request, services)
                .await?;
            Ok(WorkingStateRestorationOutcome::SessionReplaced(
                FreshSessionReplacementOutcome::new(self.interrupted_turn_id, session),
            ))
        })
    }
}
