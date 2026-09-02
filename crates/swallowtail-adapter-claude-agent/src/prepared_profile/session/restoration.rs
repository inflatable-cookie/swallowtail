use super::handle::{validate_management_context, wrap_management_handle};
use crate::ClaudeAgentAcpDriver;
use swallowtail_core::ProviderSessionBindingOrigin;
use swallowtail_runtime::{
    BoxFuture, HostServices, InteractiveSessionDriver, LoadSessionRequest, LoadedSession,
    PreparedAccessEvidence, ProviderSessionContinuationRecoveryOutcome, RuntimeFailure,
    RuntimeTurnId, WorkingStateRestorationMethod, WorkingStateRestorationOperation,
    WorkingStateRestorationOutcome,
};

pub(super) struct ClaudeAgentContinuationRecovery {
    pub(super) driver: ClaudeAgentAcpDriver,
    pub(super) plan: swallowtail_core::PreflightPlan,
    pub(super) request: LoadSessionRequest,
    pub(super) management_instance: swallowtail_core::ConfiguredInstance,
    pub(super) access: PreparedAccessEvidence,
    pub(super) interrupted_turn_id: RuntimeTurnId,
}

impl WorkingStateRestorationOperation for ClaudeAgentContinuationRecovery {
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
            validate_management_context(&management_instance, &access)?;
            let loaded = driver.load_session(plan, request.clone(), services).await?;
            let (replay, handle) = loaded.into_parts();
            let handle = wrap_management_handle(
                handle,
                management_instance,
                access,
                Some(
                    request
                        .working_resource()
                        .expect("prepared Claude Agent recovery binds a working resource")
                        .clone(),
                ),
                ProviderSessionBindingOrigin::Loaded,
            )
            .await?;
            Ok(WorkingStateRestorationOutcome::SessionRecovered(
                ProviderSessionContinuationRecoveryOutcome::new(
                    interrupted_turn_id,
                    LoadedSession::new(replay, handle),
                ),
            ))
        })
    }
}
