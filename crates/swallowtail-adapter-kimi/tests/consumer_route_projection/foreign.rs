use super::*;

#[test]
fn concrete_foreign_and_unbounded_values_keep_the_preserved_failure() {
    for (scenario, forbidden) in [
        (
            Scenario::ReasoningForeign,
            "swallowtail.kimi.acp.reasoning_value_foreign",
        ),
        (
            Scenario::ReasoningUnbounded,
            "swallowtail.kimi.acp.reasoning_value_unbounded",
        ),
    ] {
        let host_id = host_id(forbidden);
        let host = FixtureHost::new(scenario);
        let prepared = session(
            &host,
            host_id.clone(),
            SessionOptions::default()
                .with_reasoning_mode(ReasoningMode::new("high").expect("reasoning")),
        );
        let failure = match block_on(prepared.open_session_with_projection(
            source("kimi.case2.prepared"),
            source("kimi.case2.active"),
            host.services(host_id),
        )) {
            Ok(_) => panic!("concrete mismatch must reject"),
            Err(failure) => failure,
        };
        assert!(matches!(failure, KimiProjectionOpenFailure::Runtime(_)));
        assert_eq!(
            failure.failure().diagnostic().code(),
            "swallowtail.negotiated_reasoning.effective_mismatch"
        );
        assert_ne!(failure.failure().diagnostic().code(), forbidden);
        assert!(failure.rejected_contribution().is_none());
    }
}

#[test]
fn normalized_on_keeps_preserved_open_but_projection_rejects_unpublishable_tokens() {
    for (scenario, expected) in [
        (
            Scenario::ReasoningForeign,
            "swallowtail.kimi.acp.reasoning_value_foreign",
        ),
        (
            Scenario::ReasoningUnbounded,
            "swallowtail.kimi.acp.reasoning_value_unbounded",
        ),
    ] {
        let options = SessionOptions::default()
            .with_reasoning_mode(ReasoningMode::new("on").expect("reasoning"));
        let preserved_host_id = host_id(&format!("preserved-{expected}"));
        let preserved_host = FixtureHost::new(scenario);
        let preserved = session(&preserved_host, preserved_host_id.clone(), options.clone());
        let handle = block_on(preserved.open_session(preserved_host.services(preserved_host_id)))
            .expect("preserved normalization opens");
        assert_eq!(
            block_on(close_session(
                handle,
                preserved_host.services(host_id(&format!("preserved-{expected}"))),
            )),
            swallowtail_runtime::CleanupOutcome::Clean
        );

        let projected_host_id = host_id(&format!("projected-{expected}"));
        let projected_host = FixtureHost::new(scenario);
        let projected = session(&projected_host, projected_host_id.clone(), options);
        let failure = match block_on(projected.open_session_with_projection(
            source("kimi.case4.prepared"),
            source("kimi.case4.active"),
            projected_host.services(projected_host_id),
        )) {
            Ok(_) => panic!("unpublishable token must fail projected open"),
            Err(failure) => failure,
        };
        assert!(matches!(failure, KimiProjectionOpenFailure::Runtime(_)));
        assert_eq!(failure.failure().diagnostic().code(), expected);
        assert!(failure.rejected_contribution().is_none());
        assert_eq!(projected_host.cleanup_counts(), (1, 1));
    }
}
