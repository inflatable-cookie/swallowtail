use super::replay_walk::{ReplayAccess, replay_bound_failure, walk_conversation_replay};
use crate::driver::AlibabaModelStudioDriver;
use crate::driver::access::AccessLeases;
use crate::failure::{failure, protocol, unsupported};
use crate::protocol::ConversationRef;
use swallowtail_core::{Capability, SessionProviderStatePolicy};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, ProviderSessionHistoryDriver, ProviderSessionHistoryPage,
    ProviderSessionHistoryPlan, ProviderSessionHistoryRequest, ProviderSessionHistoryTotal,
    RequestId, RuntimeFailure, ScopeId, page_provider_session_history_window,
    validate_provider_session_history_execution,
};

impl ProviderSessionHistoryDriver for AlibabaModelStudioDriver {
    fn page_provider_session_history(
        &self,
        plan: ProviderSessionHistoryPlan,
        request: ProviderSessionHistoryRequest,
        services: HostServices,
    ) -> swallowtail_runtime::BoxFuture<
        '_,
        Result<ProviderSessionHistoryPage, RuntimeFailure>,
    > {
        Box::pin(async move {
            validate_provider_session_history_execution(&plan, &request, &services)?;
            Self::validate_plan(plan.preflight())?;
            validate_history(&plan, &request, &services)?;
            services.require_execution_host(plan.preflight().execution_host_id())?;
            let scope = history_scope(request.request_id())?;
            let mut access = AccessLeases::acquire(plan.preflight(), scope.clone(), &services).await?;
            let conversation = ConversationRef::new(
                plan.agreement()
                    .binding()
                    .provider_session_ref()
                    .as_provider_value()
                    .to_owned(),
            )
            .map_err(protocol)?;
            let replay_access = ReplayAccess::new(access.endpoint.clone(), access.secret()?);
            let replay = match walk_conversation_replay(
                &self.transport,
                &scope,
                &replay_access,
                &conversation,
                plan.agreement().deadline(),
                &services,
            )
            .await
            {
                Ok(replay) => replay,
                Err(error) => {
                    let _ = access.release(&services).await;
                    return Err(error);
                }
            };
            let total = u32::try_from(replay.len()).map_err(|_| replay_bound_failure())?;
            let window = match page_provider_session_history_window(
                &plan,
                &request,
                replay,
                ProviderSessionHistoryTotal::Exact(total),
            ) {
                Ok(window) => window,
                Err(error) => {
                    let _ = access.release(&services).await;
                    return Err(error);
                }
            };
            let cleanup = access.release(&services).await;
            match &cleanup {
                CleanupOutcome::Clean | CleanupOutcome::NotApplicable => {}
                CleanupOutcome::Degraded(diagnostic) | CleanupOutcome::Failed(diagnostic) => {
                    return Err(RuntimeFailure::new(diagnostic.clone()));
                }
            }
            ProviderSessionHistoryPage::new(&plan, &request, window, cleanup)
        })
    }
}

fn history_scope(request_id: &RequestId) -> Result<ScopeId, RuntimeFailure> {
    ScopeId::new(format!(
        "alibaba-model-studio:history:{}",
        request_id.as_str()
    ))
    .map_err(|_| {
        failure(
            "swallowtail.alibaba_model_studio.scope_invalid",
            "Alibaba Model Studio session scope was invalid",
        )
    })
}

fn validate_history(
    plan: &ProviderSessionHistoryPlan,
    request: &ProviderSessionHistoryRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    let preflight = plan.preflight();
    let agreement = plan.agreement();
    if !preflight
        .requirements()
        .capabilities()
        .any(|required| required.capability() == Capability::ProviderSessionHistory)
        || preflight.requirements().session_provider_state_policy()
            != Some(SessionProviderStatePolicy::DurableProviderSessionPreserved)
    {
        return Err(unsupported("retained conversation history"));
    }
    if !agreement
        .binding()
        .matches_resource_free_attachment(preflight, agreement.binding().access_policy())
        || agreement.binding().provider_session_ref()
            != request.agreement().binding().provider_session_ref()
    {
        return Err(failure(
            "swallowtail.alibaba_model_studio.history_binding_mismatch",
            "Alibaba Model Studio retained conversation history binding did not match preflight",
        ));
    }
    if let Some(deadline) = agreement.deadline()
        && services
            .time()
            .is_some_and(|time| time.now() >= deadline.instant())
    {
        return Err(failure(
            "swallowtail.alibaba_model_studio.deadline_elapsed",
            "Alibaba Model Studio session deadline elapsed before provider work",
        ));
    }
    Ok(())
}
