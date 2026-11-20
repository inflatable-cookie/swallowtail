use crate::{
    command::arguments,
    failure::{failure, malformed, protocol_failure, unsupported},
};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use swallowtail_acp_client::{
    AcpActivityConfig, AcpDiagnostics, AcpSessionConfig, AcpSessionHandle, AcpSessionOpenContext,
    AcpTransport, ActiveTurnConfig, MessageClassification, PlanProjection, StopReasonPolicy,
    default_permission_namespace, open_acp_session,
};
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, Capability, CapabilityConstraint,
    CredentialMechanism, DriverDescriptor, DriverRole, ExecutionLayer, HarnessConfigurationPosture,
    HarnessIsolation, HarnessMode, HostServiceKind, IntegrationFamilyId, OperationShape,
    PreflightPlan, ResourceAccess, ResourceRepresentation, SafeDiagnostic, SessionAccessPolicy,
    SessionRef, TransportFamilyId,
};
use swallowtail_protocol_acp::{DEFAULT_MAX_BUFFER_BYTES, DEFAULT_MAX_FRAME_BYTES, FramingLimits};
use swallowtail_runtime::{
    BoxFuture, EnvironmentRef, ExecutableRef, HostServices, InteractiveSessionDriver,
    InteractiveSessionHandle, OpenSessionRequest, ProcessHandle, ProcessRequest, ResourceLease,
    ResumeSessionRequest, RuntimeFailure, ScopeId, validate_session_resource_lease,
};

const DRIVER_ID: &str = "swallowtail.cline.acp";
const RECEIVE_FRAMING_LIMITS: FramingLimits =
    FramingLimits::new(DEFAULT_MAX_FRAME_BYTES, DEFAULT_MAX_BUFFER_BYTES);

fn cline_diagnostics() -> AcpDiagnostics {
    AcpDiagnostics {
        route_name: "cline.acp",
        activity: AcpActivityConfig {
            id_prefix: "cline-acp",
            namespace_prefix: "cline.acp",
            plan_projection: PlanProjection::Ignore,
            message_classification: MessageClassification::ModeAndWarning,
            malformed,
        },
        malformed,
        protocol_failure,
        version_mismatch: || {
            failure(
                "swallowtail.cline.acp.version_mismatch",
                "Cline negotiated an incompatible ACP version",
            )
        },
        connection_closed: || {
            failure(
                "swallowtail.cline.acp.connection_closed",
                "Cline ACP connection is closed",
            )
        },
        correlation_limit: || {
            failure(
                "swallowtail.cline.acp.correlation_limit",
                "Cline ACP request correlation limit was exceeded",
            )
        },
        session_duplicate: || {
            failure(
                "swallowtail.cline.acp.session_duplicate",
                "Cline returned more than one ACP session",
            )
        },
        turn_active: || {
            failure(
                "swallowtail.cline.acp.turn_active",
                "Cline session already has an active turn",
            )
        },
        unknown_response: || {
            failure(
                "swallowtail.cline.acp.unknown_response",
                "Cline returned an unknown ACP response",
            )
        },
        update_without_turn: || {
            failure(
                "swallowtail.cline.acp.update_without_turn",
                "Cline updated a session without an active turn",
            )
        },
        agent_cancel_unsupported: || {
            failure(
                "swallowtail.cline.acp.agent_cancel_unsupported",
                "Cline sent an unsupported client cancellation notification",
            )
        },
        notification_unsupported: || {
            failure(
                "swallowtail.cline.acp.notification_unsupported",
                "Cline sent an unsupported ACP notification",
            )
        },
        callback_unsupported: || {
            failure(
                "swallowtail.cline.acp.callback_unsupported",
                "Cline requested an unsupported ACP client callback",
            )
        },
        permission_without_turn: || {
            failure(
                "swallowtail.cline.acp.permission_without_turn",
                "Cline requested permission without an active turn",
            )
        },
        session_mismatch_callback: || {
            failure(
                "swallowtail.cline.acp.session_mismatch",
                "Cline callback does not match the active session",
            )
        },
        session_mismatch_update: || {
            failure(
                "swallowtail.cline.acp.session_mismatch",
                "Cline update does not match the active session",
            )
        },
        process_cleanup_failed: || {
            SafeDiagnostic::new(
                "swallowtail.cline.acp.process_cleanup_failed",
                "Cline process cleanup failed",
            )
        },
        cleanup_missing: || {
            SafeDiagnostic::new(
                "swallowtail.cline.acp.cleanup_missing",
                "Cline process cleanup did not complete",
            )
        },
        connection_ended: || {
            failure(
                "swallowtail.cline.acp.connection_ended",
                "Cline ACP connection ended",
            )
        },
        output_limit_exceeded: || {
            failure(
                "swallowtail.cline.acp.output_limit_exceeded",
                "Cline output exceeded the adapter limit",
            )
        },
        prompt_stopped: || {
            SafeDiagnostic::new(
                "swallowtail.cline.acp.prompt_stopped",
                "Cline stopped before completing the turn",
            )
        },
        stop_reason_unsupported: || {
            failure(
                "swallowtail.cline.acp.stop_reason_unsupported",
                "Cline returned an unsupported ACP stop reason",
            )
        },
        task_service_missing: || {
            failure(
                "swallowtail.cline.acp.task_service_missing",
                "Cline ACP requires a scoped task service",
            )
        },
        task_join_failed: || {
            SafeDiagnostic::new(
                "swallowtail.cline.acp.task_join_failed",
                "Cline ACP protocol task did not join cleanly",
            )
        },
        turn_join_failed: || {
            SafeDiagnostic::new(
                "swallowtail.cline.acp.turn_join_failed",
                "Cline ACP prompt task did not join cleanly",
            )
        },
        resource_release_failed: || {
            SafeDiagnostic::new(
                "swallowtail.cline.acp.resource_release_failed",
                "Cline ACP working-resource service disappeared during cleanup",
            )
        },
        classify_rpc_error: |error| {
            if error.message() == "auth_required" {
                failure(
                    "swallowtail.cline.acp.auth_required",
                    "Cline requires host-owned credentials before ACP work",
                )
            } else {
                failure(
                    "swallowtail.cline.acp.request_rejected",
                    "Cline rejected an ACP request",
                )
            }
        },
        permission_namespace: default_permission_namespace,
        unsupported,
    }
}

