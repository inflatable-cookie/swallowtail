#[cfg(test)]
mod tests {
    use super::{DRIVER_ID, opencode_http_descriptor};
    use crate::selection::{
        OPENCODE_BASELINE_VERSION, OPENCODE_LATEST_QUALIFIED_VERSION, opencode_server_binding,
    };
    use swallowtail_core::{
        DriverRole, ExecutionLayer, HostServiceKind, InterfaceCompatibilityAssessment,
        OperationShape,
    };

    #[test]
    fn descriptor_claims_only_attached_harness_roles() {
        let descriptor = opencode_http_descriptor();
        assert_eq!(descriptor.identity().id().as_str(), DRIVER_ID);
        assert!(descriptor.supports_role(DriverRole::ModelCatalog));
        assert!(descriptor.supports_role(DriverRole::InteractiveSession));
        assert!(descriptor.supports_role(DriverRole::ProviderSessionManagement));
        assert!(descriptor.supports_role(DriverRole::ProviderSessionReconciliation));
        assert!(descriptor.supports_role(DriverRole::ProviderSessionHistory));
        assert!(descriptor.supports_role(DriverRole::StructuredRun));
        assert!(descriptor.supports_execution_layer(ExecutionLayer::HarnessInteraction));
        assert!(!descriptor.supports_execution_layer(ExecutionLayer::DirectModelInference));
        assert!(descriptor.supports_operation_shape(OperationShape::InteractiveSession));
        assert!(descriptor.supports_operation_shape(OperationShape::StructuredRun));
        assert!(
            descriptor.supports_operation_shape(OperationShape::ProviderSessionManagement)
        );
        assert!(
            descriptor.supports_operation_shape(OperationShape::ProviderSessionReconciliation)
        );
        assert!(
            descriptor.supports_operation_shape(OperationShape::ProviderSessionHistory)
        );
        assert!(
            descriptor
                .required_host_services(DriverRole::InteractiveSession)
                .any(|service| service == HostServiceKind::BlockingWork)
        );
        assert!(
            descriptor
                .required_host_services(DriverRole::StructuredRun)
                .any(|service| service == HostServiceKind::WorkingResource)
        );
        for version in [OPENCODE_BASELINE_VERSION, OPENCODE_LATEST_QUALIFIED_VERSION] {
            assert!(
                descriptor.supports_interface_version(
                    &opencode_server_binding(version).expect("version is safe")
                )
            );
        }
        assert!(!descriptor.supports_interface_version(
            &opencode_server_binding("1.15.8").expect("gap version is safe")
        ));
        let gap = opencode_server_binding("1.18.36").expect("gap version is safe");
        assert!(!descriptor.supports_interface_version(&gap));
        assert!(!descriptor.permits_interface_version(&gap));
        let newer = opencode_server_binding("1.19.5").expect("newer version is safe");
        assert!(!descriptor.supports_interface_version(&newer));
        assert!(descriptor.permits_interface_version(&newer));
        assert!(matches!(
            descriptor.assess_interface_version(&newer),
            InterfaceCompatibilityAssessment::UnverifiedNewer(_)
        ));
    }
}

#[test]
fn session_history_control_failures_use_the_history_namespace() {
    let cancelled = HISTORY_CONTROL_CODES.failure(ControlFailureKind::Cancelled);
    let timed_out = HISTORY_CONTROL_CODES.failure(ControlFailureKind::TimedOut);
    assert_eq!(
        cancelled.diagnostic().code(),
        "swallowtail.opencode.session_history.cancelled"
    );
    assert_eq!(
        timed_out.diagnostic().code(),
        "swallowtail.opencode.session_history.timed_out"
    );

    let reconciliation = RECONCILIATION_CONTROL_CODES.failure(ControlFailureKind::Cancelled);
    assert_eq!(
        reconciliation.diagnostic().code(),
        "swallowtail.opencode.session_reconciliation.cancelled"
    );
}

#[test]
fn a_refused_cancellation_at_close_is_reported_not_discarded() {
    let refused = cancellation_cleanup(Err(RuntimeFailure::new(
        swallowtail_core::SafeDiagnostic::new("fixture.abort_refused", "abort refused"),
    )));
    assert_eq!(
        refused.diagnostic().map(swallowtail_core::SafeDiagnostic::code),
        Some("fixture.abort_refused")
    );
    assert_eq!(
        cancellation_cleanup(Ok(swallowtail_runtime::CancellationAcknowledgement::Requested)),
        CleanupOutcome::NotApplicable
    );
    assert_eq!(
        CleanupOutcome::merge(refused, CleanupOutcome::Clean)
            .diagnostic()
            .map(swallowtail_core::SafeDiagnostic::code),
        Some("fixture.abort_refused"),
        "a refused cancellation is not hidden by a clean join"
    );
}

