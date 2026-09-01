#[test]
fn owned_facade_binds_context_size_across_evidence_driver_and_argv() {
    let omitted = owned_start(None, None);
    assert_eq!(omitted.evidence().context_size(), None);
    assert_eq!(omitted.prepared.context_size(), None);
    assert_eq!(omitted.evidence().reasoning(), None);
    assert_eq!(omitted.prepared.reasoning(), None);
    assert_eq!(
        omitted.fixture.owned.arguments(),
        [
            "--model",
            "/private/models/fixture.gguf",
            "--alias",
            "swallowtail-fixture-stories260k",
            "--host",
            "127.0.0.1",
            "--port",
            "0",
            "--offline",
            "--no-ui",
            "--no-agent",
        ]
    );

    for value in [1_u32, 4096, i32::MAX as u32] {
        let selected = LlamaCppContextSize::from_u64(u64::from(value)).expect("admitted value");
        let started = owned_start(Some(selected), None);
        assert_eq!(started.evidence().context_size(), Some(selected));
        assert_eq!(started.prepared.context_size(), Some(selected));
        assert_eq!(started.evidence().reasoning(), None);
        assert_eq!(started.prepared.reasoning(), None);
        assert_eq!(
            started.fixture.owned.arguments(),
            [
                "--model".to_owned(),
                "/private/models/fixture.gguf".to_owned(),
                "--alias".to_owned(),
                "swallowtail-fixture-stories260k".to_owned(),
                "--host".to_owned(),
                "127.0.0.1".to_owned(),
                "--port".to_owned(),
                "0".to_owned(),
                "--offline".to_owned(),
                "--no-ui".to_owned(),
                "--no-agent".to_owned(),
                "--ctx-size".to_owned(),
                value.to_string(),
            ]
        );
    }
}

#[test]
fn owned_facade_binds_reasoning_across_evidence_driver_and_argv() {
    let selected = LlamaCppReasoningSelection::Disabled;
    let started = owned_start(None, Some(selected));
    assert_eq!(started.evidence().reasoning(), Some(selected));
    assert_eq!(started.prepared.reasoning(), Some(selected));
    assert_eq!(started.evidence().context_size(), None);
    assert_eq!(
        started.fixture.owned.arguments(),
        [
            "--model",
            "/private/models/fixture.gguf",
            "--alias",
            "swallowtail-fixture-stories260k",
            "--host",
            "127.0.0.1",
            "--port",
            "0",
            "--offline",
            "--no-ui",
            "--no-agent",
            "--reasoning",
            "off",
        ]
    );
}

#[test]
fn owned_facade_composes_context_size_and_reasoning_without_interference() {
    let context_size = LlamaCppContextSize::from_u64(8192).expect("admitted value");
    let reasoning = LlamaCppReasoningSelection::Disabled;
    let started = owned_start(Some(context_size), Some(reasoning));
    assert_eq!(started.evidence().context_size(), Some(context_size));
    assert_eq!(started.evidence().reasoning(), Some(reasoning));
    assert_eq!(started.prepared.context_size(), Some(context_size));
    assert_eq!(started.prepared.reasoning(), Some(reasoning));
    assert_eq!(
        started.fixture.owned.arguments(),
        [
            "--model",
            "/private/models/fixture.gguf",
            "--alias",
            "swallowtail-fixture-stories260k",
            "--host",
            "127.0.0.1",
            "--port",
            "0",
            "--offline",
            "--no-ui",
            "--no-agent",
            "--ctx-size",
            "8192",
            "--reasoning",
            "off",
        ]
    );
}
