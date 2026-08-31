use super::{Contribution, session_options_bound};
use crate::consumer_route_projection::builder::{
    ProjectionBuilder, ProjectionRoute, bounded, exact,
};
use crate::{ClaudeAgentPreparedDelete, ClaudeAgentPreparedRun, ClaudeAgentPreparedSession};
use swallowtail_core::{Capability, ProviderSessionManagementAction};
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteOmissionSemantics, ConsumerRouteProjectionSourceId,
    ConsumerRouteValueKind, ProviderRetentionPolicy,
};

impl ClaudeAgentPreparedRun {
    /// Emits only the exact structured-run truth this prepared ACP run proves.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Contribution {
        let mut builder =
            ProjectionBuilder::prepared(self.plan(), ProjectionRoute::Agent, source_id)
                .with_prepared_capabilities()
                .with_callback_features()
                .with_model_selection();
        if let Some(reasoning) = self.request().policy().reasoning_mode() {
            builder.push_control(
                ConsumerRouteControlId::ReasoningSelection,
                ConsumerRouteValueKind::BoundedEnumeration,
                exact(reasoning.as_str())?,
                ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
                true,
            );
        }
        let mediated = self.plan().requirements().extension_namespaces().len() > 0;
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
        let retention = builder.local_control("control.run-retention")?;
        builder.push_control(
            retention,
            ConsumerRouteValueKind::BoundedEnumeration,
            exact(match self.request().policy().provider_retention() {
                ProviderRetentionPolicy::DurableAllowed => "Durable",
                ProviderRetentionPolicy::TemporaryAllowed => "TemporaryWithOwnedSessionCleanup",
                ProviderRetentionPolicy::Prohibited => "Prohibited",
            })?,
            ConsumerRouteOmissionSemantics::Required,
            false,
        );
        builder.build()
    }
}

impl ClaudeAgentPreparedSession {
    /// Emits only prepared ACP session truth; acknowledgement remains post-open.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Contribution {
        let mut builder =
            ProjectionBuilder::prepared(self.plan(), ProjectionRoute::Agent, source_id)
                .with_prepared_capabilities()
                .with_callback_features()
                .with_model_selection();
        for capability in [
            Capability::ProviderSessionDelete,
            Capability::ProviderNativeSessionClose,
        ] {
            if self
                .management_instance()
                .capabilities()
                .supports(capability)
            {
                builder = builder.with_additional_capability(capability);
            }
        }
        if let Some(reasoning) = self.request().options().reasoning_mode() {
            builder.push_control(
                ConsumerRouteControlId::ReasoningSelection,
                ConsumerRouteValueKind::AcknowledgedEnumeration,
                exact(reasoning.as_str())?,
                ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
                true,
            );
        }
        builder.push_control(
            ConsumerRouteControlId::SessionOptions,
            ConsumerRouteValueKind::StructuredOptions,
            bounded(session_options_bound(self))?,
            ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
            false,
        );
        let mediated = self.plan().requirements().extension_namespaces().len() > 0;
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
        builder.build()
    }
}

impl ClaudeAgentPreparedDelete {
    /// Emits only the exact inactive-session delete truth this operation proves.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Contribution {
        let mut builder =
            ProjectionBuilder::prepared(self.plan().preflight(), ProjectionRoute::Agent, source_id)
                .with_prepared_capabilities();
        let control = builder.local_control("control.provider-session-delete")?;
        let action = match self.plan().agreement().action() {
            ProviderSessionManagementAction::Delete(_) => {
                "exact inactive provider session deletion"
            }
            _ => "unsupported provider session management action",
        };
        builder.push_control(
            control,
            ConsumerRouteValueKind::LifecycleAction,
            bounded(action)?,
            ConsumerRouteOmissionSemantics::SuppliesNothing,
            false,
        );
        builder.build()
    }
}
