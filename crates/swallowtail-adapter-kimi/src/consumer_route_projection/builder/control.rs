use super::Projection;
use swallowtail_runtime::{
    ConsumerRouteControlId, ConsumerRouteFeatureId, ConsumerRouteOmissionSemantics,
    ConsumerRouteStateSupport, ConsumerRouteValueKind,
};

impl Projection<'_> {
    pub(in crate::consumer_route_projection) fn model_selection(mut self) -> Self {
        let Some(model_id) = self
            .applicability
            .model()
            .map(|model| model.model_id().as_str().to_owned())
        else {
            return self;
        };
        let Some(domain) = self.exact(&model_id) else {
            return self;
        };
        self.push_control(
            ConsumerRouteControlId::ModelSelection,
            ConsumerRouteValueKind::ExactModelRoute,
            domain,
            ConsumerRouteOmissionSemantics::Required,
        );
        self
    }

    pub(in crate::consumer_route_projection) fn reasoning_control(
        mut self,
        value: Option<&str>,
        pending: bool,
    ) -> Self {
        let Some(value) = value else { return self };
        let Some(domain) = self.exact(value) else {
            return self;
        };
        let mut state = ConsumerRouteStateSupport::descriptor_only()
            .with_requested()
            .with_prepared();
        if pending {
            state = state.with_pending();
        }
        let row = self
            .control_row(
                ConsumerRouteControlId::ReasoningSelection,
                ConsumerRouteValueKind::AcknowledgedEnumeration,
                domain,
                ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
            )
            .with_state_support(state);
        self.session_start.push(row);
        self
    }

    pub(in crate::consumer_route_projection) fn session_options(
        mut self,
        descriptor: &str,
    ) -> Self {
        let Some(domain) = self.bounded(descriptor) else {
            return self;
        };
        let row = self.control_row(
            ConsumerRouteControlId::SessionOptions,
            ConsumerRouteValueKind::FixedStructuredConfig,
            domain,
            ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
        );
        self.session_start.push(row);
        self
    }

    pub(in crate::consumer_route_projection) fn named_control(
        mut self,
        semantic: &str,
        value: &str,
        omission: ConsumerRouteOmissionSemantics,
    ) -> Self {
        let (Some(control), Some(domain)) = (self.local_control(semantic), self.bounded(value))
        else {
            return self;
        };
        self.push_control(
            control,
            ConsumerRouteValueKind::BoundedPolicy,
            domain,
            omission,
        );
        self
    }

    pub(in crate::consumer_route_projection) fn named_feature(mut self, semantic: &str) -> Self {
        if let Some(feature) = self.local_feature(semantic) {
            let row = self.prepared_feature(feature);
            self.selection.push(row);
        }
        self
    }

    pub(in crate::consumer_route_projection) fn portable_feature(
        mut self,
        feature: ConsumerRouteFeatureId,
    ) -> Self {
        let row = self.prepared_feature(feature);
        self.selection.push(row);
        self
    }

    pub(in crate::consumer_route_projection) fn portable_control(
        mut self,
        control: ConsumerRouteControlId,
        value: &str,
    ) -> Self {
        let Some(domain) = self.bounded(value) else {
            return self;
        };
        self.push_control(
            control,
            ConsumerRouteValueKind::BoundedPolicy,
            domain,
            ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
        );
        self
    }
}
