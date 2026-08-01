use crate::{
    ProviderSessionImportFixture, RecordingHostServices, RecordingOutcome, poll_immediate,
    provider_session_catalogue_bounds,
};
use std::sync::{Arc, Mutex};
use swallowtail_core::{
    CancellationScope, ProviderSessionActivityState, ProviderSessionBindingOrigin,
    ProviderSessionImportAvailability, ProviderSessionImportUnavailableReason, SafeDiagnostic,
    SessionRef,
};
use swallowtail_runtime::{
    BoxFuture, CancellationControl, CleanupOutcome, HostServices, ImmediateCancellation,
    InteractiveSessionDriver, InteractiveSessionHandle, LoadSessionRequest, LoadedSession,
    OpenSessionRequest, OperationContent, ProviderSessionCatalogueOutcome,
    ProviderSessionCatalogueRequest, ProviderSessionImportOutcome, ProviderSessionImportRequest,
    ProviderSessionImportRevalidation, ProviderSessionOperationFailure,
    ProviderSessionOperationFailureStage, RequestId, ResumeSessionRequest, RuntimeFailure,
    RuntimeSessionId, SessionReplayItem, SessionReplayKind, TurnHandle, TurnRequest,
    validate_provider_session_catalogue_execution, validate_provider_session_import_execution,
};

/// Runs the provider-neutral catalogue/import conformance pack.
pub fn assert_provider_session_import_contract() {
    assert_topology_and_prepared_evidence();
    assert_bounds_pagination_and_redaction();
    assert_drift_and_stale_targets_fail_closed();
    assert_lifecycle_failures_remain_distinct();
    assert_import_load_and_resume_sequence();
}

fn fixtures() -> [ProviderSessionImportFixture; 2] {
    [
        ProviderSessionImportFixture::local(),
        ProviderSessionImportFixture::remote_authoritative(),
    ]
}

fn standard_bounds() -> swallowtail_core::ProviderSessionCatalogueBounds {
    provider_session_catalogue_bounds(2, 4, 64, 128, 128)
}

fn assert_topology_and_prepared_evidence() {
    for fixture in fixtures() {
        let catalogue = fixture.catalogue_plan("fixture-catalogue", standard_bounds());
        let prepared_catalogue = fixture.prepared_catalogue(catalogue.clone());
        assert!(
            prepared_catalogue
                .operation()
                .matches_plan(catalogue.preflight())
        );

        let request = ProviderSessionCatalogueRequest::from_plan(
            request_id("fixture-catalogue-request"),
            &catalogue,
            None,
        )
        .expect("catalogue request is valid");
        let host = RecordingHostServices::for_host(
            fixture.topology().execution_host_id().clone(),
            RecordingOutcome::Succeed,
        );
        validate_provider_session_catalogue_execution(&catalogue, &request, host.services())
            .expect("catalogue execution retains exact host authority");

        let candidate = fixture
            .candidate(
                &catalogue,
                "fixture-candidate",
                "provider/private/session",
                ProviderSessionImportAvailability::Available,
            )
            .expect("candidate is bounded");
        let import = fixture
            .import_plan(catalogue, candidate)
            .expect("import plan is valid");
        let prepared_import = fixture.prepared_import(import.clone());
        assert!(prepared_import.operation().matches_plan(import.preflight()));
        assert_eq!(
            prepared_import.plan().preflight().execution_host_id(),
            fixture.topology().execution_host_id()
        );
        let request =
            ProviderSessionImportRequest::from_plan(request_id("fixture-import-request"), &import)
                .expect("import request is valid");
        validate_provider_session_import_execution(&import, &request, host.services())
            .expect("import execution retains exact host authority");
        assert!(host.calls().is_empty());
    }
}

