use std::num::NonZeroU32;
use swallowtail_core::{ModelId, ModelRouteId, ModelRouteRevision};
use swallowtail_runtime::{
    Deadline, OperationContent, ProviderRecoveryPolicy, ProviderRetentionPolicy, RequestId,
    StreamReattachmentPolicy, ToolDeclaration,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnthropicManagedModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    model_id: ModelId,
}

impl AnthropicManagedModelSelection {
    #[must_use]
    pub const fn new(
        route_id: ModelRouteId,
        route_revision: ModelRouteRevision,
        model_id: ModelId,
    ) -> Self {
        Self {
            route_id,
            route_revision,
            model_id,
        }
    }

    pub(super) fn into_parts(self) -> (ModelRouteId, ModelRouteRevision, ModelId) {
        (self.route_id, self.route_revision, self.model_id)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnthropicManagedAgentRunInput {
    request_id: RequestId,
    model: AnthropicManagedModelSelection,
    content: OperationContent,
    deadline: Deadline,
    tools: Vec<ToolDeclaration>,
    provider_retention: ProviderRetentionPolicy,
    provider_recovery: ProviderRecoveryPolicy,
    stream_reattachment: StreamReattachmentPolicy,
}

impl AnthropicManagedAgentRunInput {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        request_id: RequestId,
        model: AnthropicManagedModelSelection,
        content: OperationContent,
        deadline: Deadline,
        tools: impl IntoIterator<Item = ToolDeclaration>,
        provider_retention: ProviderRetentionPolicy,
        provider_recovery: ProviderRecoveryPolicy,
        stream_reattachment: StreamReattachmentPolicy,
    ) -> Self {
        Self {
            request_id,
            model,
            content,
            deadline,
            tools: tools.into_iter().collect(),
            provider_retention,
            provider_recovery,
            stream_reattachment,
        }
    }

    #[must_use]
    pub fn durable_with_managed_recovery_and_one_reattachment(
        request_id: RequestId,
        model: AnthropicManagedModelSelection,
        content: OperationContent,
        deadline: Deadline,
        tools: impl IntoIterator<Item = ToolDeclaration>,
    ) -> Self {
        Self::new(
            request_id,
            model,
            content,
            deadline,
            tools,
            ProviderRetentionPolicy::DurableAllowed,
            ProviderRecoveryPolicy::ManagedAllowed,
            StreamReattachmentPolicy::Bounded(NonZeroU32::new(1).expect("one is non-zero")),
        )
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        RequestId,
        AnthropicManagedModelSelection,
        OperationContent,
        Deadline,
        Vec<ToolDeclaration>,
        ProviderRetentionPolicy,
        ProviderRecoveryPolicy,
        StreamReattachmentPolicy,
    ) {
        (
            self.request_id,
            self.model,
            self.content,
            self.deadline,
            self.tools,
            self.provider_retention,
            self.provider_recovery,
            self.stream_reattachment,
        )
    }
}
