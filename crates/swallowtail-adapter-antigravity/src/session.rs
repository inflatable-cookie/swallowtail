mod lifecycle;
mod turn;

use self::lifecycle::{ActiveSlot, SessionCancellation, close_active};
use crate::failure::{failure, unsupported};
use crate::{AntigravityHeadlessDriver, HEADLESS_DRIVER_ID};
use std::sync::{Arc, Mutex};
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, CredentialMechanism,
    HarnessConfigurationPosture, HarnessIsolation, InstanceOwnership, PreflightPlan,
    ResourceAccess, ResourceRepresentation, SessionProviderStatePolicy, SessionRef,
    SupportAuthority,
};
use swallowtail_runtime::{
    BoxFuture, CancellationControl, CleanupOutcome, HostServices, InteractiveSessionDriver,
    InteractiveSessionHandle, OpenSessionRequest, RequestId, ResumeSessionRequest, RuntimeFailure,
    RuntimeSessionId, SessionAccessPolicy, SessionResumeBinding, TurnHandle, TurnRequest,
    validate_session_plan_agreement,
};

pub(super) struct SessionState {
    pub(super) conversation_id: Option<String>,
    pub(super) completed_turns: u32,
    pub(super) usable: bool,
}

pub(super) struct AntigravitySessionHandle {
    request_id: RequestId,
    runtime_id: RuntimeSessionId,
    pub(super) model: swallowtail_core::ModelId,
    pub(super) working_resource: swallowtail_runtime::WorkingResourceRef,
    pub(super) services: HostServices,
    pub(super) state: Arc<Mutex<SessionState>>,
    pub(super) active: ActiveSlot,
    cancellation: Arc<SessionCancellation>,
    pub(super) environment: swallowtail_runtime::EnvironmentRef,
    pub(super) target: swallowtail_core::InstanceTargetRef,
}

impl InteractiveSessionDriver for AntigravityHeadlessDriver {
    fn open_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            validate_open(&plan, &request, &services)?;
            let active = Arc::new(Mutex::new(None));
            let state = Arc::new(Mutex::new(SessionState {
                conversation_id: None,
                completed_turns: 0,
                usable: true,
            }));
            let cancellation = Arc::new(SessionCancellation::new(
                Arc::clone(&active),
                Arc::clone(&state),
            ));
            let runtime_id = RuntimeSessionId::new(format!(
                "antigravity-headless:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| {
                failure(
                    "swallowtail.antigravity.headless.session_id_invalid",
                    "Antigravity runtime session identity was invalid",
                )
            })?;
            Ok(Box::new(AntigravitySessionHandle {
                request_id: request.request_id().clone(),
                runtime_id,
                model: plan
                    .model_id()
                    .cloned()
                    .expect("validated model is present"),
                working_resource: request
                    .working_resource()
                    .cloned()
                    .expect("validated working resource is present"),
                services,
                state,
                active,
                cancellation,
                environment: self.environment().clone(),
                target: plan.instance_target_ref().clone(),
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

impl InteractiveSessionHandle for AntigravitySessionHandle {
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
                    .expect("Antigravity session lock poisoned")
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
) -> Result<(), RuntimeFailure> {
    if plan.driver_identity().id().as_str() != HEADLESS_DRIVER_ID
        || plan.ownership() != InstanceOwnership::HostOwnedEphemeral
        || plan.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
        || plan.credential_reference().is_some()
        || plan.endpoint_audience().as_str() != crate::ANTIGRAVITY_PERSONAL_GOOGLE_AUDIENCE
        || plan.access_status().support_authority() != SupportAuthority::ProviderSupported
    {
        return Err(plan_mismatch("driver or access"));
    }
    crate::selection::validate_antigravity_headless_plan(plan)?;
    services.require_execution_host(plan.execution_host_id())?;
    if services.task().is_none()
        || services.process().is_none()
        || services.time().is_none()
        || services.working_resource().is_none()
    {
        return Err(failure(
            "swallowtail.antigravity.headless.host_service_missing",
            "Antigravity continuation requires task, process, time, and working-resource services",
        ));
    }
    validate_session_plan_agreement(plan, request.plan_agreement())?;
    if plan.requirements().operation_shape() != swallowtail_core::OperationShape::InteractiveSession
        || plan.requirements().driver_role() != swallowtail_core::DriverRole::InteractiveSession
        || plan.model_id().is_none()
        || plan.model_route_id().is_none()
        || plan.provider_id().is_none()
    {
        return Err(plan_mismatch("operation or model route"));
    }
    for capability in [
        Capability::InteractiveSession,
        Capability::StreamingEvents,
        Capability::ObservableActivity,
        Capability::UsageReporting,
        Capability::ProviderDurableRetention,
    ] {
        require_capability(plan, capability)?;
    }
    require_constraint(
        plan,
        Capability::InteractiveSession,
        CapabilityConstraint::MaximumTurns(24),
    )?;
    require_constraint(
        plan,
        Capability::StreamingEvents,
        CapabilityConstraint::StreamRecordMaximumCount(4096),
    )?;
    require_constraint(
        plan,
        Capability::Interruption,
        CapabilityConstraint::CancellationScope(CancellationScope::ActiveTurn),
    )?;
    require_constraint(
        plan,
        Capability::WorkingResource,
        CapabilityConstraint::ResourceAccess(ResourceAccess::Read),
    )?;
    require_constraint(
        plan,
        Capability::WorkingResource,
        CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
    )?;
    if request.working_resource().is_none()
        || request.access_policy() != &SessionAccessPolicy::ambient_harness(ResourceAccess::Read)
        || request.provider_state_policy()
            != Some(SessionProviderStatePolicy::DurableProviderSessionPreserved)
        || request.harness_configuration_posture() != Some(HarnessConfigurationPosture::Ambient)
        || plan.requirements().harness_isolation() != Some(HarnessIsolation::AmbientHost)
        || !request.options().is_empty()
    {
        return Err(unsupported(
            "session access, provider state, configuration, isolation, or options",
        ));
    }
    if let Some(deadline) = request.deadline()
        && services.time().expect("validated time service").now() >= deadline.instant()
    {
        return Err(failure(
            "swallowtail.antigravity.headless.deadline_elapsed",
            "Antigravity session deadline elapsed before opening",
        ));
    }
    Ok(())
}

fn require_capability(plan: &PreflightPlan, capability: Capability) -> Result<(), RuntimeFailure> {
    if plan
        .requirements()
        .capabilities()
        .any(|required| required.capability() == capability)
    {
        Ok(())
    } else {
        Err(plan_mismatch("capability"))
    }
}

fn require_constraint(
    plan: &PreflightPlan,
    capability: Capability,
    constraint: CapabilityConstraint,
) -> Result<(), RuntimeFailure> {
    if plan.requirements().capabilities().any(|required| {
        required.capability() == capability
            && required
                .constraints()
                .any(|required| required == &constraint)
    }) {
        Ok(())
    } else {
        Err(plan_mismatch("capability constraint"))
    }
}

fn plan_mismatch(dimension: &str) -> RuntimeFailure {
    failure(
        "swallowtail.antigravity.headless.session_plan_mismatch",
        format!("Antigravity continuation plan did not match {dimension}"),
    )
}
