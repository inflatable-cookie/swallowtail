use super::access::{SecretMaterial, SessionAccess};
use super::checkpoint::{KimiCursorCheckpoint, decode};
use super::websocket::{Subscription, SubscriptionInput};
use crate::failure::failure;
use crate::local_server::protocol::{
    InteractiveSessionRecord, RestReply, TurnEndReason, WsEvent, WsFrame,
    decode_interactive_session, decode_rest, decode_ws_frame,
};
use crate::local_server::transport::{Request, session_path};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use swallowtail_core::{
    AccessRequirement, Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement,
    CredentialState, Diagnostic, DriverRole, EndpointAuthorization, EntitlementState,
    ExecutionLayer, HarnessConfigurationPosture, InstanceOwnership, ModelRoute,
    OperationRequirements, OperationShape, PreflightContext, ResourceAccess,
    ResourceRepresentation, RuntimeReadiness, SessionAccessPolicy, SessionProviderStatePolicy,
    preflight,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, HostServices, PersistedProviderOperationCheckpoint,
    PreparationFailure, PreparationStage, PreparedProviderSessionReconciliationEvidence,
    PreparedWorkingStateRestoration, ProviderOperationCheckpoint,
    ProviderSessionReconciliationAgreement, ProviderSessionReconciliationBounds,
    ProviderSessionReconciliationDriver, ProviderSessionReconciliationOutcome,
    ProviderSessionReconciliationPlan, ProviderSessionReconciliationRequest, RequestId,
    RuntimeFailure, SessionResumeBinding, WorkingStateRestorationMethod,
    WorkingStateRestorationOperation, WorkingStateRestorationOutcome,
    validate_provider_session_reconciliation_execution,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KimiLocalServerReconciliationInput {
    request_id: RequestId,
    model: crate::KimiModelSelection,
    binding: SessionResumeBinding,
    checkpoint: PersistedProviderOperationCheckpoint,
    bounds: ProviderSessionReconciliationBounds,
    deadline: Option<swallowtail_runtime::Deadline>,
}

impl KimiLocalServerReconciliationInput {
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: crate::KimiModelSelection,
        binding: SessionResumeBinding,
        checkpoint: PersistedProviderOperationCheckpoint,
        bounds: ProviderSessionReconciliationBounds,
    ) -> Self {
        Self {
            request_id,
            model,
            binding,
            checkpoint,
            bounds,
            deadline: None,
        }
    }

    #[must_use]
    pub const fn with_deadline(mut self, deadline: swallowtail_runtime::Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }
}

#[derive(Clone, Debug)]
pub struct KimiLocalServerPreparedReconciliation {
    evidence: PreparedProviderSessionReconciliationEvidence,
    request: ProviderSessionReconciliationRequest,
}

impl KimiLocalServerPreparedReconciliation {
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionReconciliationEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionReconciliationPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &ProviderSessionReconciliationRequest {
        &self.request
    }

    pub fn execute(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionReconciliationOutcome, RuntimeFailure>> {
        let driver = crate::KimiLocalServerDriver::new();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move {
            driver
                .reconcile_provider_session(plan, request, services)
                .await
        })
    }
}

impl WorkingStateRestorationOperation for KimiLocalServerPreparedReconciliation {
    fn method(&self) -> WorkingStateRestorationMethod {
        WorkingStateRestorationMethod::ProviderSessionReconciliation
    }

    fn restore(
        self: Box<Self>,
        services: HostServices,
    ) -> BoxFuture<'static, Result<WorkingStateRestorationOutcome, RuntimeFailure>> {
        let future = self.execute(services);
        Box::pin(async move {
            future
                .await
                .map(WorkingStateRestorationOutcome::SessionReconciled)
        })
    }
}

impl crate::KimiLocalServerPreparedIntegration {
    pub fn prepare_working_state_restoration(
        &self,
        input: KimiLocalServerReconciliationInput,
    ) -> Result<PreparedWorkingStateRestoration, PreparationFailure> {
        self.prepare_session_reconciliation(input)
            .map(PreparedWorkingStateRestoration::new)
    }
}

