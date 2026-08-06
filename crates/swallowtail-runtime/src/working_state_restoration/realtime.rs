use super::{
    PreparedWorkingStateRestoration, WorkingStateRestorationMethod,
    WorkingStateRestorationOperation, WorkingStateRestorationOutcome,
};
use crate::{
    BoxFuture, HostServices, OpenRealtimeMediaSessionRequest, RealtimeMediaSessionDriver,
    RealtimeMediaSessionHandle, RuntimeFailure, RuntimeTurnId,
};

/// A new usable realtime session carrying no media state from the lost one.
pub struct FreshRealtimeSessionReplacementOutcome {
    interrupted_turn_id: RuntimeTurnId,
    session: Box<dyn RealtimeMediaSessionHandle>,
}

impl FreshRealtimeSessionReplacementOutcome {
    /// Creates a context-losing realtime replacement outcome.
    #[must_use]
    pub const fn new(
        interrupted_turn_id: RuntimeTurnId,
        session: Box<dyn RealtimeMediaSessionHandle>,
    ) -> Self {
        Self {
            interrupted_turn_id,
            session,
        }
    }

    #[must_use]
    /// Returns the unresolved interrupted consumer turn.
    pub const fn interrupted_turn_id(&self) -> &RuntimeTurnId {
        &self.interrupted_turn_id
    }

    #[must_use]
    /// Separates the interrupted turn identity from the new realtime session.
    pub fn into_parts(self) -> (RuntimeTurnId, Box<dyn RealtimeMediaSessionHandle>) {
        (self.interrupted_turn_id, self.session)
    }
}

impl PreparedWorkingStateRestoration {
    /// Prepares a fresh realtime session with explicit connection-context loss.
    #[must_use]
    pub fn fresh_realtime_session_replacement(
        interrupted_turn_id: RuntimeTurnId,
        driver: impl RealtimeMediaSessionDriver + 'static,
        plan: swallowtail_core::PreflightPlan,
        request: OpenRealtimeMediaSessionRequest,
    ) -> Self {
        Self::new(FreshRealtimeSessionReplacementOperation {
            interrupted_turn_id,
            driver: Box::new(driver),
            plan,
            request,
        })
    }
}

struct FreshRealtimeSessionReplacementOperation {
    interrupted_turn_id: RuntimeTurnId,
    driver: Box<dyn RealtimeMediaSessionDriver>,
    plan: swallowtail_core::PreflightPlan,
    request: OpenRealtimeMediaSessionRequest,
}

impl WorkingStateRestorationOperation for FreshRealtimeSessionReplacementOperation {
    fn method(&self) -> WorkingStateRestorationMethod {
        WorkingStateRestorationMethod::FreshRealtimeSessionReplacement
    }

    fn restore(
        self: Box<Self>,
        services: HostServices,
    ) -> BoxFuture<'static, Result<WorkingStateRestorationOutcome, RuntimeFailure>> {
        Box::pin(async move {
            let session = self
                .driver
                .open_realtime_media_session(self.plan, self.request, services)
                .await?;
            Ok(WorkingStateRestorationOutcome::RealtimeSessionReplaced(
                FreshRealtimeSessionReplacementOutcome::new(self.interrupted_turn_id, session),
            ))
        })
    }
}