#[test]
fn stalled_abort_during_turn_run_and_active_close_is_bounded() {
    use futures_util::FutureExt;
    use swallowtail_core::ExecutionHostId;
    use swallowtail_runtime::{
        BlockingJob, BlockingWorkService, BoxFuture, Deadline, DeadlineObservation,
        MonotonicInstant, TimeService,
    };

    struct PastClock;

    impl TimeService for PastClock {
        fn now(&self) -> MonotonicInstant {
            MonotonicInstant::from_ticks(10)
        }

        fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
            Box::pin(async move {
                DeadlineObservation::new(deadline, MonotonicInstant::from_ticks(u64::MAX))
            })
        }
    }

    struct StalledBlocking;

    impl BlockingWorkService for StalledBlocking {
        fn run(
            &self,
            _scope: ScopeId,
            _job: BlockingJob,
        ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            Box::pin(std::future::pending())
        }
    }

    fn make_cancellation(services: HostServices) -> Arc<TurnCancellation> {
        Arc::new(TurnCancellation {
            scope: ScopeId::new("opencode.test.abort-close").expect("scope"),
            session_id: "session-1".to_owned(),
            directory: "/workspace".to_owned(),
            endpoint: "http://127.0.0.1:4096".to_owned(),
            services,
            transport: CurlTransport,
            stream_cancelled: Arc::new(AtomicBool::new(false)),
            requested: AtomicBool::new(false),
            callbacks: None,
        })
    }

    let services = HostServices::new(ExecutionHostId::new("opencode.abort.bound").expect("host"))
        .with_time(Arc::new(PastClock))
        .with_blocking_work(Arc::new(StalledBlocking));

    // 1. OpenCodeTurnHandle::close bounds a stalled /abort.
    let turn_handle = Box::new(OpenCodeTurnHandle {
        runtime_id: RuntimeTurnId::new("turn-stalled-abort").expect("turn id"),
        events: None,
        terminal: None,
        cancellation: make_cancellation(services.clone()),
        detachment: None,
        terminal_flag: Arc::new(AtomicBool::new(false)),
        active: Arc::new(Mutex::new(None)),
        callbacks: None,
        attachment: input::SharedAttachment::default(),
    });
    let turn_cleanup = futures_executor::block_on(turn_handle.close());
    assert_eq!(
        turn_cleanup
            .diagnostic()
            .map(swallowtail_core::SafeDiagnostic::code),
        Some("swallowtail.opencode.abort_timed_out")
    );

    // 2. close_active bounds a stalled /abort.
    let active: ActiveSlot = Arc::new(Mutex::new(Some(ActiveTurn {
        task: None,
        terminal: Arc::new(AtomicBool::new(false)),
        cancellation: make_cancellation(services.clone()),
        detachment: None,
        attachment: input::SharedAttachment::default(),
    })));
    let active_cleanup = futures_executor::block_on(close_active(&active));
    assert_eq!(
        active_cleanup
            .diagnostic()
            .map(swallowtail_core::SafeDiagnostic::code),
        Some("swallowtail.opencode.abort_timed_out")
    );

    struct FinishedTask;

    impl JoinedTask for FinishedTask {
        fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
    }

    let run_handle = Box::new(OpenCodeRunHandle {
        request_id: RequestId::new("run-stalled-abort").expect("request id"),
        run_id: RuntimeRunId::new("run-stalled-abort").expect("run id"),
        events: None,
        terminal: None,
        callbacks: None,
        cancellation: Arc::new(OpenCodeRunCancellation {
            inner: make_cancellation(services),
            terminal: Arc::new(AtomicBool::new(false)),
            requested: AtomicBool::new(false),
        }),
        task: Box::new(FinishedTask),
    });
    let run_cleanup = run_handle.close().now_or_never().expect("stalled run abort is bounded");
    assert_eq!(
        run_cleanup.diagnostic().map(SafeDiagnostic::code),
        Some("swallowtail.opencode.abort_timed_out")
    );
}

