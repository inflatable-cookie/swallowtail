use super::{CodexProjectionBuilder, bounded};
use crate::CodexPreparedExec;
use swallowtail_core::{
    ExternalNetworkPolicy, ExternalSearchPolicy, InstalledExecutableCompatibility,
};
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteEnumerableValue, ConsumerRouteEnumeratedValues,
    ConsumerRouteNamespacedExtension, ConsumerRouteOmissionSemantics,
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure,
    ConsumerRouteProjectionSourceId, ConsumerRouteValueDomain, ConsumerRouteValueKind,
};

/// Exact census route the bounded exec-local descriptors belong to.
const EXEC_ROUTE: &str = "codex.exec";

impl CodexPreparedExec {
    /// Emits only the structured-exec truth this prepared run proves.
    ///
    /// The one-shot exec route proves no catalogue, interactive-session,
    /// provider-session lifecycle, or tool-exchange truth, so those rows are
    /// withheld at construction rather than borrowed from `codex.app-server`
    /// or a documentation matrix. Adapter-local exec controls without portable
    /// identity are published as bounded namespaced descriptors qualified by
    /// the exact route and the prepared behavior revision.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        let segment = self.behavior_segment();
        let policy = self.request().policy();
        let mut builder = CodexProjectionBuilder::new(self.plan(), source_id)
            .with_prepared_capabilities()
            .with_question_exchange()
            .with_model_selection();
        if policy.reasoning_mode().is_some() {
            builder.push_session_start_control(
                ConsumerRouteControlId::ReasoningSelection,
                ConsumerRouteValueKind::BoundedEnumeration,
                bounded("route-qualified Codex exec reasoning"),
                ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
            );
        }
        if self.request().structured_output().is_some() {
            builder.push_session_start_control(
                exec_control(segment, "control.structured-output")?,
                ConsumerRouteValueKind::StructuredDeclarations,
                bounded("schema dialect and bounded schema document the exact route accepts"),
                ConsumerRouteOmissionSemantics::SuppliesNothing,
            );
        }
        if self.request().attachments().len() > 0 {
            builder.push_session_start_control(
                exec_control(segment, "control.attachments")?,
                ConsumerRouteValueKind::StructuredDeclarations,
                bounded("bounded attachment media, count, and byte constraints"),
                ConsumerRouteOmissionSemantics::SuppliesNothing,
            );
        }
        builder.push_session_start_control(
            exec_control(segment, "control.external-network-policy")?,
            ConsumerRouteValueKind::BoundedPolicy,
            exact(network_policy(policy.external_network()))?,
            ConsumerRouteOmissionSemantics::Required,
        );
        builder.push_session_start_control(
            exec_control(segment, "control.external-search-policy")?,
            ConsumerRouteValueKind::BoundedPolicy,
            exact(search_policy(policy.external_search()))?,
            ConsumerRouteOmissionSemantics::Required,
        );
        if let Some(verbosity) = self.model_verbosity() {
            builder.push_session_start_control(
                exec_control(segment, "control.model-verbosity")?,
                ConsumerRouteValueKind::BoundedEnumeration,
                exact(verbosity.as_str())?,
                ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
            );
        }
        builder.build()
    }

    /// Returns the qualified behavior revision the prepared run is bound to.
    fn behavior_segment(&self) -> &str {
        match self.evidence().observation().compatibility() {
            InstalledExecutableCompatibility::Qualified(assessment) => {
                assessment.behavior_revision().as_str()
            }
            InstalledExecutableCompatibility::UnverifiedNewer(assessment) => {
                assessment.behavior_revision().as_str()
            }
            InstalledExecutableCompatibility::Incompatible => {
                unreachable!("incompatible executable cannot be prepared")
            }
        }
    }
}

/// Names one exec-local control by exact route, behavior revision, and id.
fn exec_control(
    segment: &str,
    semantic_id: &str,
) -> Result<ConsumerRouteControlId, ConsumerRouteProjectionFailure> {
    ConsumerRouteNamespacedExtension::new(EXEC_ROUTE, segment, semantic_id)
        .map(ConsumerRouteControlId::Namespaced)
}

/// Publishes the exact prepared value as the only admitted domain member.
fn exact(value: &str) -> Result<ConsumerRouteValueDomain, ConsumerRouteProjectionFailure> {
    Ok(ConsumerRouteValueDomain::Enumerated(
        ConsumerRouteEnumeratedValues::new([ConsumerRouteEnumerableValue::new(value)?])?,
    ))
}

const fn network_policy(policy: ExternalNetworkPolicy) -> &'static str {
    match policy {
        ExternalNetworkPolicy::Denied => "Denied",
        ExternalNetworkPolicy::HostApproved => "HostApproved",
        ExternalNetworkPolicy::AmbientHost => "AmbientHost",
    }
}

const fn search_policy(policy: ExternalSearchPolicy) -> &'static str {
    match policy {
        ExternalSearchPolicy::Disabled => "Disabled",
        ExternalSearchPolicy::Enabled => "Enabled",
    }
}