fn assert_bounds_pagination_and_redaction() {
    let fixture = ProviderSessionImportFixture::local();
    let catalogue = fixture.catalogue_plan("fixture-bounded", standard_bounds());
    let first = fixture
        .candidate(
            &catalogue,
            "fixture-candidate-a",
            "provider/private/session-a",
            ProviderSessionImportAvailability::Available,
        )
        .expect("first candidate is bounded");
    let second = fixture
        .candidate(
            &catalogue,
            "fixture-candidate-b",
            "provider/private/session-b",
            ProviderSessionImportAvailability::Available,
        )
        .expect("second candidate is bounded");
    let request = ProviderSessionCatalogueRequest::from_plan(
        request_id("fixture-page-one"),
        &catalogue,
        None,
    )
    .expect("first page request is valid");
    let first_page = ProviderSessionCatalogueOutcome::new(
        &catalogue,
        &request,
        vec![first.clone(), second],
        Some("private-next-cursor".to_owned()),
        CleanupOutcome::Clean,
    )
    .expect("bounded page is valid");
    assert_eq!(first_page.candidates().len(), 2);
    assert_eq!(
        first_page
            .next_cursor()
            .expect("next cursor exists")
            .observed_candidates(),
        2
    );

    let next_request = ProviderSessionCatalogueRequest::from_plan(
        request_id("fixture-page-two"),
        &catalogue,
        first_page.next_cursor().cloned(),
    )
    .expect("second page request is valid");
    let duplicate = ProviderSessionCatalogueOutcome::new(
        &catalogue,
        &next_request,
        vec![first],
        None,
        CleanupOutcome::Clean,
    )
    .expect_err("cross-page duplicate must fail");
    assert_eq!(
        duplicate.diagnostic().code(),
        "swallowtail.provider_session_catalogue.duplicate_candidate"
    );

    let overflow = (0..3)
        .map(|index| {
            fixture
                .candidate(
                    &catalogue,
                    &format!("fixture-overflow-{index}"),
                    &format!("provider/private/overflow-{index}"),
                    ProviderSessionImportAvailability::Available,
                )
                .expect("overflow candidate is individually bounded")
        })
        .collect::<Vec<_>>();
    let page_failure = ProviderSessionCatalogueOutcome::new(
        &catalogue,
        &request,
        overflow,
        None,
        CleanupOutcome::Clean,
    )
    .expect_err("page-size overflow must fail");
    assert_eq!(
        page_failure.diagnostic().code(),
        "swallowtail.provider_session_catalogue.page_limit_exceeded"
    );

    let tight = fixture.catalogue_plan(
        "fixture-tight",
        provider_session_catalogue_bounds(1, 1, 4, 4, 4),
    );
    let content_failure = fixture
        .candidate(
            &tight,
            "fixture-private-candidate",
            "p",
            ProviderSessionImportAvailability::Available,
        )
        .expect_err("oversized provider content must fail");
    let reference_failure = fixture
        .candidate(
            &tight,
            "fixture-private-reference",
            "provider/private/oversized",
            ProviderSessionImportAvailability::Available,
        )
        .expect_err("oversized provider identity must fail");
    assert_eq!(
        content_failure.diagnostic().code(),
        "swallowtail.provider_session_catalogue.content_limit_exceeded"
    );
    assert_eq!(
        reference_failure.diagnostic().code(),
        "swallowtail.provider_session_catalogue.reference_limit_exceeded"
    );
    let diagnostic = format!("{content_failure:?}{reference_failure:?}");
    assert!(!diagnostic.contains("private provider title"));
    assert!(!diagnostic.contains("provider/private/oversized"));
    let page_debug = format!("{first_page:?}");
    assert!(!page_debug.contains("private-next-cursor"));
    assert!(!page_debug.contains("private provider title"));
    assert!(!page_debug.contains("private provider preview"));
    assert!(!page_debug.contains("provider/private/session"));
}