#[test]
fn project_opencode_page_advances_cursor_when_full_page_is_filtered_by_directory() {
    use std::collections::BTreeMap;
    use std::num::NonZeroU32;
    use swallowtail_core::{
        AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, CapabilityProfile,
        CapabilityRequirement, ConfiguredInstance, ConfiguredInstanceId, CredentialState,
        EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState,
        ExecutionHostId, ExtensionNamespace, InstancePolicyId, InstanceRevision,
        InstanceTargetRef, OperationRequirements, PreflightContext, ProtocolFacadeId,
        ProviderSessionCatalogueBounds, RuntimeReadiness, SupportAuthority, preflight,
    };
    use swallowtail_runtime::{
        ProviderSessionCatalogueAgreement, ProviderSessionCatalogueId,
        ProviderSessionCatalogueScope, WorkingResourceRef,
    };

    let host = ExecutionHostId::new("opencode.page-cursor.test").expect("host id");
    let descriptor = opencode_http_descriptor();
    let access_id = AccessProfileId::new("opencode.delegated").expect("access id");
    let access = AccessProfile::new(
        access_id.clone(),
        CredentialMechanism::ProviderSpecific(
            ExtensionNamespace::new("opencode/delegated-auth").expect("namespace"),
        ),
        EntitlementMetering::Unknown,
        EndpointAudience::new("opencode.local").expect("audience"),
        SupportAuthority::IntegrationMaintainerSupported,
    );
    let access_status = AccessStatus::new(
        access_id.clone(),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    );
    let version = crate::selection::opencode_server_binding(
        crate::selection::OPENCODE_LATEST_QUALIFIED_VERSION,
    )
    .expect("version binding");
    let capabilities = [
        CapabilityRequirement::new(Capability::ProviderSessionCatalogue, []),
        crate::prepared::working_resource_capability(ResourceAccess::Read),
    ];
    let instance = ConfiguredInstance::new(
        ConfiguredInstanceId::new("opencode.page-cursor.instance").expect("instance id"),
        InstanceRevision::new("1").expect("revision"),
        descriptor.identity().id().clone(),
        host.clone(),
        InstanceTargetRef::new("opencode.local").expect("target"),
        InstanceOwnership::ExternalAttached,
        access_id.clone(),
        SupportAuthority::IntegrationMaintainerSupported,
        ProtocolFacadeId::new("opencode-http-sse-v1").expect("facade"),
        InstancePolicyId::new("read-only-deny-first").expect("policy"),
        CapabilityProfile::new(capabilities.clone()),
    )
    .with_interface_versions([version.clone()])
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::ProviderSessionCatalogue,
        DriverRole::ProviderSessionCatalogue,
        host,
        AccessRequirement::new(access_id)
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::IntegrationMaintainerSupported]),
    )
    .with_ownership_modes([InstanceOwnership::ExternalAttached])
    .with_host_services(descriptor.required_host_services(DriverRole::ProviderSessionCatalogue))
    .with_capabilities(capabilities)
    .with_interface_versions([version])
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let context = PreflightContext::new(
        &descriptor,
        &instance,
        &access,
        &access_status,
        descriptor.required_host_services(DriverRole::ProviderSessionCatalogue),
    );
    let preflight_plan = preflight(&context, &requirements).expect("preflight plan builds");
    let bounds = ProviderSessionCatalogueBounds::new(
        NonZeroU32::new(2).expect("non-zero"),
        NonZeroU32::new(4).expect("non-zero"),
        NonZeroU32::new(64).expect("non-zero"),
        NonZeroU32::new(128).expect("non-zero"),
        NonZeroU32::new(128).expect("non-zero"),
    )
    .expect("bounds build");
    let plan = ProviderSessionCataloguePlan::new(
        preflight_plan,
        ProviderSessionCatalogueAgreement::new(
            ProviderSessionCatalogueId::new("opencode-catalogue-test").expect("catalogue id"),
            ProviderSessionCatalogueScope::working_resource(
                WorkingResourceRef::new("workspace.main").expect("resource"),
            ),
            bounds,
            None,
        ),
    )
    .expect("catalogue plan builds");

    let observations = vec![
        OpenCodeSessionObservation {
            id: "ses-other-1".to_owned(),
            directory: "/other/workspace".to_owned(),
            title: "other 1".to_owned(),
            updated_at: 1_700_000_000_000,
            version: crate::selection::OPENCODE_LATEST_QUALIFIED_VERSION.to_owned(),
            parent: false,
            archived: false,
        },
        OpenCodeSessionObservation {
            id: "ses-other-2".to_owned(),
            directory: "/other/workspace".to_owned(),
            title: "other 2".to_owned(),
            updated_at: 1_700_000_000_100,
            version: crate::selection::OPENCODE_LATEST_QUALIFIED_VERSION.to_owned(),
            parent: false,
            archived: false,
        },
    ];
    let (candidates, cursor) = project_opencode_page(
        &plan,
        observations,
        &BTreeMap::new(),
        0,
        2,
        "/target/workspace",
        crate::selection::OPENCODE_LATEST_QUALIFIED_VERSION,
    )
    .expect("full filtered page projects cleanly");
    assert!(candidates.is_empty());
    assert_eq!(
        cursor.as_deref(),
        Some("2"),
        "full page filtered by directory must still advance the cursor"
    );

    // A partial page (< limit) ends pagination.
    let (_, partial_cursor) = project_opencode_page(
        &plan,
        vec![OpenCodeSessionObservation {
            id: "ses-other-3".to_owned(),
            directory: "/other/workspace".to_owned(),
            title: "other 3".to_owned(),
            updated_at: 1_700_000_000_200,
            version: crate::selection::OPENCODE_LATEST_QUALIFIED_VERSION.to_owned(),
            parent: false,
            archived: false,
        }],
        &BTreeMap::new(),
        2,
        2,
        "/target/workspace",
        crate::selection::OPENCODE_LATEST_QUALIFIED_VERSION,
    )
    .expect("partial page projects cleanly");
    assert_eq!(partial_cursor, None);
}

