use super::session::AnthropicPreparedSession;
use crate::AnthropicDirectDriver;
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    BoxFuture, FreshSessionReplacementOutcome, HostServices, InteractiveSessionDriver,
    OpenDirectContinuationSessionRequest, PreparedWorkingStateRestoration, RuntimeFailure,
    RuntimeTurnId, WorkingStateRestorationMethod, WorkingStateRestorationOperation,
    WorkingStateRestorationOutcome,
};

impl AnthropicPreparedSession {
    #[must_use]
    /// Prepares fresh-session replacement after private continuation is lost.
    pub fn prepare_working_state_restoration(
        &self,
        interrupted_turn_id: RuntimeTurnId,
    ) -> PreparedWorkingStateRestoration {
        PreparedWorkingStateRestoration::new(AnthropicFreshSessionReplacement {
            interrupted_turn_id,
            plan: self.plan().clone(),
            request: self.request().clone(),
            thinking_mode: self.evidence().thinking_mode(),
        })
    }
}

struct AnthropicFreshSessionReplacement {
    interrupted_turn_id: RuntimeTurnId,
    plan: PreflightPlan,
    request: OpenDirectContinuationSessionRequest,
    thinking_mode: Option<crate::AnthropicThinkingMode>,
}

impl WorkingStateRestorationOperation for AnthropicFreshSessionReplacement {
    fn method(&self) -> WorkingStateRestorationMethod {
        WorkingStateRestorationMethod::FreshSessionReplacement
    }

    fn restore(
        self: Box<Self>,
        services: HostServices,
    ) -> BoxFuture<'static, Result<WorkingStateRestorationOutcome, RuntimeFailure>> {
        Box::pin(async move {
            let driver = match self.thinking_mode {
                Some(mode) => AnthropicDirectDriver::new().with_thinking_mode(mode),
                None => AnthropicDirectDriver::new(),
            };
            let session = driver
                .open_direct_continuation_session(self.plan, self.request, services)
                .await?;
            Ok(WorkingStateRestorationOutcome::SessionReplaced(
                FreshSessionReplacementOutcome::new(self.interrupted_turn_id, session),
            ))
        })
    }
}
