/// Request to open a fresh interactive provider session.
///
/// The embedded plan agreement carries the preflight decisions that the
/// runtime must preserve while preparing the session.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenSessionRequest {
    state: OpenSessionRequestState,
}

impl OpenSessionRequest {
    /// Creates a request bound to one working resource.
    pub fn new(
        request_id: RequestId,
        working_resource: WorkingResourceRef,
        deadline: Option<Deadline>,
        plan_agreement: SessionPlanAgreement,
    ) -> Self {
        Self::new_inner(request_id, working_resource, deadline, plan_agreement)
    }

    /// Creates a request for a route that does not require a working resource.
    pub fn resource_free(
        request_id: RequestId,
        deadline: Option<Deadline>,
        plan_agreement: SessionPlanAgreement,
    ) -> Self {
        Self::resource_free_inner(request_id, deadline, plan_agreement)
    }

    /// Creates a resource-bound request from an admitted preflight plan.
    pub fn from_plan(
        plan: &PreflightPlan,
        request_id: RequestId,
        working_resource: WorkingResourceRef,
        deadline: Option<Deadline>,
    ) -> Result<Self, PreparationFailure> {
        Self::from_plan_inner(plan, request_id, working_resource, deadline)
    }

    /// Creates a resource-free request from an admitted preflight plan.
    pub fn resource_free_from_plan(
        plan: &PreflightPlan,
        request_id: RequestId,
        deadline: Option<Deadline>,
    ) -> Result<Self, PreparationFailure> {
        Self::resource_free_from_plan_inner(plan, request_id, deadline)
    }

    /// Replaces the provider-facing session options.
    pub fn with_options(mut self, options: SessionOptions) -> Self {
        self.with_options_inner(options)
    }

    /// Returns the caller-assigned request identity.
    pub const fn request_id(&self) -> &RequestId {
        self.request_id_inner()
    }

    /// Returns the working resource when the route requires one.
    pub const fn working_resource(&self) -> Option<&WorkingResourceRef> {
        self.working_resource_inner()
    }

    /// Returns the absolute session-open deadline when present.
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline_inner()
    }

    /// Returns the requested session options.
    pub const fn options(&self) -> &SessionOptions {
        self.options_inner()
    }

    /// Returns the access policy retained from preflight.
    pub const fn access_policy(&self) -> &SessionAccessPolicy {
        self.access_policy_inner()
    }

    /// Returns the provider-state policy retained from preflight when applicable.
    pub const fn provider_state_policy(&self) -> Option<SessionProviderStatePolicy> {
        self.provider_state_policy_inner()
    }

    /// Returns the harness-configuration posture retained from preflight.
    pub const fn harness_configuration_posture(&self) -> Option<HarnessConfigurationPosture> {
        self.harness_configuration_posture_inner()
    }

    /// Returns the complete immutable preflight agreement.
    pub const fn plan_agreement(&self) -> &SessionPlanAgreement {
        self.plan_agreement_inner()
    }
}

/// Request to resume an admitted provider session without loading replay.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResumeSessionRequest {
    state: ResumeSessionRequestState,
}

/// Request to load bounded replay and attach to an admitted provider session.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoadSessionRequest {
    state: LoadSessionRequestState,
}

impl LoadSessionRequest {
    /// Creates a load request bound to one working resource.
    pub fn new(
        request_id: RequestId,
        binding: SessionResumeBinding,
        working_resource: WorkingResourceRef,
        deadline: Option<Deadline>,
        plan_agreement: SessionPlanAgreement,
    ) -> Self {
        Self::new_inner(
            request_id,
            binding,
            working_resource,
            deadline,
            plan_agreement,
        )
    }

    /// Creates a resource-bound load request from an admitted preflight plan.
    pub fn from_plan(
        plan: &PreflightPlan,
        request_id: RequestId,
        binding: SessionResumeBinding,
        working_resource: WorkingResourceRef,
        deadline: Option<Deadline>,
    ) -> Result<Self, PreparationFailure> {
        Self::from_plan_inner(plan, request_id, binding, working_resource, deadline)
    }

    /// Creates a load request for a route without a working resource.
    #[must_use]
    pub fn resource_free(
        request_id: RequestId,
        binding: SessionResumeBinding,
        deadline: Option<Deadline>,
        plan_agreement: SessionPlanAgreement,
    ) -> Self {
        Self::resource_free_inner(request_id, binding, deadline, plan_agreement)
    }

    /// Creates a resource-free load request from an admitted preflight plan.
    pub fn resource_free_from_plan(
        plan: &PreflightPlan,
        request_id: RequestId,
        binding: SessionResumeBinding,
        deadline: Option<Deadline>,
    ) -> Result<Self, PreparationFailure> {
        Self::resource_free_from_plan_inner(plan, request_id, binding, deadline)
    }

    /// Replaces the provider-facing session options.
    pub fn with_options(mut self, options: SessionOptions) -> Self {
        self.with_options_inner(options)
    }

