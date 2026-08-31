use super::{CodexProjectionBuilder, bounded};
use crate::{
    CodexPreparedArchive, CodexPreparedCatalogue, CodexPreparedDelete, CodexPreparedRestore,
    CodexPreparedSession, CodexPreparedSessionCatalogue, CodexPreparedSessionHistory,
    CodexPreparedSessionImport, CodexPreparedSessionReconciliation,
};
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteOmissionSemantics, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailure, ConsumerRouteProjectionSourceId, ConsumerRouteValueKind,
};

/// Result of one prepared Codex contribution request.
type Contribution = Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure>;

macro_rules! plan_contribution {
    ($facade:ty, $doc:expr, $($plan:tt)*) => {
        impl $facade {
            #[doc = $doc]
            pub fn consumer_route_projection_contribution(
                &self,
                source_id: ConsumerRouteProjectionSourceId,
            ) -> Contribution {
                CodexProjectionBuilder::new(self.$($plan)*, source_id)
                    .with_prepared_capabilities()
                    .with_question_exchange()
                    .with_model_selection()
                    .build()
            }
        }
    };
}

plan_contribution!(
    CodexPreparedCatalogue,
    "Emits only the model-catalogue truth this prepared operation proves.",
    plan()
);
plan_contribution!(
    CodexPreparedSessionImport,
    "Emits only the session-import truth this prepared operation proves.",
    plan().preflight()
);
plan_contribution!(
    CodexPreparedArchive,
    "Emits only the archive truth this prepared operation proves.",
    plan().preflight()
);
plan_contribution!(
    CodexPreparedRestore,
    "Emits only the restore truth this prepared operation proves.",
    plan().preflight()
);
plan_contribution!(
    CodexPreparedDelete,
    "Emits only the delete truth this prepared operation proves.",
    plan().preflight()
);

impl CodexPreparedSession {
    /// Emits only the interactive-session truth this prepared session proves.
    ///
    /// Session-start controls come from the bound open request. The per-turn
    /// user-input exchange stays per-turn and claims no provider mutation.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Contribution {
        let mut builder = CodexProjectionBuilder::new(self.plan(), source_id)
            .with_prepared_capabilities()
            .with_question_exchange()
            .with_model_selection();
        let options = self.request().options();
        builder.push_session_start_control(
            ConsumerRouteControlId::SessionOptions,
            ConsumerRouteValueKind::StructuredOptions,
            bounded("developer instructions, reasoning, harness, tools, and idioms"),
            ConsumerRouteOmissionSemantics::SuppliesNothing,
        );
        if options.reasoning_mode().is_some() {
            builder.push_session_start_control(
                ConsumerRouteControlId::ReasoningSelection,
                ConsumerRouteValueKind::BoundedEnumeration,
                bounded("route-qualified Codex session reasoning"),
                ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
            );
        }
        if options.tools().len() > 0 {
            builder.push_session_start_control(
                ConsumerRouteControlId::ToolDeclarations,
                ConsumerRouteValueKind::StructuredDeclarations,
                bounded("bounded declarations with schema and media constraints"),
                ConsumerRouteOmissionSemantics::SuppliesNothing,
            );
        }
        if options.developer_instructions().is_some() {
            builder.push_session_start_control(
                ConsumerRouteControlId::DeveloperInstructions,
                ConsumerRouteValueKind::StructuredContent,
                bounded("bounded developer instruction content"),
                ConsumerRouteOmissionSemantics::SuppliesNothing,
            );
        }
        if options.idioms().is_some() {
            builder.push_session_start_control(
                ConsumerRouteControlId::Idioms,
                ConsumerRouteValueKind::BoundedOption,
                bounded("runtime idiom values the exact route accepts"),
                ConsumerRouteOmissionSemantics::SuppliesNothing,
            );
        }
        if self
            .plan()
            .requirements()
            .session_access_policy()
            .is_some_and(|policy| policy.provider_requests().exchanged_extensions().len() > 0)
        {
            builder.push_per_turn_exchange();
        }
        builder.push_session_start_control(
            ConsumerRouteControlId::LoadSession,
            ConsumerRouteValueKind::LifecycleAction,
            bounded("exact provider session identity and prepared route binding"),
            ConsumerRouteOmissionSemantics::SuppliesNothing,
        );
        builder.push_session_start_control(
            ConsumerRouteControlId::ResumeSession,
            ConsumerRouteValueKind::LifecycleAction,
            bounded("exact provider session identity and prepared route binding"),
            ConsumerRouteOmissionSemantics::SuppliesNothing,
        );
        builder.build()
    }
}

impl CodexPreparedSessionCatalogue {
    /// Emits only the session-catalogue truth this prepared operation proves.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Contribution {
        let mut builder = CodexProjectionBuilder::new(self.plan().preflight(), source_id)
            .with_prepared_capabilities()
            .with_question_exchange()
            .with_model_selection();
        builder.push_observed_query(
            ConsumerRouteControlId::SessionCatalogueBounds,
            "route-qualified provider session catalogue bounds",
        );
        builder.build()
    }
}

impl CodexPreparedSessionHistory {
    /// Emits only the session-history truth this prepared operation proves.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Contribution {
        let mut builder = CodexProjectionBuilder::new(self.plan().preflight(), source_id)
            .with_prepared_capabilities()
            .with_question_exchange()
            .with_model_selection();
        builder.push_observed_query(
            ConsumerRouteControlId::SessionHistoryBounds,
            "route-qualified provider session history bounds",
        );
        builder.build()
    }
}

impl CodexPreparedSessionReconciliation {
    /// Emits only the reconciliation truth this prepared operation proves.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Contribution {
        let mut builder = CodexProjectionBuilder::new(self.plan().preflight(), source_id)
            .with_prepared_capabilities()
            .with_question_exchange()
            .with_model_selection();
        builder.push_observed_query(
            ConsumerRouteControlId::SessionReconciliation,
            "route-qualified provider session reconciliation bounds",
        );
        builder.build()
    }
}