fn assert_drift_and_stale_targets_fail_closed() {
    let local = ProviderSessionImportFixture::local();
    let remote = ProviderSessionImportFixture::remote_authoritative();
    let local_catalogue = local.catalogue_plan("fixture-local", standard_bounds());
    let remote_catalogue = remote.catalogue_plan("fixture-remote", standard_bounds());
    let local_candidate = local
        .candidate(
            &local_catalogue,
            "fixture-local-candidate",
            "provider/private/local",
            ProviderSessionImportAvailability::Available,
        )
        .expect("local candidate is valid");

    let copied = remote
        .import_plan(remote_catalogue.clone(), local_candidate)
        .expect_err("copied cross-plan candidate must fail");
    assert_eq!(
        copied.diagnostic().code(),
        "swallowtail.provider_session_import.candidate_plan_mismatch"
    );

    let remote_request = ProviderSessionCatalogueRequest::from_plan(
        request_id("fixture-remote-page"),
        &remote_catalogue,
        None,
    )
    .expect("remote request is valid");
    let remote_candidate = remote
        .candidate(
            &remote_catalogue,
            "fixture-remote-candidate",
            "provider/private/remote",
            ProviderSessionImportAvailability::Available,
        )
        .expect("remote candidate is valid");
    let remote_page = ProviderSessionCatalogueOutcome::new(
        &remote_catalogue,
        &remote_request,
        vec![remote_candidate],
        Some("private-remote-cursor".to_owned()),
        CleanupOutcome::Clean,
    )
    .expect("remote page is valid");
    let cross_plan_cursor = ProviderSessionCatalogueRequest::from_plan(
        request_id("fixture-cross-plan-page"),
        &local_catalogue,
        remote_page.next_cursor().cloned(),
    )
    .expect_err("cross-plan cursor must fail before effects");
    assert_eq!(
        cross_plan_cursor.diagnostic().code(),
        "swallowtail.provider_session_catalogue.cursor_plan_mismatch"
    );

    let selected = local
        .candidate(
            &local_catalogue,
            "fixture-selected",
            "provider/private/selected",
            ProviderSessionImportAvailability::Available,
        )
        .expect("selected candidate is valid");
    let import = local
        .import_plan(local_catalogue, selected)
        .expect("import plan is valid");
    let request =
        ProviderSessionImportRequest::from_plan(request_id("fixture-stale-import"), &import)
            .expect("import request is valid");
    for revalidation in [
        ProviderSessionImportRevalidation::new(
            import.agreement().candidate_id().clone(),
            session_ref("provider/private/disappeared"),
            local.topology().working_resource().clone(),
            ProviderSessionActivityState::Inactive,
            ProviderSessionImportAvailability::Available,
        ),
        ProviderSessionImportRevalidation::new(
            import.agreement().candidate_id().clone(),
            session_ref("provider/private/selected"),
            local.topology().working_resource().clone(),
            ProviderSessionActivityState::Active,
            ProviderSessionImportAvailability::Unavailable(
                ProviderSessionImportUnavailableReason::Active,
            ),
        ),
    ] {
        let failure = ProviderSessionImportOutcome::new(
            &import,
            &request,
            revalidation,
            CleanupOutcome::Clean,
        )
        .expect_err("stale or unavailable target must issue no binding");
        assert_eq!(
            failure.stage(),
            ProviderSessionOperationFailureStage::ImportRevalidation
        );
        assert!(!format!("{failure:?}").contains("provider/private"));
    }
}

fn assert_lifecycle_failures_remain_distinct() {
    let fixture = ProviderSessionImportFixture::local();
    let catalogue = fixture.catalogue_plan("fixture-lifecycle", standard_bounds());
    let request = ProviderSessionCatalogueRequest::from_plan(
        request_id("fixture-cancelled"),
        &catalogue,
        None,
    )
    .expect("catalogue request is valid");
    poll_immediate(request.cancellation().request()).expect("cancellation is recorded");
    assert!(request.cancellation().is_requested());
    assert_eq!(
        request.cancellation().scope(),
        CancellationScope::ProviderSessionCatalogue
    );

    let lifecycle = [
        ProviderSessionOperationFailure::new(
            ProviderSessionOperationFailureStage::Cancelled,
            SafeDiagnostic::new("fixture.cancelled", "Catalogue was cancelled"),
        ),
        ProviderSessionOperationFailure::new(
            ProviderSessionOperationFailureStage::TimedOut,
            SafeDiagnostic::new("fixture.timed_out", "Catalogue deadline elapsed"),
        ),
        ProviderSessionOperationFailure::new(
            ProviderSessionOperationFailureStage::CatalogueDispatch,
            SafeDiagnostic::new("fixture.disconnected", "Catalogue transport disconnected"),
        ),
    ];
    assert_eq!(
        lifecycle.map(|failure| failure.stage()),
        [
            ProviderSessionOperationFailureStage::Cancelled,
            ProviderSessionOperationFailureStage::TimedOut,
            ProviderSessionOperationFailureStage::CatalogueDispatch,
        ]
    );

    let cleanup = ProviderSessionCatalogueOutcome::new(
        &catalogue,
        &request,
        Vec::new(),
        None,
        CleanupOutcome::Failed(SafeDiagnostic::new(
            "fixture.cleanup_failed",
            "Fixture cleanup failed",
        )),
    )
    .expect_err("cleanup failure must prevent success");
    assert_eq!(
        cleanup.stage(),
        ProviderSessionOperationFailureStage::Cleanup
    );
}