/// Low-level interactive driver for the installed Cline ACP agent.
pub struct ClineAcpDriver {
    isolated_environment: EnvironmentRef,
}

impl ClineAcpDriver {
    /// Binds the isolated launch environment. Credentials stay host-owned.
    #[must_use]
    pub const fn new(isolated_environment: EnvironmentRef) -> Self {
        Self {
            isolated_environment,
        }
    }

    fn validate_plan(
        &self,
        plan: &PreflightPlan,
    ) -> Result<crate::selection::ClinePlanSelection, RuntimeFailure> {
        if plan.driver_identity().id().as_str() != DRIVER_ID {
            return Err(failure(
                "swallowtail.cline.acp.plan_driver_mismatch",
                "Preflight plan is bound to a different driver",
            ));
        }
        if plan.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
            || plan.credential_reference().is_some()
            || plan.endpoint_audience().as_str() != crate::CLINE_LOCAL_ACCOUNT_AUDIENCE
        {
            return Err(failure(
                "swallowtail.cline.acp.access_profile_rejected",
                "Cline ACP requires its host-owned local-account profile",
            ));
        }
        if plan.harness_configuration_posture() != Some(HarnessConfigurationPosture::Ambient) {
            return Err(failure(
                "swallowtail.cline.acp.configuration_posture_rejected",
                "Cline ACP requires explicit ambient configuration inside its selected environment",
            ));
        }
        if plan.requirements().harness_isolation() != Some(HarnessIsolation::AmbientHost) {
            return Err(failure(
                "swallowtail.cline.acp.isolation_rejected",
                "Cline ACP requires explicit ambient-host isolation posture",
            ));
        }
        crate::selection::select_cline_acp_plan(plan)
    }
}