impl crate::KimiLocalServerPreparedIntegration {
    pub fn prepare_session_reconciliation(
        &self,
        input: KimiLocalServerReconciliationInput,
    ) -> Result<KimiLocalServerPreparedReconciliation, PreparationFailure> {
        if self.instance().ownership() != InstanceOwnership::ExternalAttached
            || !self.server().is_qualified()
        {
            return Err(preparation_failure(
                "swallowtail.kimi.local_server.preparation.reconciliation_topology_unsupported",
                "Kimi local-server reconciliation requires a qualified attached server",
            ));
        }
        let KimiLocalServerReconciliationInput {
            request_id,
            model,
            binding,
            checkpoint,
            bounds,
            deadline,
        } = input;
        let reconciliation = CapabilityRequirement::new(
            Capability::ProviderSessionReconciliation,
            [
                CapabilityConstraint::ReplayMaximumItems(bounds.maximum_replay_items().get()),
                CapabilityConstraint::ReplayMaximumBytes(bounds.maximum_replay_bytes().get()),
            ],
        );
        let retention = CapabilityRequirement::new(Capability::ProviderDurableRetention, []);
        let resource = CapabilityRequirement::new(
            Capability::WorkingResource,
            [
                CapabilityConstraint::ResourceAccess(ResourceAccess::Read),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        );
        let capabilities =
            CapabilityProfile::new([reconciliation.clone(), retention.clone(), resource.clone()]);
        let instance = super::prepared::instance_with_capabilities(self, capabilities.clone());
        let (route_id, route_revision, model_id) = model.into_parts();
        if &route_id != binding.model_route_id() || &model_id != binding.model_id() {
            return Err(preparation_failure(
                "swallowtail.kimi.local_server.preparation.reconciliation_binding_mismatch",
                "Kimi local-server reconciliation model does not match its durable binding",
            ));
        }
        let route = ModelRoute::new(
            route_id,
            route_revision,
            instance.id().clone(),
            model_id,
            capabilities,
        );
        let access_policy = SessionAccessPolicy::ambient_harness(ResourceAccess::Read);
        let requirements = OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            OperationShape::ProviderSessionReconciliation,
            DriverRole::ProviderSessionReconciliation,
            self.instance().execution_host_id().clone(),
            AccessRequirement::new(self.access_profile().id().clone())
                .with_credential_states([CredentialState::Ready])
                .with_entitlement_states([EntitlementState::Available])
                .with_endpoint_authorizations([EndpointAuthorization::Allowed])
                .with_runtime_readiness([RuntimeReadiness::Ready])
                .with_support_authorities([self.access_profile().support_authority()]),
        )
        .with_ownership_modes([InstanceOwnership::ExternalAttached])
        .with_host_services(
            crate::kimi_local_server_descriptor()
                .required_host_services(DriverRole::ProviderSessionReconciliation),
        )
        .with_capabilities([reconciliation, retention, resource])
        .with_interface_versions([self.server().binding().clone()])
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
        .with_session_access_policy(access_policy)
        .with_session_provider_state_policy(
            SessionProviderStatePolicy::DurableProviderSessionPreserved,
        )
        .require_model_route();
        let descriptor = crate::kimi_local_server_descriptor();
        let preflight = preflight(
            &PreflightContext::new(
                &descriptor,
                &instance,
                self.access_profile(),
                self.access_evidence().status(),
                self.available_host_services(),
            )
            .with_model_route(&route),
            &requirements,
        )
        .map_err(|error| {
            PreparationFailure::new(
                PreparationStage::Preflight,
                Diagnostic::new(error.diagnostic().clone()),
            )
        })?;
        let checkpoint =
            ProviderOperationCheckpoint::restore_persisted(&checkpoint, &preflight, &binding)
                .map_err(|error| {
                    PreparationFailure::new(
                        PreparationStage::Preflight,
                        Diagnostic::new(error.diagnostic().clone()),
                    )
                })?;
        let agreement = ProviderSessionReconciliationAgreement::new(
            binding,
            checkpoint.runtime_turn_id().clone(),
            Some(checkpoint.provider_turn_ref().clone()),
            bounds,
            deadline,
        )
        .with_checkpoint(checkpoint)
        .map_err(|error| {
            PreparationFailure::new(
                PreparationStage::Preflight,
                Diagnostic::new(error.diagnostic().clone()),
            )
        })?;
        let plan =
            ProviderSessionReconciliationPlan::new(preflight, agreement).map_err(|error| {
                PreparationFailure::new(
                    PreparationStage::Preflight,
                    Diagnostic::new(error.diagnostic().clone()),
                )
            })?;
        let request = ProviderSessionReconciliationRequest::from_plan(request_id, &plan).map_err(
            |error| {
                PreparationFailure::new(
                    PreparationStage::Preflight,
                    Diagnostic::new(error.diagnostic().clone()),
                )
            },
        )?;
        Ok(KimiLocalServerPreparedReconciliation {
            evidence: PreparedProviderSessionReconciliationEvidence::from_plan(
                plan,
                self.access_evidence().clone(),
            )?,
            request,
        })
    }
}

