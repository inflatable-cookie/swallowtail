mod lifecycle;
mod turn;

use self::lifecycle::{ActiveSlot, SessionCancellation, close_active};
use crate::budgets::QwenHeadlessBudgets;
use crate::validation::{failure, unsupported};
use crate::{DRIVER_ID, QwenHeadlessDriver};
use std::sync::{Arc, Mutex};
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, HarnessConfigurationPosture,
    HarnessIsolation, HarnessMode, InstanceOwnership, PreflightPlan, ReasoningMode, ResourceAccess,
    SessionProviderStatePolicy, SessionRef,
};
use swallowtail_runtime::{
    BoxFuture, CancellationControl, CleanupOutcome, HostServices, InteractiveSessionDriver,
    InteractiveSessionHandle, OpenSessionRequest, RequestId, ResumeSessionRequest, RuntimeFailure,
    RuntimeSessionId, SessionAccessPolicy, SessionResumeBinding, TurnHandle, TurnRequest,
    validate_session_plan_agreement,
};

pub(super) struct SessionState {
    pub(super) provider_session_id: Option<String>,
    pub(super) completed_turns: u32,
    pub(super) usable: bool,
}

pub(super) struct QwenSessionHandle {
    request_id: RequestId,
    runtime_id: RuntimeSessionId,
    pub(super) model: swallowtail_core::ModelId,
    pub(super) expected_version: swallowtail_core::InterfaceVersion,
    pub(super) working_resource: swallowtail_runtime::WorkingResourceRef,
    pub(super) services: HostServices,
    pub(super) state: Arc<Mutex<SessionState>>,
    pub(super) active: ActiveSlot,
    cancellation: Arc<SessionCancellation>,
    pub(super) environment: swallowtail_runtime::EnvironmentRef,
    pub(super) target: swallowtail_core::InstanceTargetRef,
    pub(super) reasoning: Option<ReasoningMode>,
    pub(super) harness_mode: Option<HarnessMode>,
    pub(super) budgets: QwenHeadlessBudgets,
}

impl InteractiveSessionDriver for QwenHeadlessDriver {
    fn open_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            let selection = validate_open(&plan, &request, &services)?;
            crate::budgets::validate_runtime(&selection, self.budgets())?;
            let active = Arc::new(Mutex::new(None));
            let state = Arc::new(Mutex::new(SessionState {
                provider_session_id: None,
                completed_turns: 0,
                usable: true,
            }));
            let cancellation = Arc::new(SessionCancellation::new(
                Arc::clone(&active),
                Arc::clone(&state),
            ));
            let runtime_id =
                RuntimeSessionId::new(format!("qwen-headless:{}", request.request_id().as_str()))
                    .map_err(|_| {
                    failure(
                        "swallowtail.qwen.headless.session_id_invalid",
                        "Qwen runtime session identity was invalid",
                    )
                })?;
            Ok(Box::new(QwenSessionHandle {
                request_id: request.request_id().clone(),
                runtime_id,
                model: plan
                    .model_id()
                    .cloned()
                    .expect("validated Qwen model is present"),
                expected_version: selection.version().clone(),
                working_resource: request
                    .working_resource()
                    .cloned()
                    .expect("validated Qwen working resource is present"),
                services,
                state,
                active,
                cancellation,
                environment: self.environment().clone(),
                target: plan.instance_target_ref().clone(),
                reasoning: request.options().reasoning_mode().cloned(),
                harness_mode: request.options().harness_mode(),
                budgets: self.budgets(),
            }) as Box<dyn InteractiveSessionHandle>)
        })
    }

    fn resume_session(
        &self,
        _plan: PreflightPlan,
        _request: ResumeSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async { Err(unsupported("public session resume")) })
    }
}

impl InteractiveSessionHandle for QwenSessionHandle {
    fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    fn session_id(&self) -> &RuntimeSessionId {
        &self.runtime_id
    }

    fn provider_session_ref(&self) -> Option<&SessionRef> {
        None
    }

    fn resume_binding(&self) -> Option<&SessionResumeBinding> {
        None
    }

    fn start_turn<'a>(
        &'a mut self,
        request: TurnRequest,
        services: HostServices,
    ) -> BoxFuture<'a, Result<Box<dyn TurnHandle>, RuntimeFailure>> {
        Box::pin(async move { self.start_turn_inner(request, services).await })
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        self.cancellation.as_ref()
    }

    fn close(
        self: Box<Self>,
        request: swallowtail_runtime::SessionCleanupRequest,
        services: HostServices,
    ) -> BoxFuture<'static, CleanupOutcome> {
        let execution_host_id = self.services.execution_host_id().clone();
        swallowtail_runtime::bound_session_cleanup(
            execution_host_id,
            request,
            services,
            Box::pin(async move {
                self.state
                    .lock()
                    .expect("Qwen session lock poisoned")
                    .usable = false;
                close_active(&self.active).await
            }),
        )
    }
}

