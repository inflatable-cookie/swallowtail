#[derive(Clone, Debug, Eq, PartialEq)]
struct OpenSessionRequestState {
    request_id: RequestId,
    working_resource: Option<WorkingResourceRef>,
    deadline: Option<Deadline>,
    options: SessionOptions,
    plan_agreement: SessionPlanAgreement,
}

impl OpenSessionRequest {
    fn new_inner(
        request_id: RequestId,
        working_resource: WorkingResourceRef,
        deadline: Option<Deadline>,
        plan_agreement: SessionPlanAgreement,
    ) -> Self {
        Self {
            state: OpenSessionRequestState {
            request_id,
            working_resource: Some(working_resource),
            deadline,
            options: SessionOptions::default(),
            plan_agreement,
            },
        }
    }

    #[must_use]
    fn resource_free_inner(
        request_id: RequestId,
        deadline: Option<Deadline>,
        plan_agreement: SessionPlanAgreement,
    ) -> Self {
        Self {
            state: OpenSessionRequestState {
            request_id,
            working_resource: None,
            deadline,
            options: SessionOptions::default(),
            plan_agreement,
            },
        }
    }

    fn from_plan_inner(
        plan: &PreflightPlan,
        request_id: RequestId,
        working_resource: WorkingResourceRef,
        deadline: Option<Deadline>,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self::new(
            request_id,
            working_resource,
            deadline,
            SessionPlanAgreement::from_plan(plan)?,
        ))
    }

    fn resource_free_from_plan_inner(
        plan: &PreflightPlan,
        request_id: RequestId,
        deadline: Option<Deadline>,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self::resource_free(
            request_id,
            deadline,
            SessionPlanAgreement::from_plan(plan)?,
        ))
    }

    #[must_use]
    fn with_options_inner(mut self, options: SessionOptions) -> Self {
        self.state.options = options;
        self
    }

    #[must_use]
    const fn request_id_inner(&self) -> &RequestId {
        &self.state.request_id
    }

    #[must_use]
    const fn working_resource_inner(&self) -> Option<&WorkingResourceRef> {
        self.state.working_resource.as_ref()
    }

    #[must_use]
    const fn deadline_inner(&self) -> Option<Deadline> {
        self.state.deadline
    }

    #[must_use]
    const fn options_inner(&self) -> &SessionOptions {
        &self.state.options
    }

    #[must_use]
    const fn access_policy_inner(&self) -> &SessionAccessPolicy {
        self.state.plan_agreement.access_policy()
    }

    #[must_use]
    const fn provider_state_policy_inner(&self) -> Option<SessionProviderStatePolicy> {
        self.state.plan_agreement.provider_state_policy()
    }

    #[must_use]
    const fn harness_configuration_posture_inner(&self) -> Option<HarnessConfigurationPosture> {
        self.state.plan_agreement.harness_configuration_posture()
    }

    #[must_use]
    const fn plan_agreement_inner(&self) -> &SessionPlanAgreement {
        &self.state.plan_agreement
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LoadSessionRequestState {
    request_id: RequestId,
    binding: SessionResumeBinding,
    working_resource: Option<WorkingResourceRef>,
    deadline: Option<Deadline>,
    options: SessionOptions,
    plan_agreement: SessionPlanAgreement,
}

impl LoadSessionRequest {
    fn new_inner(
        request_id: RequestId,
        binding: SessionResumeBinding,
        working_resource: WorkingResourceRef,
        deadline: Option<Deadline>,
        plan_agreement: SessionPlanAgreement,
    ) -> Self {
        Self {
            state: LoadSessionRequestState {
            request_id,
            binding,
            working_resource: Some(working_resource),
            deadline,
            options: SessionOptions::default(),
            plan_agreement,
            },
        }
    }

    #[must_use]
    fn resource_free_inner(
        request_id: RequestId,
        binding: SessionResumeBinding,
        deadline: Option<Deadline>,
        plan_agreement: SessionPlanAgreement,
    ) -> Self {
        Self {
            state: LoadSessionRequestState {
                request_id,
                binding,
                working_resource: None,
                deadline,
                options: SessionOptions::default(),
                plan_agreement,
            },
        }
    }

    fn from_plan_inner(
        plan: &PreflightPlan,
        request_id: RequestId,
        binding: SessionResumeBinding,
        working_resource: WorkingResourceRef,
        deadline: Option<Deadline>,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self::new(
            request_id,
            binding,
            working_resource,
            deadline,
            SessionPlanAgreement::from_plan(plan)?,
        ))
    }

    fn resource_free_from_plan_inner(
        plan: &PreflightPlan,
        request_id: RequestId,
        binding: SessionResumeBinding,
        deadline: Option<Deadline>,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self::resource_free(
            request_id,
            binding,
            deadline,
            SessionPlanAgreement::from_plan(plan)?,
        ))
    }

    #[must_use]
    fn with_options_inner(mut self, options: SessionOptions) -> Self {
        self.state.options = options;
        self
    }

    #[must_use]
    const fn request_id_inner(&self) -> &RequestId {
        &self.state.request_id
    }
    #[must_use]
    const fn provider_session_ref_inner(&self) -> &SessionRef {
        self.state.binding.provider_session_ref()
    }
    #[must_use]
    const fn resume_binding_inner(&self) -> &SessionResumeBinding {
        &self.state.binding
    }
    #[must_use]
    const fn working_resource_inner(&self) -> Option<&WorkingResourceRef> {
        self.state.working_resource.as_ref()
    }
    #[must_use]
    const fn deadline_inner(&self) -> Option<Deadline> {
        self.state.deadline
    }
    #[must_use]
    const fn options_inner(&self) -> &SessionOptions {
        &self.state.options
    }
    #[must_use]
    const fn access_policy_inner(&self) -> &SessionAccessPolicy {
        self.state.plan_agreement.access_policy()
    }
    #[must_use]
    const fn provider_state_policy_inner(&self) -> Option<SessionProviderStatePolicy> {
        self.state.plan_agreement.provider_state_policy()
    }
    #[must_use]
    const fn harness_configuration_posture_inner(&self) -> Option<HarnessConfigurationPosture> {
        self.state.plan_agreement.harness_configuration_posture()
    }
    #[must_use]
    const fn plan_agreement_inner(&self) -> &SessionPlanAgreement {
        &self.state.plan_agreement
    }
}

