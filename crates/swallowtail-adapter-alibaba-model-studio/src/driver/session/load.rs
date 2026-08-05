use super::{AlibabaSessionHandle, ConversationRetention, retention};
use crate::driver::AlibabaModelStudioDriver;
use crate::driver::access::AccessLeases;
use crate::driver::lifecycle::{SessionCancellation, complete_before_deadline};
use crate::failure::{failure, protocol, unsupported};
use crate::protocol::{
    ConversationRef, MAXIMUM_REPLAY_BYTES, MAXIMUM_REPLAY_ITEMS, MAXIMUM_REPLAY_PAGES, WireRequest,
    parse_conversation_retrieval, parse_replay_page,
};
use crate::transport::CurlTransport;
use std::collections::BTreeSet;
use std::sync::atomic::{AtomicBool, AtomicU8};
use std::sync::{Arc, Mutex};
use swallowtail_core::{Capability, PreflightPlan};
use swallowtail_runtime::{
    HostServices, LoadSessionRequest, LoadedSession, RequestId, RuntimeFailure, RuntimeSessionId,
    ScopeId, validate_session_plan_agreement,
};

pub(super) async fn load_session(
    driver: AlibabaModelStudioDriver,
    plan: PreflightPlan,
    request: LoadSessionRequest,
    services: HostServices,
) -> Result<LoadedSession, RuntimeFailure> {
    AlibabaModelStudioDriver::validate_plan(&plan)?;
    services.require_execution_host(plan.execution_host_id())?;
    validate_load(&plan, &request, &services)?;
    let scope = session_scope("load", request.request_id())?;
    let runtime_id = runtime_session_id(request.request_id())?;
    let mut access = AccessLeases::acquire(&plan, scope.clone(), &services).await?;
    let conversation = ConversationRef::new(
        request
            .provider_session_ref()
            .as_provider_value()
            .to_owned(),
    )
    .map_err(protocol)?;
    let replay_access = ReplayAccess {
        endpoint: access.endpoint.clone(),
        secret: access.secret()?,
    };
    let result = load_replay(
        &driver.transport,
        &scope,
        &replay_access,
        &conversation,
        request.deadline(),
        &services,
    )
    .await;
    let replay = match result {
        Ok(replay) => replay,
        Err(error) => {
            let _ = access.release(&services).await;
            return Err(error);
        }
    };
    let active = Arc::new(Mutex::new(None));
    let usable = Arc::new(AtomicBool::new(true));
    let cancellation = Arc::new(SessionCancellation::new(
        Arc::clone(&active),
        Arc::clone(&usable),
    ));
    let handle = AlibabaSessionHandle {
        request_id: request.request_id().clone(),
        runtime_id,
        scope,
        services,
        transport: driver.transport,
        conversation,
        provider_session_ref: Some(request.provider_session_ref().clone()),
        resume_binding: Some(request.resume_binding().clone()),
        retention: ConversationRetention::Preserve,
        access: Some(access),
        completed_turns: Arc::new(AtomicU8::new(0)),
        usable,
        remote_uncertain: Arc::new(AtomicBool::new(false)),
        active,
        cancellation,
    };
    Ok(LoadedSession::new(replay, Box::new(handle)))
}