fn validate_open(
    plan: &PreflightPlan,
    request: &OpenSessionRequest,
    services: &HostServices,
) -> Result<crate::selection::QwenPlanSelection, RuntimeFailure> {
    if plan.driver_identity().id().as_str() != DRIVER_ID
        || plan.ownership() != InstanceOwnership::HostOwnedEphemeral
    {
        return Err(failure(
            "swallowtail.qwen.headless.session_plan_mismatch",
            "Qwen interactive session plan did not match the qualified driver",
        ));
    }
    match plan.credential_mechanism() {
        swallowtail_core::CredentialMechanism::ProviderSpecific(namespace)
            if namespace.as_str() == "qwen-code/delegated-harness-auth" => {}
        _ => {
            return Err(failure(
                "swallowtail.qwen.headless.session_access_rejected",
                "Qwen interactive session requires delegated harness access",
            ));
        }
    }
    if plan.endpoint_audience().as_str() != "qwen-code" {
        return Err(failure(
            "swallowtail.qwen.headless.session_access_rejected",
            "Qwen interactive session endpoint audience did not match",
        ));
    }
    let selection = crate::selection::validate_qwen_plan_version(plan)?;
    services.require_execution_host(plan.execution_host_id())?;
    if services.task().is_none() || services.process().is_none() || services.time().is_none() {
        return Err(failure(
            "swallowtail.qwen.headless.host_service_missing",
            "Qwen interactive session required host services are unavailable",
        ));
    }
    validate_session_plan_agreement(plan, request.plan_agreement())?;
    if plan.requirements().operation_shape() != swallowtail_core::OperationShape::InteractiveSession
        || plan.requirements().driver_role() != swallowtail_core::DriverRole::InteractiveSession
        || plan.model_id().is_none()
        || plan.model_route_id().is_none()
        || plan.provider_id().is_none()
    {
        return Err(failure(
            "swallowtail.qwen.headless.session_binding_rejected",
            "Qwen interactive session requires one exact prepared model route",
        ));
    }
    let required = |capability| {
        plan.requirements()
            .capabilities()
            .find(|requirement| requirement.capability() == capability)
    };
    for capability in [
        Capability::InteractiveSession,
        Capability::StreamingEvents,
        Capability::ProviderDurableRetention,
    ] {
        if required(capability).is_none() {
            return Err(failure(
                "swallowtail.qwen.headless.session_capability_rejected",
                "Qwen interactive session capabilities were incomplete",
            ));
        }
    }
    let exact_constraint = |capability, constraint| {
        required(capability)
            .is_some_and(|requirement| requirement.constraints().eq(std::iter::once(&constraint)))
    };
    if !exact_constraint(
        Capability::InteractiveSession,
        CapabilityConstraint::MaximumTurns(24),
    ) || !exact_constraint(
        Capability::Interruption,
        CapabilityConstraint::CancellationScope(CancellationScope::ActiveTurn),
    ) || !exact_constraint(
        Capability::StreamingEvents,
        CapabilityConstraint::StreamRecordMaximumCount(4096),
    ) {
        return Err(failure(
            "swallowtail.qwen.headless.session_bound_rejected",
            "Qwen interactive session bounds did not match the qualified profile",
        ));
    }
    crate::reasoning::validate_runtime_binding(
        &selection,
        plan,
        request.options().reasoning_mode(),
    )?;
    crate::plan_mode::validate_runtime_binding(&selection, plan, request.options().harness_mode())?;
    if request.working_resource().is_none()
        || request.access_policy() != &SessionAccessPolicy::ambient_harness(ResourceAccess::Read)
        || request.provider_state_policy()
            != Some(SessionProviderStatePolicy::DurableProviderSessionPreserved)
        || request.harness_configuration_posture() != Some(HarnessConfigurationPosture::Ambient)
        || plan.requirements().harness_isolation() != Some(HarnessIsolation::AmbientHost)
        || request.options().developer_instructions().is_some()
        || request.options().tools().next().is_some()
        || request.options().idioms().is_some()
    {
        return Err(unsupported(
            "session access, provider state, configuration, or options",
        ));
    }
    if let Some(deadline) = request.deadline()
        && services.time().expect("validated Qwen time").now() >= deadline.instant()
    {
        return Err(failure(
            "swallowtail.qwen.headless.deadline_elapsed",
            "Qwen session deadline elapsed before opening",
        ));
    }
    Ok(selection)
}
