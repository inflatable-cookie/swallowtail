#[test]
fn prepared_facades_reconcile_separate_acp_and_headless_ledgers() {
    let (acp, _, _) = session(Scenario::Success, true, "prepared-acp");
    let acp_contribution = acp
        .consumer_route_projection_contribution(source("cline.prepared.ledger.acp"))
        .expect("ACP contribution");
    assert_eq!(semantic_ids(&acp_contribution).len(), 7);
    assert!(semantic_ids(&acp_contribution).contains("control.harness-mode"));
    assert_eq!(acp_contribution.applicability().operation_shape(), swallowtail_core::OperationShape::InteractiveSession);
    let headless_contribution = headless_run(true)
        .consumer_route_projection_contribution(source("cline.prepared.ledger.headless"))
        .expect("headless contribution");
    assert_eq!(semantic_ids(&headless_contribution).len(), 7);
    assert!(semantic_ids(&headless_contribution).contains("control.harness-mode"));
    assert_eq!(headless_contribution.applicability().operation_shape(), swallowtail_core::OperationShape::StructuredRun);
    for contribution in [&acp_contribution, &headless_contribution] {
        assert!(!semantic_ids(contribution).contains("feature.model-catalogue"));
        assert!(contribution.selection_rows().chain(contribution.session_start_rows()).chain(contribution.active_session_rows()).all(|row| row.applicability() == contribution.applicability()));
    }
    let acp_mode = projection_rows(&acp_contribution)
        .find(|row| {
            row.identity().namespaced_extension().is_some_and(|extension| {
                extension.semantic_id() == "control.harness-mode"
            })
        })
        .expect("prepared ACP Plan control");
    assert!(acp_mode.state_support().requested());
    assert!(acp_mode.state_support().prepared());
    assert!(acp_mode.state_support().pending());
    assert!(!acp_mode.state_support().provider_effective());
    assert!(!acp_mode.state_support().rejected());
    let headless_mode = projection_rows(&headless_contribution)
        .find(|row| {
            row.identity().namespaced_extension().is_some_and(|extension| {
                extension.semantic_id() == "control.harness-mode"
            })
        })
        .expect("prepared headless Plan control");
    assert!(headless_mode.state_support().requested());
    assert!(headless_mode.state_support().prepared());
    assert!(!headless_mode.state_support().pending());
    assert!(!headless_mode.state_support().provider_effective());
    assert!(!headless_mode.state_support().rejected());
}

#[test]
fn exact_nineteen_row_ledger_reconciles_to_the_reviewed_census() {
    const LEDGER: [(&str, &str, &str, bool); 19] = [
        ("cline.acp", "model-catalogue", "feature.model-catalogue", false),
        ("cline.acp", "interactive-session", "feature.interactive-session", true),
        ("cline.acp", "route-observation", "feature.streaming-events", true),
        ("cline.acp", "route-capability", "feature.cancellation-or-interruption", true),
        ("cline.acp", "route-capability", "feature.working-resource", true),
        ("cline.acp", "session-lifecycle", "feature.persistent-session-posture", false),
        ("cline.acp", "route-capability", "feature.prepared-facade", true),
        ("cline.acp", "route-observation", "feature.activity-observation", true),
        ("cline.acp", "interactive-session", "feature.active-session-plan-ack", true),
        ("cline.acp", "interactive-session", "feature.negotiated-model-options-observation", true),
        ("cline.acp", "interactive-session", "control.harness-mode", true),
        ("cline.headless", "model-catalogue", "feature.model-catalogue", false),
        ("cline.headless", "structured-run", "feature.structured-run", true),
        ("cline.headless", "route-observation", "feature.streaming-events", true),
        ("cline.headless", "route-capability", "feature.cancellation-or-interruption", true),
        ("cline.headless", "route-capability", "feature.working-resource", true),
        ("cline.headless", "route-capability", "feature.prepared-facade", true),
        ("cline.headless", "route-observation", "feature.activity-observation", true),
        ("cline.headless", "structured-run", "control.harness-mode", true),
    ];
    let ledger = LEDGER.iter().map(|(route, shape, semantic, _)| {
        ((*route).to_owned(), (*shape).to_owned(), (*semantic).to_owned())
    }).collect::<BTreeSet<_>>();
    assert_eq!(ledger.len(), 19);
    assert_eq!(ledger, census_tuples(&["cline.acp", "cline.headless"]));
    assert_eq!(LEDGER.iter().filter(|row| row.0 == "cline.acp" && row.3).count(), 9);
    assert_eq!(LEDGER.iter().filter(|row| row.0 == "cline.acp" && !row.3).count(), 2);
    assert_eq!(LEDGER.iter().filter(|row| row.0 == "cline.headless" && row.3).count(), 7);
    assert_eq!(LEDGER.iter().filter(|row| row.0 == "cline.headless" && !row.3).count(), 1);

    let (prepared, _, services) = session(Scenario::ModelExact, true, "ledger-maximal");
    let outcome = block_on(prepared.open_session_with_projection(
        source("cline.ledger.prepared"), source("cline.ledger.active"), services,
    )).unwrap_or_else(|failure| panic!("maximal open failed: {:?}", failure.failure()));
    let mut observed = semantic_ids(outcome.contribution()).into_iter().map(|semantic| {
        ("cline.acp".to_owned(), cline_shape("cline.acp", &semantic).to_owned(), semantic)
    }).collect::<BTreeSet<_>>();
    observed.extend(semantic_ids(
        &headless_run(true).consumer_route_projection_contribution(source("cline.ledger.headless")).expect("headless contributes")
    ).into_iter().map(|semantic| {
        ("cline.headless".to_owned(), cline_shape("cline.headless", &semantic).to_owned(), semantic)
    }));
    let emitted = LEDGER.iter().filter(|row| row.3).map(|row| {
        (row.0.to_owned(), row.1.to_owned(), row.2.to_owned())
    }).collect::<BTreeSet<_>>();
    assert_eq!(observed, emitted);
    let _ = block_on(outcome.into_parts().0.close());
}