async fn load_replay(
    transport: &CurlTransport,
    scope: &ScopeId,
    access: &ReplayAccess,
    conversation: &ConversationRef,
    deadline: Option<swallowtail_runtime::Deadline>,
    services: &HostServices,
) -> Result<Vec<swallowtail_runtime::SessionReplayItem>, RuntimeFailure> {
    let response = request_before_deadline(
        transport,
        scope,
        access,
        WireRequest::retrieve_conversation(conversation),
        deadline,
        services,
    )
    .await?;
    parse_conversation_retrieval(&response.body, conversation).map_err(protocol)?;

    let mut replay = Vec::new();
    let mut seen = BTreeSet::new();
    let mut after = None;
    let mut content_bytes = 0usize;
    for page_index in 0..MAXIMUM_REPLAY_PAGES {
        let response = request_before_deadline(
            transport,
            scope,
            access,
            WireRequest::list_items_after(conversation, after.as_ref()),
            deadline,
            services,
        )
        .await?;
        let sequence = u64::try_from(replay.len()).map_err(|_| replay_bound_failure())?;
        let page = parse_replay_page(&response.body, conversation, sequence).map_err(protocol)?;
        if page.item_ids().any(|item| !seen.insert(item.clone())) {
            return Err(replay_bound_failure());
        }
        content_bytes = content_bytes
            .checked_add(page.content_bytes())
            .ok_or_else(replay_bound_failure)?;
        if content_bytes > MAXIMUM_REPLAY_BYTES
            || replay.len().saturating_add(page.replay().len()) > MAXIMUM_REPLAY_ITEMS
        {
            return Err(replay_bound_failure());
        }
        let next = page.next_after().cloned();
        replay.extend(page.into_replay());
        match next {
            None => return Ok(replay),
            Some(next)
                if after.as_ref() == Some(&next) || page_index + 1 == MAXIMUM_REPLAY_PAGES =>
            {
                return Err(replay_bound_failure());
            }
            Some(next) => after = Some(next),
        }
    }
    Err(replay_bound_failure())
}

async fn request_before_deadline(
    transport: &CurlTransport,
    scope: &ScopeId,
    access: &ReplayAccess,
    request: WireRequest,
    deadline: Option<swallowtail_runtime::Deadline>,
    services: &HostServices,
) -> Result<crate::transport::Response, RuntimeFailure> {
    let cancelled = Arc::new(AtomicBool::new(false));
    complete_before_deadline(
        transport.request(
            scope.clone(),
            access.endpoint.clone(),
            access.secret.clone(),
            request,
            services,
            Arc::clone(&cancelled),
        ),
        deadline,
        services,
        cancelled,
    )
    .await
}

struct ReplayAccess {
    endpoint: String,
    secret: Vec<u8>,
}

impl Drop for ReplayAccess {
    fn drop(&mut self) {
        self.secret.fill(0);
    }
}

fn session_scope(kind: &str, request_id: &RequestId) -> Result<ScopeId, RuntimeFailure> {
    ScopeId::new(format!(
        "alibaba-model-studio:{kind}:{}",
        request_id.as_str()
    ))
    .map_err(|_| {
        failure(
            "swallowtail.alibaba_model_studio.scope_invalid",
            "Alibaba Model Studio session scope was invalid",
        )
    })
}

fn runtime_session_id(request_id: &RequestId) -> Result<RuntimeSessionId, RuntimeFailure> {
    RuntimeSessionId::new(format!("alibaba-model-studio:{}", request_id.as_str())).map_err(|_| {
        failure(
            "swallowtail.alibaba_model_studio.session_id_invalid",
            "Alibaba Model Studio runtime session identity was invalid",
        )
    })
}

fn replay_bound_failure() -> RuntimeFailure {
    failure(
        "swallowtail.alibaba_model_studio.replay_invalid",
        "Alibaba Model Studio retained conversation replay was invalid or exceeded its bound",
    )
}

fn validate_load(
    plan: &PreflightPlan,
    request: &LoadSessionRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    validate_session_plan_agreement(plan, request.plan_agreement())?;
    if !plan
        .requirements()
        .capabilities()
        .any(|required| required.capability() == Capability::LoadSession)
        || retention(plan)? != ConversationRetention::Preserve
    {
        return Err(unsupported("retained conversation load"));
    }
    if request.working_resource().is_some()
        || !request
            .resume_binding()
            .matches_resource_free_attachment(plan, request.access_policy())
        || request.provider_session_ref() != request.resume_binding().provider_session_ref()
    {
        return Err(failure(
            "swallowtail.alibaba_model_studio.load_binding_mismatch",
            "Alibaba Model Studio retained conversation binding did not match preflight",
        ));
    }
    if !request.options().is_empty() {
        return Err(unsupported("session options"));
    }
    if let Some(deadline) = request.deadline()
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
