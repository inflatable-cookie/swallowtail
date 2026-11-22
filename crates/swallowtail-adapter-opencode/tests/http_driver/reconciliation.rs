#[test]
fn interrupted_session_reconciliation_clears_idle_work_without_claiming_terminal_truth() {
    let server = FixtureServer::start_with_version(StreamFixture::Success, "1.18.10");
    let fixture = Fixture::new_with_version(server.endpoint(), "host.reconciliation", "1.18.10");
    let plan = fixture.plan(DriverRole::ProviderSessionReconciliation);
    let binding = SessionResumeBinding::new(
        swallowtail_core::SessionRef::new("ses_fixture").unwrap(),
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id().unwrap().clone(),
        plan.model_id().unwrap().clone(),
        fixture.resource.clone(),
        SessionAccessPolicy::ambient_harness(swallowtail_core::ResourceAccess::Read),
    );
    let plan = ProviderSessionReconciliationPlan::new(
        plan,
        ProviderSessionReconciliationAgreement::new(
            binding,
            RuntimeTurnId::new("turn-before-restart").unwrap(),
            None,
            ProviderSessionReconciliationBounds::new(
                std::num::NonZeroU32::new(4).unwrap(),
                std::num::NonZeroU64::new(1024).unwrap(),
            ),
            None,
        ),
    )
    .unwrap();
    let request = ProviderSessionReconciliationRequest::from_plan(
        RequestId::new("reconcile-after-restart").unwrap(),
        &plan,
    )
    .unwrap();

    let invalid_terminal = swallowtail_runtime::ProviderSessionReconciliationOutcome::new(
        &plan,
        &request,
        swallowtail_runtime::ProviderSessionReconciliationObservation::session_scoped(
            swallowtail_runtime::InterruptedTurnState::Completed,
            Vec::new(),
            true,
        ),
        swallowtail_runtime::CleanupOutcome::Clean,
    )
    .expect_err("session-scoped evidence cannot claim terminal truth");
    assert_eq!(
        invalid_terminal.diagnostic().code(),
        "swallowtail.provider_session_reconciliation.terminal_attribution_required"
    );

    let outcome = block_on(OpenCodeHttpDriver::new().reconcile_provider_session(
        plan,
        request,
        fixture.services(),
    ))
    .expect("reconciliation succeeds");

    assert_eq!(
        outcome.state(),
        swallowtail_runtime::InterruptedTurnState::InactiveUnresolved
    );
    assert_eq!(
        outcome.attribution(),
        swallowtail_runtime::InterruptedTurnAttribution::ProviderSession
    );
    assert!(outcome.provider_turn_ref().is_none());
    assert!(outcome.replay_complete());
    assert_eq!(
        outcome
            .replay()
            .filter_map(|item| item.content().map(OperationContent::as_str))
            .collect::<Vec<_>>(),
        [
            "Earlier question.",
            "Earlier answer.",
            "Later question.",
            "Later answer."
        ]
    );
    assert!(server.requests().iter().any(|request| request.starts_with("GET /session/status?")));
    assert!(!server.requests().iter().any(|request| request.starts_with("POST ")));
    assert!(!server.requests().iter().any(|request| request.starts_with("DELETE ")));
}

#[test]
fn active_reconciliation_observes_without_aborting_or_resuming_provider_work() {
    let server = FixtureServer::start_with_version(StreamFixture::ReconciliationActive, "1.18.10");
    let fixture = Fixture::new_with_version(
        server.endpoint(),
        "host.active-reconciliation",
        "1.18.10",
    );
    let plan = fixture.plan(DriverRole::ProviderSessionReconciliation);
    let binding = SessionResumeBinding::new(
        swallowtail_core::SessionRef::new("ses_fixture").unwrap(),
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id().unwrap().clone(),
        plan.model_id().unwrap().clone(),
        fixture.resource.clone(),
        SessionAccessPolicy::ambient_harness(swallowtail_core::ResourceAccess::Read),
    );
    let plan = ProviderSessionReconciliationPlan::new(
        plan,
        ProviderSessionReconciliationAgreement::new(
            binding,
            RuntimeTurnId::new("active-turn-before-restart").unwrap(),
            None,
            ProviderSessionReconciliationBounds::new(
                std::num::NonZeroU32::new(4).unwrap(),
                std::num::NonZeroU64::new(1024).unwrap(),
            ),
            None,
        ),
    )
    .unwrap();
    let request = ProviderSessionReconciliationRequest::from_plan(
        RequestId::new("observe-active-after-restart").unwrap(),
        &plan,
    )
    .unwrap();

    let outcome = block_on(OpenCodeHttpDriver::new().reconcile_provider_session(
        plan,
        request,
        fixture.services(),
    ))
    .expect("active reconciliation succeeds");

    assert_eq!(
        outcome.state(),
        swallowtail_runtime::InterruptedTurnState::Active
    );
    assert!(outcome.replay_complete());
    assert_eq!(outcome.replay().count(), 4);
    let requests = server.requests();
    assert!(!requests.iter().any(|request| request.contains("/abort")));
    assert!(!requests.iter().any(|request| request.contains("/prompt_async")));
}