fn assert_import_load_and_resume_sequence() {
    for fixture in fixtures() {
        let catalogue = fixture.catalogue_plan("fixture-continuation", standard_bounds());
        let selected = fixture
            .candidate(
                &catalogue,
                "fixture-continuation-candidate",
                "provider/private/continuation",
                ProviderSessionImportAvailability::Available,
            )
            .expect("continuation candidate is valid");
        let import = fixture
            .import_plan(catalogue, selected)
            .expect("continuation import is valid");
        let request = ProviderSessionImportRequest::from_plan(
            request_id("fixture-continuation-import"),
            &import,
        )
        .expect("continuation import request is valid");
        let outcome = ProviderSessionImportOutcome::new(
            &import,
            &request,
            ProviderSessionImportRevalidation::new(
                import.agreement().candidate_id().clone(),
                session_ref("provider/private/continuation"),
                fixture.topology().working_resource().clone(),
                ProviderSessionActivityState::Inactive,
                ProviderSessionImportAvailability::Available,
            ),
            CleanupOutcome::Clean,
        )
        .expect("matching revalidation issues one binding");
        assert_eq!(
            outcome.binding().origin(),
            ProviderSessionBindingOrigin::ExplicitlyImported
        );
        assert!(outcome.binding().matches_attachment(
            import.preflight(),
            fixture.topology().working_resource(),
            &fixture.access_policy(),
        ));

        let load = LoadSessionRequest::from_plan(
            import.preflight(),
            request_id("fixture-load-imported"),
            outcome.binding().clone(),
            fixture.topology().working_resource().clone(),
            None,
        )
        .expect("imported binding enters ordinary load");
        let resume = ResumeSessionRequest::from_plan(
            import.preflight(),
            request_id("fixture-resume-imported"),
            outcome.binding().clone(),
            fixture.topology().working_resource().clone(),
            None,
        )
        .expect("imported binding enters ordinary resume");
        let events = Arc::new(Mutex::new(Vec::new()));
        let driver = ContinuationFixtureDriver {
            events: Arc::clone(&events),
        };
        let host = RecordingHostServices::for_host(
            fixture.topology().execution_host_id().clone(),
            RecordingOutcome::Succeed,
        );
        let loaded = poll_immediate(driver.load_session(
            import.preflight().clone(),
            load,
            host.services().clone(),
        ))
        .expect("load returns replay and a ready handle");
        assert_eq!(
            loaded
                .replay()
                .map(SessionReplayItem::sequence)
                .collect::<Vec<_>>(),
            [0, 1]
        );
        let (_replay, loaded_handle) = loaded.into_parts();
        poll_immediate(loaded_handle.close());
        let resumed = poll_immediate(driver.resume_session(
            import.preflight().clone(),
            resume,
            host.services().clone(),
        ))
        .expect("resume returns a ready handle without replay");
        poll_immediate(resumed.close());
        assert_eq!(
            *events.lock().expect("fixture event lock is valid"),
            [
                ContinuationEvent::Replay(0),
                ContinuationEvent::Replay(1),
                ContinuationEvent::ReadyAfterLoad,
                ContinuationEvent::ReadyAfterResume,
            ]
        );
        assert!(host.calls().is_empty());
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ContinuationEvent {
    Replay(u64),
    ReadyAfterLoad,
    ReadyAfterResume,
}

struct ContinuationFixtureDriver {
    events: Arc<Mutex<Vec<ContinuationEvent>>>,
}

impl InteractiveSessionDriver for ContinuationFixtureDriver {
    fn open_session(
        &self,
        _plan: swallowtail_core::PreflightPlan,
        _request: OpenSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async { Err(fixture_failure("fixture open is not used")) })
    }

    fn resume_session(
        &self,
        _plan: swallowtail_core::PreflightPlan,
        request: ResumeSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        self.events
            .lock()
            .expect("fixture event lock is valid")
            .push(ContinuationEvent::ReadyAfterResume);
        let handle = ContinuationFixtureHandle::new(
            request.request_id().clone(),
            request.resume_binding().clone(),
        );
        Box::pin(async move { Ok(Box::new(handle) as Box<dyn InteractiveSessionHandle>) })
    }

    fn load_session(
        &self,
        _plan: swallowtail_core::PreflightPlan,
        request: LoadSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<LoadedSession, RuntimeFailure>> {
        let provider_ref = request.provider_session_ref().clone();
        let replay = vec![
            SessionReplayItem::with_content(
                provider_ref.clone(),
                0,
                SessionReplayKind::UserMessage,
                OperationContent::new("private historical prompt")
                    .expect("fixture replay is valid"),
            ),
            SessionReplayItem::with_content(
                provider_ref,
                1,
                SessionReplayKind::AgentMessage,
                OperationContent::new("private historical response")
                    .expect("fixture replay is valid"),
            ),
        ];
        let mut events = self.events.lock().expect("fixture event lock is valid");
        events.extend(
            replay
                .iter()
                .map(|item| ContinuationEvent::Replay(item.sequence())),
        );
        events.push(ContinuationEvent::ReadyAfterLoad);
        drop(events);
        let handle = ContinuationFixtureHandle::new(
            request.request_id().clone(),
            request.resume_binding().clone(),
        );
        Box::pin(async move { Ok(LoadedSession::new(replay, Box::new(handle))) })
    }
}

struct ContinuationFixtureHandle {
    request_id: RequestId,
    session_id: RuntimeSessionId,
    binding: swallowtail_runtime::SessionResumeBinding,
    cancellation: ImmediateCancellation,
}

impl ContinuationFixtureHandle {
    fn new(request_id: RequestId, binding: swallowtail_runtime::SessionResumeBinding) -> Self {
        Self {
            request_id,
            session_id: RuntimeSessionId::new("fixture-imported-session")
                .expect("fixture runtime session id is valid"),
            binding,
            cancellation: ImmediateCancellation::new(CancellationScope::InteractiveSession),
        }
    }
}

impl InteractiveSessionHandle for ContinuationFixtureHandle {
    fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    fn session_id(&self) -> &RuntimeSessionId {
        &self.session_id
    }

    fn provider_session_ref(&self) -> Option<&SessionRef> {
        Some(self.binding.provider_session_ref())
    }

    fn resume_binding(&self) -> Option<&swallowtail_runtime::SessionResumeBinding> {
        Some(&self.binding)
    }

    fn start_turn<'a>(
        &'a mut self,
        _request: TurnRequest,
        _services: HostServices,
    ) -> BoxFuture<'a, Result<Box<dyn TurnHandle>, RuntimeFailure>> {
        Box::pin(async { Err(fixture_failure("fixture turn is not used")) })
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        &self.cancellation
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async { CleanupOutcome::Clean })
    }
}

fn request_id(input: &str) -> RequestId {
    RequestId::new(input).expect("fixture request id is valid")
}

fn session_ref(input: &str) -> SessionRef {
    SessionRef::new(input).expect("fixture provider session ref is valid")
}

fn fixture_failure(message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new("fixture.unused", message))
}

#[cfg(test)]
mod tests {
    #[test]
    fn provider_session_import_contract_passes() {
        super::assert_provider_session_import_contract();
    }
}