struct LoadedSessionState {
    replay: Vec<SessionReplayItem>,
    session: Box<dyn InteractiveSessionHandle>,
}

impl LoadedSession {
    fn new_inner(replay: Vec<SessionReplayItem>, session: Box<dyn InteractiveSessionHandle>) -> Self {
        Self { state: LoadedSessionState { replay, session } }
    }

    fn replay_inner(&self) -> impl ExactSizeIterator<Item = &SessionReplayItem> {
        self.state.replay.iter()
    }

    #[must_use]
    fn into_parts_inner(self) -> (Vec<SessionReplayItem>, Box<dyn InteractiveSessionHandle>) {
        (self.state.replay, self.state.session)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ResumeSessionRequestState {
    request_id: RequestId,
    binding: SessionResumeBinding,
    working_resource: WorkingResourceRef,
    deadline: Option<Deadline>,
    options: SessionOptions,
    plan_agreement: SessionPlanAgreement,
}

impl ResumeSessionRequest {
    fn new_inner(
        request_id: RequestId,
        binding: SessionResumeBinding,
        working_resource: WorkingResourceRef,
        deadline: Option<Deadline>,
        plan_agreement: SessionPlanAgreement,
    ) -> Self {
        Self {
            state: ResumeSessionRequestState {
            request_id,
            binding,
            working_resource,
            deadline,
            options: SessionOptions::default(),
            plan_agreement,
            },
        }
    }

    fn from_plan_inner(
        plan: &PreflightPlan,
        request_id: RequestId,
        binding: SessionResumeBinding,
        working_resource: WorkingResourceRef,
        deadline: Option<Deadline>,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self::new(
            request_id,
            binding,
            working_resource,
            deadline,
            SessionPlanAgreement::from_plan(plan)?,
        ))
    }

    #[must_use]
    fn with_options_inner(mut self, options: SessionOptions) -> Self {
        self.state.options = options;
        self
    }

    #[must_use]
    const fn request_id_inner(&self) -> &RequestId {
        &self.state.request_id
    }

    #[must_use]
    const fn provider_session_ref_inner(&self) -> &SessionRef {
        self.state.binding.provider_session_ref()
    }

    #[must_use]
    const fn resume_binding_inner(&self) -> &SessionResumeBinding {
        &self.state.binding
    }

    #[must_use]
    const fn working_resource_inner(&self) -> &WorkingResourceRef {
        &self.state.working_resource
    }

    #[must_use]
    const fn deadline_inner(&self) -> Option<Deadline> {
        self.state.deadline
    }

    #[must_use]
    const fn options_inner(&self) -> &SessionOptions {
        &self.state.options
    }

    #[must_use]
    const fn access_policy_inner(&self) -> &SessionAccessPolicy {
        self.state.plan_agreement.access_policy()
    }
    #[must_use]
    const fn provider_state_policy_inner(&self) -> Option<SessionProviderStatePolicy> {
        self.state.plan_agreement.provider_state_policy()
    }
    #[must_use]
    const fn harness_configuration_posture_inner(&self) -> Option<HarnessConfigurationPosture> {
        self.state.plan_agreement.harness_configuration_posture()
    }
    #[must_use]
    const fn plan_agreement_inner(&self) -> &SessionPlanAgreement {
        &self.state.plan_agreement
    }
}