/// Describes the installed Cline ACP discovery and session roles.
#[must_use]
pub fn cline_acp_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("cline").expect("static family id is valid"),
        TransportFamilyId::new("acp-v1-stdio").expect("static transport id is valid"),
    )
    .with_roles([DriverRole::Discovery, DriverRole::InteractiveSession])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([OperationShape::InteractiveSession])
    .with_required_host_services(
        DriverRole::InteractiveSession,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::WorkingResource,
        ],
    )
    .with_required_host_services(
        DriverRole::Discovery,
        [
            HostServiceKind::Task,
            HostServiceKind::Time,
            HostServiceKind::Process,
        ],
    )
    .with_discovery_actions([swallowtail_core::DiscoveryAction::Probe])
    .with_interface_compatibility(crate::cline_acp_claim())
}

impl InteractiveSessionDriver for ClineAcpDriver {
    fn open_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            self.open_session_lifecycle(plan, request, services)
                .await
                .map(|(session, _)| session)
                .map_err(ClineOpenRejection::into_failure)
        })
    }

    fn resume_session(
        &self,
        _plan: PreflightPlan,
        _request: ResumeSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async { Err(unsupported("session resume")) })
    }
}

impl ClineAcpDriver {
    async fn start_session(
        &self,
        plan: &PreflightPlan,
        request: &OpenSessionRequest,
        services: &HostServices,
        scope: ScopeId,
        resource: ResourceLease,
        selected: crate::selection::ClinePlanSelection,
    ) -> Result<(AcpSessionHandle, ClineOpenObservation), Box<(ClineOpenRejection, ResourceLease)>>
    {
        let cwd = resource
            .filesystem()
            .expect("validated filesystem lease")
            .as_driver_value()
            .to_owned();
        let process_service = services
            .process()
            .cloned()
            .expect("validated process service");
        let process_request = ProcessRequest::new(ExecutableRef::from_instance_target(
            plan.instance_target_ref(),
        ))
        .with_arguments(arguments())
        .with_environment([self.isolated_environment.clone()])
        .with_working_resource(
            request
                .working_resource()
                .expect("validated resource")
                .clone(),
        );
        let process: Arc<dyn ProcessHandle> =
            match process_service.start(scope.clone(), process_request).await {
                Ok(process) => Arc::from(process),
                Err(error) => return Err(Box::new((error.into(), resource))),
            };
        let transport = AcpTransport::stdio(process, RECEIVE_FRAMING_LIMITS);
        let want_plan_mode = requested_plan_mode(request.options());
        let observation_slot: Arc<Mutex<Option<ClineOpenObservation>>> = Arc::new(Mutex::new(None));
        let rejection_slot: Arc<Mutex<Option<ClineOpenRejection>>> = Arc::new(Mutex::new(None));
        let obs_writer = Arc::clone(&observation_slot);
        let rej_writer = Arc::clone(&rejection_slot);

        let opened = open_acp_session(
            transport,
            AcpSessionOpenContext {
                scope,
                request_id: request.request_id().clone(),
                execution_host_id: plan.execution_host_id().clone(),
                resource,
                services: services.clone(),
                config: cline_session_config(),
            },
            move |connection| async move {
                let initialize = connection.initialize().await?;
                validate_initialize(&initialize, selected.version())?;
                let response = connection
                    .request("session/new", json!({"cwd": cwd, "mcpServers": []}))
                    .await?;
                let provider_id = parse_new_session(&response)?;
                let model = observe_model_options(&response);
                let _ = SessionRef::new(&provider_id).map_err(|_| malformed())?;
                let plan_acknowledgement = if want_plan_mode {
                    prepare_plan_mode(&response)?;
                    let confirmation = connection
                        .request(
                            "session/set_config_option",
                            json!({
                                "sessionId": provider_id,
                                "configId": "mode",
                                "value": "plan",
                            }),
                        )
                        .await?;
                    match confirm_plan_mode(&confirmation) {
                        Ok(ack) => ack,
                        Err(rejection) => {
                            let failure = rejection.failure.clone();
                            *rej_writer.lock().expect("rejection lock poisoned") = Some(rejection);
                            return Err(failure);
                        }
                    }
                } else {
                    ClinePlanAcknowledgement::NotRequested
                };
                *obs_writer.lock().expect("observation lock poisoned") =
                    Some(ClineOpenObservation {
                        plan_acknowledgement,
                        model,
                    });
                Ok(provider_id)
            },
        )
        .await;

        match opened {
            Ok(session) => {
                let observation = observation_slot
                    .lock()
                    .expect("observation lock poisoned")
                    .take()
                    .expect("handshake populated observation");
                let negotiated = observation.model.exact().cloned();
                Ok((
                    session.with_negotiated_model_options(negotiated),
                    observation,
                ))
            }
            Err(pair) => {
                let (error, resource) = *pair;
                let rejection = rejection_slot
                    .lock()
                    .expect("rejection lock poisoned")
                    .take()
                    .unwrap_or_else(|| error.into());
                Err(Box::new((rejection, resource)))
            }
        }
    }
}

