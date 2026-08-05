use crate::WorkingResourceRef;
use swallowtail_core::{
    ConfiguredInstanceId, ExecutionHostId, ModelId, ModelRouteId, PreflightPlan,
    ProviderSessionBindingOrigin, SessionAccessPolicy, SessionRef,
};

mod persistence;

pub(crate) use persistence::attachment_fingerprint_for_checkpoint;

pub use persistence::{
    PersistedSessionResumeBinding, SessionResumeBindingPersistenceFailure,
    SessionResumeBindingPersistenceFailureKind,
};

/// Durable provider-session identity plus the route that is allowed to resume it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionResumeBinding {
    provider_session_ref: SessionRef,
    configured_instance_id: ConfiguredInstanceId,
    execution_host_id: ExecutionHostId,
    model_route_id: Option<ModelRouteId>,
    model_id: Option<ModelId>,
    working_resource: Option<WorkingResourceRef>,
    access_policy: SessionAccessPolicy,
    origin: ProviderSessionBindingOrigin,
}

impl SessionResumeBinding {
    #[must_use]
    pub const fn new(
        provider_session_ref: SessionRef,
        configured_instance_id: ConfiguredInstanceId,
        execution_host_id: ExecutionHostId,
        model_route_id: ModelRouteId,
        model_id: ModelId,
        working_resource: WorkingResourceRef,
        access_policy: SessionAccessPolicy,
    ) -> Self {
        Self {
            provider_session_ref,
            configured_instance_id,
            execution_host_id,
            model_route_id: Some(model_route_id),
            model_id: Some(model_id),
            working_resource: Some(working_resource),
            access_policy,
            origin: ProviderSessionBindingOrigin::Created,
        }
    }

    /// Creates a binding for a route whose exact prepared posture has no
    /// selectable model identity.
    #[must_use]
    pub const fn without_model(
        provider_session_ref: SessionRef,
        configured_instance_id: ConfiguredInstanceId,
        execution_host_id: ExecutionHostId,
        working_resource: WorkingResourceRef,
        access_policy: SessionAccessPolicy,
    ) -> Self {
        Self {
            provider_session_ref,
            configured_instance_id,
            execution_host_id,
            model_route_id: None,
            model_id: None,
            working_resource: Some(working_resource),
            access_policy,
            origin: ProviderSessionBindingOrigin::Created,
        }
    }

    /// Creates a binding for an exact resource-free route.
    #[must_use]
    pub const fn resource_free(
        provider_session_ref: SessionRef,
        configured_instance_id: ConfiguredInstanceId,
        execution_host_id: ExecutionHostId,
        model_route_id: ModelRouteId,
        model_id: ModelId,
        access_policy: SessionAccessPolicy,
    ) -> Self {
        Self {
            provider_session_ref,
            configured_instance_id,
            execution_host_id,
            model_route_id: Some(model_route_id),
            model_id: Some(model_id),
            working_resource: None,
            access_policy,
            origin: ProviderSessionBindingOrigin::Created,
        }
    }

    pub(crate) const fn with_origin(mut self, origin: ProviderSessionBindingOrigin) -> Self {
        self.origin = origin;
        self
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
    pub const fn model_route_id(&self) -> Option<&ModelRouteId> {
        self.model_route_id.as_ref()
    }

    #[must_use]
    pub const fn model_id(&self) -> Option<&ModelId> {
        self.model_id.as_ref()
    }

    #[must_use]
    pub const fn working_resource(&self) -> Option<&WorkingResourceRef> {
        self.working_resource.as_ref()
    }

    #[must_use]
    pub const fn is_resource_free(&self) -> bool {
        self.working_resource.is_none()
    }

    #[must_use]
    pub const fn access_policy(&self) -> &SessionAccessPolicy {
        &self.access_policy
    }

    #[must_use]
    pub const fn origin(&self) -> ProviderSessionBindingOrigin {
        self.origin
    }

    #[must_use]
    pub fn matches_plan(&self, plan: &PreflightPlan) -> bool {
        &self.configured_instance_id == plan.instance_id()
            && &self.execution_host_id == plan.execution_host_id()
            && plan.model_route_id() == self.model_route_id.as_ref()
            && plan.model_id() == self.model_id.as_ref()
    }

    #[must_use]
    pub fn matches_attachment(
        &self,
        plan: &PreflightPlan,
        working_resource: &WorkingResourceRef,
        access_policy: &SessionAccessPolicy,
    ) -> bool {
        self.matches_plan(plan)
            && self.working_resource.as_ref() == Some(working_resource)
            && &self.access_policy == access_policy
    }

    #[must_use]
    pub fn matches_resource_free_attachment(
        &self,
        plan: &PreflightPlan,
        access_policy: &SessionAccessPolicy,
    ) -> bool {
        self.matches_plan(plan)
            && self.working_resource.is_none()
            && &self.access_policy == access_policy
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
            crate::WorkingResourceRef::new("resource.one").expect("resource ref is valid"),
            swallowtail_core::SessionAccessPolicy::ambient_harness(
                swallowtail_core::ResourceAccess::Read,
            ),
        );

        assert!(!format!("{binding:?}").contains("provider/private/thread"));
        assert_eq!(
            binding.origin(),
            swallowtail_core::ProviderSessionBindingOrigin::Created
        );
    }
}