#[test]
fn finalize_turn_events_preserves_authentic_status_on_dropped_stream_and_surfaces_live_overflow() {
    let cases = [
        TerminalStatus::Completed,
        TerminalStatus::Cancelled,
        TerminalStatus::TimedOut,
        TerminalStatus::ProviderFailed(SafeDiagnostic::new(
            "swallowtail.opencode.provider_failed",
            "OpenCode reported a provider failure",
        )),
    ];
    for expected in cases {
        let (events, raw_stream) = runtime_event_channel(1).expect("channel constructs");
        let events_dropped = Arc::new(AtomicBool::new(false));
        let stream = ObservedEventStream {
            inner: raw_stream,
            dropped: Arc::clone(&events_dropped),
        };
        // Fill the single-slot buffer, then drop the actual receiver stream
        // without calling sender-side `close()`.
        events
            .send(RuntimeEvent::new(1, RuntimeEventKind::Started))
            .expect("initial event fills buffer");
        drop(stream);
        assert!(events_dropped.load(Ordering::SeqCst));

        let mut activity = crate::activity::OpenCodeActivityProjection::new(
            RuntimeTurnId::new("turn-dropped-events").expect("turn id"),
        );
        let _ = activity
            .project(&Event::ReasoningSnapshot {
                message_id: "msg-1".to_owned(),
                part_id: "part-1".to_owned(),
                text: "thinking".to_owned(),
            })
            .expect("open reasoning activity projects");
        let usage = swallowtail_runtime::TokenUsage::new(Some(10), Some(5));
        let preserved = finalize_turn_events(
            &mut activity,
            &events,
            &events_dropped,
            2,
            expected.clone(),
            Some(usage),
        );
        assert_eq!(
            preserved, expected,
            "dropped event stream must not overwrite authentic terminal status"
        );
    }

    // With a still-attached consumer, buffer overflow on final delivery must
    // surface RuntimeFailed instead of claiming Completed (Contract 009).
    let (events, raw_stream) = runtime_event_channel(1).expect("channel constructs");
    let events_dropped = Arc::new(AtomicBool::new(false));
    let _attached_stream = ObservedEventStream {
        inner: raw_stream,
        dropped: Arc::clone(&events_dropped),
    };
    events
        .send(RuntimeEvent::new(1, RuntimeEventKind::Started))
        .expect("initial event fills buffer");
    let mut activity = crate::activity::OpenCodeActivityProjection::new(
        RuntimeTurnId::new("turn-live-overflow").expect("turn id"),
    );
    let usage = swallowtail_runtime::TokenUsage::new(Some(10), Some(5));
    let outcome = finalize_turn_events(
        &mut activity,
        &events,
        &events_dropped,
        2,
        TerminalStatus::Completed,
        Some(usage),
    );
    assert!(
        matches!(outcome, TerminalStatus::RuntimeFailed(_)),
        "still-attached full event buffer must surface RuntimeFailed, got {outcome:?}"
    );
}
