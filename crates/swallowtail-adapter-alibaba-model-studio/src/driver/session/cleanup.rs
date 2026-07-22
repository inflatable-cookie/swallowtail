use super::AccessLeases;
use crate::failure::protocol;
use crate::protocol::{
    ConversationRef, DeletionKind, WireRequest, parse_deletion, parse_inventory,
};
use crate::transport::CurlTransport;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use swallowtail_runtime::{CleanupOutcome, HostServices, RuntimeFailure, ScopeId};

pub(super) struct CleanupAccess {
    endpoint: String,
    secret: CleanupSecret,
}

impl CleanupAccess {
    pub(super) fn acquire(access: &AccessLeases) -> Result<Self, RuntimeFailure> {
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
