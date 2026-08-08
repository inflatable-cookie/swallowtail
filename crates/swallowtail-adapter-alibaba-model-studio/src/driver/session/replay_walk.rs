use crate::driver::lifecycle::complete_before_deadline;
use crate::failure::{failure, protocol};
use crate::protocol::{
    ConversationRef, MAXIMUM_REPLAY_BYTES, MAXIMUM_REPLAY_ITEMS, MAXIMUM_REPLAY_PAGES, WireRequest,
    parse_conversation_retrieval, parse_replay_page,
};
use crate::transport::CurlTransport;
use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use swallowtail_runtime::{HostServices, RuntimeFailure, ScopeId, SessionReplayItem};

pub(super) struct ReplayAccess {
    pub(super) endpoint: String,
    secret: Vec<u8>,
}

impl ReplayAccess {
    pub(super) fn new(endpoint: String, secret: Vec<u8>) -> Self {
        Self { endpoint, secret }
    }

    pub(super) fn secret(&self) -> Result<Vec<u8>, RuntimeFailure> {
        Ok(self.secret.clone())
    }
}

impl Drop for ReplayAccess {
    fn drop(&mut self) {
        self.secret.fill(0);
    }
}

pub(super) async fn walk_conversation_replay(
    transport: &CurlTransport,
    scope: &ScopeId,
    access: &ReplayAccess,
    conversation: &ConversationRef,
    deadline: Option<swallowtail_runtime::Deadline>,
    services: &HostServices,
) -> Result<Vec<SessionReplayItem>, RuntimeFailure> {
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

pub(super) async fn request_before_deadline(
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
            access.secret()?,
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

pub(super) fn replay_bound_failure() -> RuntimeFailure {
    failure(
        "swallowtail.alibaba_model_studio.replay_invalid",
        "Alibaba Model Studio retained conversation replay was invalid or exceeded its bound",
    )
}