impl ProviderSessionReconciliationDriver for crate::KimiLocalServerDriver {
    fn reconcile_provider_session(
        &self,
        plan: ProviderSessionReconciliationPlan,
        request: ProviderSessionReconciliationRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionReconciliationOutcome, RuntimeFailure>> {
        Box::pin(async move {
            validate_provider_session_reconciliation_execution(&plan, &request, &services)?;
            execute(self, plan, request, services).await
        })
    }
}

async fn execute(
    driver: &crate::KimiLocalServerDriver,
    plan: ProviderSessionReconciliationPlan,
    request: ProviderSessionReconciliationRequest,
    services: HostServices,
) -> Result<ProviderSessionReconciliationOutcome, RuntimeFailure> {
    let agreement = plan.agreement();
    let checkpoint = agreement.checkpoint().ok_or_else(checkpoint_required)?;
    let cursor = decode(checkpoint.cursor())?;
    control(&request, &services)?;
    let scope = swallowtail_runtime::ScopeId::new(format!(
        "kimi-local:reconciliation:{}",
        request.request_id().as_str()
    ))
    .map_err(|_| protocol_failure())?;
    let access_policy = SessionAccessPolicy::ambient_harness(ResourceAccess::Read);
    let mut access = SessionAccess::acquire(
        plan.preflight(),
        scope.clone(),
        &services,
        agreement.binding().working_resource(),
        &access_policy,
    )
    .await?;
    let result = observe(
        driver,
        &plan,
        &request,
        &services,
        &scope,
        access.endpoint.clone(),
        access.directory.clone(),
        Arc::clone(&access.secret),
        cursor,
    )
    .await;
    let cleanup = access.release(&services).await;
    let observation = match result {
        Ok(observation)
            if matches!(
                cleanup,
                CleanupOutcome::Clean | CleanupOutcome::NotApplicable
            ) =>
        {
            observation
        }
        Ok(_) => return Err(cleanup_failure(cleanup)),
        Err(error) => {
            if matches!(
                cleanup,
                CleanupOutcome::Clean | CleanupOutcome::NotApplicable
            ) {
                return Err(error);
            }
            return Err(cleanup_failure(cleanup));
        }
    };
    ProviderSessionReconciliationOutcome::new(&plan, &request, observation, cleanup)
}

#[allow(clippy::too_many_arguments)]
async fn observe(
    driver: &crate::KimiLocalServerDriver,
    plan: &ProviderSessionReconciliationPlan,
    request: &ProviderSessionReconciliationRequest,
    services: &HostServices,
    scope: &swallowtail_runtime::ScopeId,
    endpoint: String,
    directory: String,
    secret: Arc<SecretMaterial>,
    mut cursor: KimiCursorCheckpoint,
) -> Result<swallowtail_runtime::ProviderSessionReconciliationObservation, RuntimeFailure> {
    let agreement = plan.agreement();
    let provider_session = agreement
        .binding()
        .provider_session_ref()
        .as_provider_value();
    let expected_turn = agreement
        .provider_turn_ref()
        .ok_or_else(checkpoint_required)?
        .as_provider_value()
        .parse::<u64>()
        .map_err(|_| checkpoint_required())?;
    let before = fetch_session(
        driver,
        scope,
        &endpoint,
        &secret,
        provider_session,
        services,
    )
    .await?;
    validate_session(&before, provider_session, &directory)?;
    if before.last_seq < cursor.seq {
        return Err(stale_checkpoint());
    }
    control(request, services)?;
    let mut subscription = Subscription::open(
        SubscriptionInput {
            scope: scope.clone(),
            endpoint: endpoint.clone(),
            secret: secret.copy(),
            session_id: provider_session.to_owned(),
            cursor_seq: cursor.seq,
            cursor_epoch: Some(cursor.epoch.clone()),
            deadline: agreement.deadline(),
        },
        services,
    )
    .await?;
    let target = subscription.replay_target().0;
    if target < cursor.seq {
        let _ = subscription.close().await;
        return Err(stale_checkpoint());
    }
    let mut observed_state = None;
    while cursor.seq < target {
        let frame = next_frame(&mut subscription, request, agreement.deadline(), services).await?;
        if let Some(state) =
            apply_reconciliation_frame(&frame, provider_session, expected_turn, &mut cursor)?
        {
            observed_state = Some(state);
            if state.is_terminal() {
                break;
            }
        }
    }
    // The finite replay target is already reached. Closing still joins the
    // worker; a peer close racing our local close does not invalidate the
    // accepted observation snapshot.
    let _ = subscription.close().await;
    control(request, services)?;
    let after = fetch_session(
        driver,
        scope,
        &endpoint,
        &secret,
        provider_session,
        services,
    )
    .await?;
    validate_session(&after, provider_session, &directory)?;
    let state = observed_state.unwrap_or({
        if after.last_seq != target {
            swallowtail_runtime::InterruptedTurnState::Unknown
        } else if after.busy {
            swallowtail_runtime::InterruptedTurnState::Active
        } else {
            swallowtail_runtime::InterruptedTurnState::InactiveUnresolved
        }
    });
    Ok(
        swallowtail_runtime::ProviderSessionReconciliationObservation::exact_turn(
            state,
            agreement
                .provider_turn_ref()
                .expect("exact checkpoint has turn reference")
                .clone(),
            Vec::new(),
            true,
        ),
    )
}

async fn fetch_session(
    driver: &crate::KimiLocalServerDriver,
    scope: &swallowtail_runtime::ScopeId,
    endpoint: &str,
    secret: &Arc<SecretMaterial>,
    provider_session: &str,
    services: &HostServices,
) -> Result<InteractiveSessionRecord, RuntimeFailure> {
    let response = driver
        .transport
        .request(
            scope.clone(),
            endpoint.to_owned(),
            Request::get(session_path(provider_session)?),
            Some(secret.copy()),
            services,
            Arc::new(AtomicBool::new(false)),
        )
        .await?;
    if response.status != 200 {
        return match decode_rest(response.status, &response.body) {
            Ok(RestReply::Failure(_)) => Err(binding_failure()),
            _ => Err(protocol_failure()),
        };
    }
    decode_interactive_session(&response.body)
}

async fn next_frame(
    subscription: &mut Subscription,
    request: &ProviderSessionReconciliationRequest,
    deadline: Option<swallowtail_runtime::Deadline>,
    services: &HostServices,
) -> Result<Vec<u8>, RuntimeFailure> {
    let mut timer =
        deadline.and_then(|deadline| services.time().map(|time| time.wait_until(deadline)));
    std::future::poll_fn(|context| {
        if request.cancellation().is_requested() {
            return std::task::Poll::Ready(Err(cancelled()));
        }
        if let std::task::Poll::Ready(frame) = subscription.poll_next(context) {
            return std::task::Poll::Ready(match frame {
                Some(frame) => frame,
                None => Err(protocol_failure()),
            });
        }
        if timer
            .as_mut()
            .is_some_and(|timer| timer.as_mut().poll(context).is_ready())
        {
            return std::task::Poll::Ready(Err(timed_out()));
        }
        std::task::Poll::Pending
    })
    .await
}

fn apply_reconciliation_frame(
    frame: &[u8],
    provider_session: &str,
    expected_turn: u64,
    cursor: &mut KimiCursorCheckpoint,
) -> Result<Option<swallowtail_runtime::InterruptedTurnState>, RuntimeFailure> {
    let WsFrame::Event(envelope) = decode_ws_frame(frame)? else {
        return Err(protocol_failure());
    };
    if envelope.session_id != provider_session {
        return Err(binding_failure());
    }
    if envelope.volatile {
        return Ok(None);
    }
    if envelope.epoch.as_deref() != Some(cursor.epoch.as_str())
        || envelope.durable_seq != cursor.seq.saturating_add(1)
    {
        return Err(stale_checkpoint());
    }
    cursor.seq = envelope.durable_seq;
    let observed_turn = event_turn_id(&envelope.event);
    if observed_turn.is_some_and(|turn| turn != expected_turn) {
        return Err(binding_failure());
    }
    Ok(match envelope.event {
        WsEvent::TurnEnded { reason, .. } => Some(match reason {
            TurnEndReason::Completed => swallowtail_runtime::InterruptedTurnState::Completed,
            TurnEndReason::Cancelled => swallowtail_runtime::InterruptedTurnState::Cancelled,
            TurnEndReason::Failed | TurnEndReason::Blocked => {
                swallowtail_runtime::InterruptedTurnState::Failed
            }
        }),
        WsEvent::AwaitingApproval | WsEvent::AwaitingQuestion => {
            Some(swallowtail_runtime::InterruptedTurnState::WaitingForProviderInput)
        }
        WsEvent::SessionAborted => Some(swallowtail_runtime::InterruptedTurnState::Cancelled),
        WsEvent::TurnStarted { .. }
        | WsEvent::AssistantDelta { .. }
        | WsEvent::ThinkingDelta { .. }
        | WsEvent::StepStarted { .. }
        | WsEvent::StepEnded { .. }
        | WsEvent::ToolStarted { .. }
        | WsEvent::ToolUpdated { .. }
        | WsEvent::ToolEnded { .. }
        | WsEvent::Retrying { .. } => Some(swallowtail_runtime::InterruptedTurnState::Active),
        _ => None,
    })
}

fn event_turn_id(event: &WsEvent) -> Option<u64> {
    match event {
        WsEvent::TurnStarted { turn_id }
        | WsEvent::AssistantDelta { turn_id, .. }
        | WsEvent::ThinkingDelta { turn_id, .. }
        | WsEvent::StepStarted { turn_id, .. }
        | WsEvent::StepEnded { turn_id, .. }
        | WsEvent::ToolStarted { turn_id, .. }
        | WsEvent::ToolUpdated { turn_id, .. }
        | WsEvent::ToolEnded { turn_id, .. }
        | WsEvent::TurnEnded { turn_id, .. }
        | WsEvent::Retrying { turn_id, .. } => Some(*turn_id),
        _ => None,
    }
}

fn validate_session(
    record: &InteractiveSessionRecord,
    provider_session: &str,
    directory: &str,
) -> Result<(), RuntimeFailure> {
    if record.id != provider_session || record.archived || record.working_directory != directory {
        Err(binding_failure())
    } else {
        Ok(())
    }
}

fn control(
    request: &ProviderSessionReconciliationRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    if request.cancellation().is_requested() {
        return Err(cancelled());
    }
    if request.agreement().deadline().is_some_and(|deadline| {
        services
            .time()
            .is_some_and(|time| time.now() >= deadline.instant())
    }) {
        return Err(timed_out());
    }
    Ok(())
}

fn preparation_failure(code: &'static str, message: &'static str) -> PreparationFailure {
    PreparationFailure::new(
        PreparationStage::Preflight,
        Diagnostic::new(swallowtail_core::SafeDiagnostic::new(code, message)),
    )
}

fn checkpoint_required() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.reconciliation_checkpoint_required",
        "Kimi local-server reconciliation requires an exact operation checkpoint",
    )
}