fn cline_session_config() -> AcpSessionConfig {
    let diagnostics = cline_diagnostics();
    AcpSessionConfig::new("cline-acp", diagnostics).with_turn_config(
        ActiveTurnConfig::new(diagnostics)
            .with_content_unsupported(|| {
                failure(
                    "swallowtail.cline.acp.content_unsupported",
                    "Cline returned unsupported ACP content",
                )
            })
            .with_stop_reason_policy(StopReasonPolicy::ExtendedProviderStops)
            .with_auth_required_stop(|| {
                SafeDiagnostic::new(
                    "swallowtail.cline.acp.auth_required",
                    "Cline requires host-owned credentials before ACP work",
                )
            }),
    )
}

include!("driver/open.rs");
include!("driver/model.rs");
include!("driver/mode.rs");
include!("driver/validation.rs");

#[cfg(test)]
mod driver_tests {
    use super::*;
    use std::future::Future;
    use std::pin::pin;
    use std::task::{Context, Poll, Waker};
    use swallowtail_acp_client::{AcpTurnSink, ActiveTurn};
    use swallowtail_runtime::{RuntimeTurnId, TerminalStatus};

    #[test]
    fn non_text_agent_content_reports_content_unsupported() {
        let (turn, _events, _terminal) = ActiveTurn::new(
            RuntimeTurnId::new("cline-non-text").expect("valid turn id"),
            "sess-1".to_owned(),
            cline_session_config().turn_config,
        )
        .expect("turn creates");
        let err = turn
            .handle_update(&json!({
                "sessionId": "sess-1",
                "update": {
                    "sessionUpdate": "agent_message_chunk",
                    "content": {"type": "image", "mimeType": "image/png", "data": "AAAA"}
                }
            }))
            .expect_err("non-text content must fail");
        assert_eq!(
            err.diagnostic().code(),
            "swallowtail.cline.acp.content_unsupported"
        );
    }

    #[test]
    fn stop_reasons_map_auth_required_max_turn_requests_and_refusal_to_provider_failed() {
        for (stop_reason, expected_code) in [
            ("auth_required", "swallowtail.cline.acp.auth_required"),
            ("max_turn_requests", "swallowtail.cline.acp.prompt_stopped"),
            ("refusal", "swallowtail.cline.acp.prompt_stopped"),
        ] {
            let (turn, _events, terminal) = ActiveTurn::new(
                RuntimeTurnId::new("cline-stop-reason").expect("valid turn id"),
                "sess-1".to_owned(),
                cline_session_config().turn_config,
            )
            .expect("turn creates");
            turn.finish_prompt(stop_reason);
            let waker = Waker::noop();
            let mut cx = Context::from_waker(waker);
            let Poll::Ready(outcome) = pin!(terminal).poll(&mut cx) else {
                panic!("terminal outcome must be ready immediately after finish_prompt");
            };
            match outcome.status() {
                TerminalStatus::ProviderFailed(diag) => {
                    assert_eq!(diag.code(), expected_code, "stop_reason={stop_reason}");
                }
                other => panic!("expected ProviderFailed for {stop_reason}, got {other:?}"),
            }
        }
    }
}