#[test]
fn list_provider_sessions_scans_past_fully_filtered_foreign_page() {
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::num::NonZeroU32;
    use std::sync::{Arc, Mutex};
    use swallowtail_core::ProviderSessionCatalogueBounds;
    use swallowtail_runtime::{
        ProviderSessionCatalogueAgreement, ProviderSessionCatalogueDriver,
        ProviderSessionCatalogueId, ProviderSessionCataloguePlan, ProviderSessionCatalogueRequest,
        ProviderSessionCatalogueScope,
    };

    let listener = TcpListener::bind("127.0.0.1:0").expect("bind loopback");
    let addr = listener.local_addr().expect("local addr");
    let endpoint = format!("http://{addr}");
    let seen = Arc::new(Mutex::new(Vec::<String>::new()));
    let seen_server = Arc::clone(&seen);
    let version = "1.18.10";

    // The fixture approves `fixture-workspace` at `std::env::temp_dir()`.
    let target_dir = std::env::temp_dir().to_string_lossy().into_owned();
    let server = std::thread::spawn(move || {
        for _ in 0..4 {
            let Ok((mut stream, _)) = listener.accept() else {
                break;
            };
            let mut buf = [0u8; 4096];
            let n = stream.read(&mut buf).unwrap_or(0);
            let req = String::from_utf8_lossy(&buf[..n]).into_owned();
            let first_line = req.lines().next().unwrap_or_default().to_owned();
            seen_server.lock().expect("seen lock").push(first_line.clone());

            let body = if first_line.starts_with("GET /global/health") {
                format!(r#"{{"healthy":true,"version":"{version}"}}"#)
            } else if first_line.starts_with("GET /session/status") {
                r#"{"ses_match":{"type":"idle"}}"#.to_owned()
            } else if first_line.contains("start=0") {
                // Full page (limit=2) belonging entirely to another directory.
                format!(
                    r#"[{{"id":"ses_foreign_1","directory":"/other/dir","title":"f1","version":"{version}","time":{{"updated":1700000000000}}}},{{"id":"ses_foreign_2","directory":"/other/dir","title":"f2","version":"{version}","time":{{"updated":1700000000100}}}}]"#
                )
            } else {
                // Second page (start=2) has a matching-directory session.
                format!(
                    r#"[{{"id":"ses_match","directory":"{target_dir}","title":"matched","version":"{version}","time":{{"updated":1700000000200}}}}]"#
                )
            };
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            let _ = stream.write_all(response.as_bytes());
        }
    });

    let fixture = Fixture::new_with_version(&endpoint, "host.catalogue.scan", version);
    let preflight = fixture.plan(DriverRole::ProviderSessionCatalogue);
    let bounds = ProviderSessionCatalogueBounds::new(
        NonZeroU32::new(2).unwrap(),
        NonZeroU32::new(4).unwrap(),
        NonZeroU32::new(64).unwrap(),
        NonZeroU32::new(128).unwrap(),
        NonZeroU32::new(128).unwrap(),
    )
    .expect("bounds");
    let plan = ProviderSessionCataloguePlan::new(
        preflight,
        ProviderSessionCatalogueAgreement::new(
            ProviderSessionCatalogueId::new("opencode-scan-test").unwrap(),
            ProviderSessionCatalogueScope::working_resource(fixture.resource.clone()),
            bounds,
            None,
        ),
    )
    .expect("catalogue plan");
    let request = ProviderSessionCatalogueRequest::from_plan(
        RequestId::new("list-scan").unwrap(),
        &plan,
        None,
    )
    .expect("catalogue request");

    let outcome = block_on(OpenCodeHttpDriver::new().list_provider_sessions(
        plan,
        request,
        fixture.services(),
    ))
    .expect("list_provider_sessions must scan past fully filtered foreign page");

    server.join().expect("fixture server joins");
    let candidates: Vec<_> = outcome.candidates().collect();
    assert_eq!(candidates.len(), 1);
    assert_eq!(candidates[0].display().title(), Some("matched"));
    assert!(outcome.next_cursor().is_none());
    let requests = seen.lock().expect("seen lock");
    assert!(requests.iter().any(|r| r.contains("start=0")));
    assert!(requests.iter().any(|r| r.contains("start=2")));
}
