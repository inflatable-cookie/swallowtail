use super::AccessLeases;
use crate::failure::protocol;
use crate::protocol::{
    ConversationRef, DeletionKind, ItemRef, MAXIMUM_REPLAY_BYTES, MAXIMUM_REPLAY_ITEMS,
    MAXIMUM_REPLAY_PAGES, WireRequest, parse_deletion, parse_inventory, parse_replay_page,
};
use crate::transport::CurlTransport;
use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use swallowtail_runtime::{CleanupOutcome, HostServices, RuntimeFailure, ScopeId};

pub(in crate::driver) struct CleanupAccess {
    endpoint: String,
    secret: CleanupSecret,
}

impl CleanupAccess {
    pub(in crate::driver) fn acquire(access: &AccessLeases) -> Result<Self, RuntimeFailure> {
        Ok(Self {
            endpoint: access.endpoint.clone(),
            secret: CleanupSecret(access.secret()?),
        })
    }
}

pub(super) async fn cleanup_conversation(
    transport: &CurlTransport,
    scope: &ScopeId,
    services: &HostServices,
    access: &CleanupAccess,
    conversation: &ConversationRef,
    remote_uncertain: bool,
) -> CleanupOutcome {
    let mut first_failure = None;
    let inventory = request(
        transport,
        scope,
        services,
        access,
        WireRequest::list_items(conversation),
    )
    .await
    .and_then(|response| parse_inventory(&response.body).map_err(protocol));
    match inventory {
        Ok(inventory) => {
            for item in inventory.items() {
                let result = request(
                    transport,
                    scope,
                    services,
                    access,
                    WireRequest::delete_item(conversation, item),
                )
                .await
                .and_then(|response| {
                    parse_deletion(
                        &response.body,
                        item.as_str(),
                        DeletionKind::ConversationItem,
                    )
                    .map_err(protocol)
                });
                if let Err(error) = result {
                    first_failure.get_or_insert(error);
                }
            }
        }
        Err(error) => first_failure = Some(error),
    }
    let deletion = request(
        transport,
        scope,
        services,
        access,
        WireRequest::delete_conversation(conversation),
    )
    .await
    .and_then(|response| {
        parse_deletion(
            &response.body,
            conversation.as_str(),
            DeletionKind::Conversation,
        )
        .map_err(protocol)
    });
    if let Err(error) = deletion {
        first_failure.get_or_insert(error);
    }
    match first_failure {
        Some(error) => CleanupOutcome::Failed(error.diagnostic().clone()),
        None if remote_uncertain => {
            CleanupOutcome::Degraded(swallowtail_core::SafeDiagnostic::new(
                "swallowtail.alibaba_model_studio.remote_cleanup_unconfirmed",
                "Alibaba Model Studio cleanup raced uncertain remote turn state",
            ))
        }
        None => CleanupOutcome::Clean,
    }
}

pub(in crate::driver) enum ManagedDeletion {
    Applied,
    FailedBeforeEffect(RuntimeFailure),
    UnconfirmedAfterEffect(RuntimeFailure),
}

pub(in crate::driver) async fn delete_retained_conversation(
    transport: &CurlTransport,
    scope: &ScopeId,
    services: &HostServices,
    access: &CleanupAccess,
    conversation: &ConversationRef,
) -> ManagedDeletion {
    let inventory = match retained_inventory(transport, scope, services, access, conversation).await
    {
        Ok(inventory) => inventory,
        Err(error) => return ManagedDeletion::FailedBeforeEffect(error),
    };
    for item in &inventory {
        let result = request(
            transport,
            scope,
            services,
            access,
            WireRequest::delete_item(conversation, item),
        )
        .await
        .and_then(|response| {
            parse_deletion(
                &response.body,
                item.as_str(),
                DeletionKind::ConversationItem,
            )
            .map_err(protocol)
        });
        if let Err(error) = result {
            return ManagedDeletion::UnconfirmedAfterEffect(error);
        }
    }
    match request(
        transport,
        scope,
        services,
        access,
        WireRequest::delete_conversation(conversation),
    )
    .await
    .and_then(|response| {
        parse_deletion(
            &response.body,
            conversation.as_str(),
            DeletionKind::Conversation,
        )
        .map_err(protocol)
    }) {
        Ok(_) => ManagedDeletion::Applied,
        Err(error) => ManagedDeletion::UnconfirmedAfterEffect(error),
    }
}

async fn retained_inventory(
    transport: &CurlTransport,
    scope: &ScopeId,
    services: &HostServices,
    access: &CleanupAccess,
    conversation: &ConversationRef,
) -> Result<Vec<ItemRef>, RuntimeFailure> {
    let mut items = Vec::new();
    let mut seen = BTreeSet::new();
    let mut after = None;
    let mut content_bytes = 0usize;
    for page_index in 0..MAXIMUM_REPLAY_PAGES {
        let response = request(
            transport,
            scope,
            services,
            access,
            WireRequest::list_items_after(conversation, after.as_ref()),
        )
        .await?;
        let sequence = u64::try_from(items.len()).map_err(|_| managed_inventory_failure())?;
        let page = parse_replay_page(&response.body, conversation, sequence).map_err(protocol)?;
        for item in page.item_ids() {
            if !seen.insert(item.clone()) {
                return Err(managed_inventory_failure());
            }
            items.push(item.clone());
        }
        content_bytes = content_bytes
            .checked_add(page.content_bytes())
            .ok_or_else(managed_inventory_failure)?;
        if items.len() > MAXIMUM_REPLAY_ITEMS || content_bytes > MAXIMUM_REPLAY_BYTES {
            return Err(managed_inventory_failure());
        }
        let next = page.next_after().cloned();
        match next {
            None => return Ok(items),
            Some(next)
                if after.as_ref() == Some(&next) || page_index + 1 == MAXIMUM_REPLAY_PAGES =>
            {
                return Err(managed_inventory_failure());
            }
            Some(next) => after = Some(next),
        }
    }
    Err(managed_inventory_failure())
}

fn managed_inventory_failure() -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.alibaba_model_studio.lifecycle_inventory_invalid",
        "Alibaba Model Studio retained deletion inventory was invalid or exceeded its bound",
    )
}

async fn request(
    transport: &CurlTransport,
    scope: &ScopeId,
    services: &HostServices,
    access: &CleanupAccess,
    request: WireRequest,
) -> Result<crate::transport::Response, RuntimeFailure> {
    transport
        .request(
            scope.clone(),
            access.endpoint.clone(),
            access.secret.0.clone(),
            request,
            services,
            Arc::new(AtomicBool::new(false)),
        )
        .await
}

struct CleanupSecret(Vec<u8>);

impl Drop for CleanupSecret {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}
