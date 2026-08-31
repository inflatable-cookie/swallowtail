#[path = "contribution/agent.rs"]
mod agent;
#[path = "contribution/code.rs"]
mod code;

use super::builder::{ProjectionBuilder, ProjectionRoute, bounded, exact};
use crate::ClaudeAgentPreparedSession;
use swallowtail_core::Capability;
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteOmissionSemantics, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailure, ConsumerRouteProjectionSourceId, ConsumerRouteValueKind,
};

type Contribution = Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure>;

fn session_options_bound(session: &ClaudeAgentPreparedSession) -> &'static str {
    let options = session.request().options();
    match (options.reasoning_mode().is_some(), options.harness_mode()) {
        (false, None) => "empty validated Claude Agent session options",
        (true, None) => "validated Claude Agent reasoning session option",
        (false, Some(_)) => "validated Claude Agent harness-mode session option",
        (true, Some(_)) => "validated Claude Agent reasoning and harness-mode session options",
    }
}

pub(crate) fn observed_session_contribution(
    session: &ClaudeAgentPreparedSession,
    prepared_source_id: ConsumerRouteProjectionSourceId,
    active_source_id: ConsumerRouteProjectionSourceId,
    reasoning: Option<(&str, bool)>,
) -> Contribution {
    let mut builder = ProjectionBuilder::observed(
        session.plan(),
        ProjectionRoute::Agent,
        prepared_source_id,
        active_source_id,
    )
    .with_prepared_capabilities()
    .with_callback_features()
    .with_model_selection();
    for capability in [
        Capability::ProviderSessionDelete,
        Capability::ProviderNativeSessionClose,
    ] {
        if session
            .management_instance()
            .capabilities()
            .supports(capability)
        {
            builder = builder.with_additional_capability(capability);
        }
    }
    if let Some(requested) = session.request().options().reasoning_mode() {
        builder.push_control(
            ConsumerRouteControlId::ReasoningSelection,
            ConsumerRouteValueKind::AcknowledgedEnumeration,
            exact(requested.as_str())?,
            ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
            reasoning.is_none(),
        );
    }
    builder.push_control(
        ConsumerRouteControlId::SessionOptions,
        ConsumerRouteValueKind::StructuredOptions,
        bounded(session_options_bound(session))?,
        ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
        false,
    );
    let mediated = session.plan().requirements().extension_namespaces().len() > 0;
    let permission = builder.local_control("control.permission-handling")?;
    builder.push_control(
        permission,
        ConsumerRouteValueKind::BoundedEnumeration,
        exact(if mediated {
            "ConsumerMediated"
        } else {
            "RejectAndStop"
        })?,
        ConsumerRouteOmissionSemantics::Required,
        false,
    );
    for control in [
        ConsumerRouteControlId::LoadSession,
        ConsumerRouteControlId::ResumeSession,
    ] {
        builder.push_control(
            control,
            ConsumerRouteValueKind::LifecycleAction,
            bounded("exact provider session identity and prepared route binding")?,
            ConsumerRouteOmissionSemantics::SuppliesNothing,
            false,
        );
    }
    if let Some((value, rejected)) = reasoning {
        builder = builder.with_observed_reasoning(value, rejected)?;
    }
    builder.build()
}