fn stale_checkpoint() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.reconciliation_checkpoint_stale",
        "Kimi local-server reconciliation checkpoint is stale or discontinuous",
    )
}

fn binding_failure() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.reconciliation_binding_mismatch",
        "Kimi local-server reconciliation observed a different provider operation",
    )
}

fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.reconciliation_protocol_failed",
        "Kimi local-server reconciliation protocol failed",
    )
}

fn cancelled() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.reconciliation_cancelled",
        "Kimi local-server reconciliation was cancelled",
    )
}

fn timed_out() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.reconciliation_timed_out",
        "Kimi local-server reconciliation timed out",
    )
}

fn cleanup_failure(cleanup: CleanupOutcome) -> RuntimeFailure {
    match cleanup {
        CleanupOutcome::Degraded(diagnostic) | CleanupOutcome::Failed(diagnostic) => {
            RuntimeFailure::new(diagnostic)
        }
        CleanupOutcome::Clean | CleanupOutcome::NotApplicable => protocol_failure(),
    }
}

#[cfg(test)]
mod tests {
    use super::{KimiCursorCheckpoint, apply_reconciliation_frame};
    use swallowtail_runtime::InterruptedTurnState;

    #[test]
    fn retained_terminal_event_preserves_exact_turn_truth() {
        let mut cursor = checkpoint();
        let state = apply_reconciliation_frame(
            &event(12, "fixture-session", "fixture-epoch", 7),
            "fixture-session",
            7,
            &mut cursor,
        )
        .expect("exact event projects");

        assert_eq!(state, Some(InterruptedTurnState::Completed));
        assert_eq!(cursor.seq, 12);
    }

