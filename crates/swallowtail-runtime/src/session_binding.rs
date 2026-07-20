use swallowtail_core::{
    ConfiguredInstanceId, ExecutionHostId, ModelId, ModelRouteId, PreflightPlan, SessionRef,
};

/// Durable provider-session identity plus the route that is allowed to resume it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionResumeBinding {
    provider_session_ref: SessionRef,
    configured_instance_id: ConfiguredInstanceId,
    execution_host_id: ExecutionHostId,
    model_route_id: ModelRouteId,
    model_id: ModelId,
}

impl SessionResumeBinding {
    #[must_use]
    pub const fn new(
        provider_session_ref: SessionRef,
        configured_instance_id: ConfiguredInstanceId,
        execution_host_id: ExecutionHostId,
        model_route_id: ModelRouteId,
        model_id: ModelId,
    ) -> Self {
        Self {
            provider_session_ref,
            configured_instance_id,
            execution_host_id,
            model_route_id,
            model_id,
        }
    }

    #[must_use]
    pub const fn provider_session_ref(&self) -> &SessionRef {
        &self.provider_session_ref
    }

    #[must_use]
    pub const fn configured_instance_id(&self) -> &ConfiguredInstanceId {
        &self.configured_instance_id
    }

    #[must_use]
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    #[must_use]
    pub const fn model_route_id(&self) -> &ModelRouteId {
        &self.model_route_id
    }

    #[must_use]
    pub const fn model_id(&self) -> &ModelId {
        &self.model_id
    }

    #[must_use]
    pub fn matches_plan(&self, plan: &PreflightPlan) -> bool {
        &self.configured_instance_id == plan.instance_id()
            && &self.execution_host_id == plan.execution_host_id()
            && plan.model_route_id() == Some(&self.model_route_id)
            && plan.model_id() == Some(&self.model_id)
    }
}

#[cfg(test)]
mod tests {
    use super::SessionResumeBinding;
    use swallowtail_core::{
        ConfiguredInstanceId, ExecutionHostId, ModelId, ModelRouteId, SessionRef,
    };

    #[test]
    fn provider_reference_stays_redacted_inside_resume_binding() {
        let binding = SessionResumeBinding::new(
            SessionRef::new("provider/private/thread").expect("provider ref is valid"),
            ConfiguredInstanceId::new("instance.one").expect("instance id is valid"),
            ExecutionHostId::new("host.one").expect("host id is valid"),
            ModelRouteId::new("route.one").expect("route id is valid"),
            ModelId::new("model.one").expect("model id is valid"),
        );

        assert!(!format!("{binding:?}").contains("provider/private/thread"));
    }
}
