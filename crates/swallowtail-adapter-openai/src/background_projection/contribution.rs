use super::builder::BackgroundProjectionBuilder;
use super::features::{exact, route_local};
use crate::OpenAiPreparedBackgroundRun;
use swallowtail_core::Capability;
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteEnumerableValue, ConsumerRouteOmissionSemantics,
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure,
    ConsumerRouteProjectionSourceId, ConsumerRouteValueDomain, ConsumerRouteValueKind,
    ProviderExecutionPolicy, ProviderRetentionPolicy, StreamReattachmentPolicy,
};

impl OpenAiPreparedBackgroundRun {
    /// Emits only the background structured-run truth this prepared run proves.
    ///
    /// Catalogue, Realtime, and background reconciliation rows are withheld at
    /// construction because a separate prepared family owns them. Retained
    /// execution, reattachment, and owned-resource cleanup keep bounded
    /// route-local identity rather than borrowing a portable name. No row
    /// claims provider-effective, rejected, or acknowledged state, because the
    /// prepared background route observes no acknowledgement.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        let mut builder = BackgroundProjectionBuilder::new(self.plan(), source_id)
            .with_prepared_capabilities()
            .with_model_selection();
        let segment = builder.segment().to_owned();
        let policy = self.request().policy();
        if let Some(mode) = policy.reasoning_mode() {
            builder.push_session_start_control(
                ConsumerRouteControlId::ReasoningSelection,
                ConsumerRouteValueKind::BoundedEnumeration,
                exact(mode.as_str())?,
                ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
            );
        }
        if let Some(maximum) = self.request().maximum_output_tokens() {
            builder.push_session_start_control(
                ConsumerRouteControlId::MaximumOutputTokens,
                ConsumerRouteValueKind::BoundedInteger,
                exact(&maximum.get().to_string())?,
                ConsumerRouteOmissionSemantics::Required,
            );
        }
        if self.request().structured_output().is_some() {
            builder.push_session_start_control(
                local(&segment, "control.structured-output")?,
                ConsumerRouteValueKind::StructuredDeclarations,
                bounded("inline JSON Schema 2020-12 document the exact route accepts")?,
                ConsumerRouteOmissionSemantics::SuppliesNothing,
            );
        }
        builder.push_session_start_control(
            local(&segment, "control.provider-execution-policy")?,
            ConsumerRouteValueKind::BoundedPolicy,
            exact(execution_policy(policy.provider_execution()))?,
            ConsumerRouteOmissionSemantics::Required,
        );
        builder.push_session_start_control(
            local(&segment, "control.provider-retention-policy")?,
            ConsumerRouteValueKind::BoundedPolicy,
            exact(retention_policy(policy.provider_retention()))?,
            ConsumerRouteOmissionSemantics::Required,
        );
        builder.push_session_start_control(
            local(&segment, "control.stream-reattachment")?,
            ConsumerRouteValueKind::BoundedPolicy,
            exact(&reattachment_policy(policy.stream_reattachment()))?,
            ConsumerRouteOmissionSemantics::Required,
        );
        if let Some(tier) = self.evidence().service_tier() {
            builder.push_session_start_control(
                local(&segment, "control.service-tier")?,
                ConsumerRouteValueKind::BoundedEnumeration,
                exact(tier.as_str())?,
                ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
            );
        }
        if self
            .plan()
            .requirements()
            .capabilities()
            .any(|requirement| requirement.capability() == Capability::ActiveOperationDetachment)
        {
            builder.push_session_start_control(
                local(&segment, "control.active-run-detachment")?,
                ConsumerRouteValueKind::BoundedPolicy,
                exact("structured-run active-run detachment")?,
                ConsumerRouteOmissionSemantics::SuppliesNothing,
            );
        }
        builder.build()
    }
}

/// Names one background-local control by route, revision, and semantic id.
fn local(
    segment: &str,
    semantic_id: &str,
) -> Result<ConsumerRouteControlId, ConsumerRouteProjectionFailure> {
    route_local(segment, semantic_id).map(ConsumerRouteControlId::Namespaced)
}

/// Publishes an explicitly unenumerated domain with the bound the source gave.
fn bounded(value: &str) -> Result<ConsumerRouteValueDomain, ConsumerRouteProjectionFailure> {
    Ok(ConsumerRouteValueDomain::Unenumerated(
        ConsumerRouteEnumerableValue::new(value)?,
    ))
}

const fn execution_policy(policy: ProviderExecutionPolicy) -> &'static str {
    match policy {
        ProviderExecutionPolicy::Attached => "Attached",
        ProviderExecutionPolicy::Background => "Background",
    }
}

const fn retention_policy(policy: ProviderRetentionPolicy) -> &'static str {
    match policy {
        ProviderRetentionPolicy::Prohibited => "Prohibited",
        ProviderRetentionPolicy::TemporaryAllowed => "TemporaryAllowed",
        ProviderRetentionPolicy::DurableAllowed => "DurableAllowed",
    }
}

fn reattachment_policy(policy: StreamReattachmentPolicy) -> String {
    match policy {
        StreamReattachmentPolicy::Disabled => "Disabled".to_owned(),
        StreamReattachmentPolicy::Bounded(count) => format!("Bounded({})", count.get()),
    }
}