    #[test]
    fn foreign_turn_session_epoch_and_cursor_gap_fail_closed() {
        for (frame, code) in [
            (
                event(12, "fixture-session", "fixture-epoch", 8),
                "swallowtail.kimi.local_server.reconciliation_binding_mismatch",
            ),
            (
                event(12, "foreign-session", "fixture-epoch", 7),
                "swallowtail.kimi.local_server.reconciliation_binding_mismatch",
            ),
            (
                event(12, "fixture-session", "foreign-epoch", 7),
                "swallowtail.kimi.local_server.reconciliation_checkpoint_stale",
            ),
            (
                event(13, "fixture-session", "fixture-epoch", 7),
                "swallowtail.kimi.local_server.reconciliation_checkpoint_stale",
            ),
        ] {
            let error = apply_reconciliation_frame(&frame, "fixture-session", 7, &mut checkpoint())
                .expect_err("mismatched retained event rejects");
            assert_eq!(error.diagnostic().code(), code);
        }
    }

    fn checkpoint() -> KimiCursorCheckpoint {
        KimiCursorCheckpoint {
            seq: 11,
            epoch: "fixture-epoch".to_owned(),
        }
    }

    fn event(seq: u64, session: &str, epoch: &str, turn: u64) -> Vec<u8> {
        serde_json::to_vec(&serde_json::json!({
            "type": "turn.ended",
            "seq": seq,
            "timestamp": "fixture",
            "session_id": session,
            "epoch": epoch,
            "payload": {"turnId": turn, "reason": "completed"}
        }))
        .expect("fixture event encodes")
    }
}