    /// Returns the caller-assigned request identity.
    pub const fn request_id(&self) -> &RequestId {
        self.request_id_inner()
    }

    /// Returns the provider-session reference carried by the resume binding.
    pub const fn provider_session_ref(&self) -> &SessionRef {
        self.provider_session_ref_inner()
    }

    /// Returns the exact resume authority supplied by the consumer.
    pub const fn resume_binding(&self) -> &SessionResumeBinding {
        self.resume_binding_inner()
    }

    /// Returns the working resource when the route requires one.
    pub const fn working_resource(&self) -> Option<&WorkingResourceRef> {
        self.working_resource_inner()
    }

    /// Returns the absolute load deadline when present.
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline_inner()
    }

    /// Returns the requested session options.
    pub const fn options(&self) -> &SessionOptions {
        self.options_inner()
    }

    /// Returns the access policy retained from preflight.
    pub const fn access_policy(&self) -> &SessionAccessPolicy {
        self.access_policy_inner()
    }

    /// Returns the provider-state policy retained from preflight when applicable.
    pub const fn provider_state_policy(&self) -> Option<SessionProviderStatePolicy> {
        self.provider_state_policy_inner()
    }

    /// Returns the harness-configuration posture retained from preflight.
    pub const fn harness_configuration_posture(&self) -> Option<HarnessConfigurationPosture> {
        self.harness_configuration_posture_inner()
    }

    /// Returns the complete immutable preflight agreement.
    pub const fn plan_agreement(&self) -> &SessionPlanAgreement {
        self.plan_agreement_inner()
    }
}

/// Result of loading bounded replay before returning a live session handle.
pub struct LoadedSession {
    state: LoadedSessionState,
}

impl LoadedSession {
    /// Combines ordered replay evidence with the attached live session.
    pub fn new(replay: Vec<SessionReplayItem>, session: Box<dyn InteractiveSessionHandle>) -> Self {
        Self::new_inner(replay, session)
    }

    /// Iterates over replay items in provider order.
    pub fn replay(&self) -> impl ExactSizeIterator<Item = &SessionReplayItem> {
        self.replay_inner()
    }

    /// Separates replay evidence from the live session handle.
    pub fn into_parts(self) -> (Vec<SessionReplayItem>, Box<dyn InteractiveSessionHandle>) {
        self.into_parts_inner()
    }
}

impl ResumeSessionRequest {
    /// Creates a resume request bound to the original working resource.
    pub fn new(
        request_id: RequestId,
        binding: SessionResumeBinding,
        working_resource: WorkingResourceRef,
        deadline: Option<Deadline>,
        plan_agreement: SessionPlanAgreement,
    ) -> Self {
        Self::new_inner(
            request_id,
            binding,
            working_resource,
            deadline,
            plan_agreement,
        )
    }

    /// Creates a resume request from an admitted preflight plan.
    pub fn from_plan(
        plan: &PreflightPlan,
        request_id: RequestId,
        binding: SessionResumeBinding,
        working_resource: WorkingResourceRef,
        deadline: Option<Deadline>,
    ) -> Result<Self, PreparationFailure> {
        Self::from_plan_inner(plan, request_id, binding, working_resource, deadline)
    }

    /// Replaces the provider-facing session options.
    pub fn with_options(mut self, options: SessionOptions) -> Self {
        self.with_options_inner(options)
    }

    /// Returns the caller-assigned request identity.
    pub const fn request_id(&self) -> &RequestId {
        self.request_id_inner()
    }

    /// Returns the provider-session reference carried by the resume binding.
    pub const fn provider_session_ref(&self) -> &SessionRef {
        self.provider_session_ref_inner()
    }

    /// Returns the exact resume authority supplied by the consumer.
    pub const fn resume_binding(&self) -> &SessionResumeBinding {
        self.resume_binding_inner()
    }

    /// Returns the working resource retained for the resumed session.
    pub const fn working_resource(&self) -> &WorkingResourceRef {
        self.working_resource_inner()
    }

    /// Returns the absolute resume deadline when present.
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline_inner()
    }

    /// Returns the requested session options.
    pub const fn options(&self) -> &SessionOptions {
        self.options_inner()
    }

    /// Returns the access policy retained from preflight.
    pub const fn access_policy(&self) -> &SessionAccessPolicy {
        self.access_policy_inner()
    }

    /// Returns the provider-state policy retained from preflight when applicable.
    pub const fn provider_state_policy(&self) -> Option<SessionProviderStatePolicy> {
        self.provider_state_policy_inner()
    }

    /// Returns the harness-configuration posture retained from preflight.
    pub const fn harness_configuration_posture(&self) -> Option<HarnessConfigurationPosture> {
        self.harness_configuration_posture_inner()
    }

    /// Returns the complete immutable preflight agreement.
    pub const fn plan_agreement(&self) -> &SessionPlanAgreement {
        self.plan_agreement_inner()
    }
}

