use super::*;

#[test]
fn every_research_version_model_and_value_prepares_exactly() {
    let recording = RecordingHostServices::default();
    for version in ["0.147.0", "0.148.0", "0.149.0", "0.149.1"] {
        let current = prepared(
            CodexPreparedDriver::StructuredExec,
            version,
            &recording,
            false,
        );
        for slug in [
            "gpt-5.6-sol",
            "gpt-5.6-terra",
            "gpt-5.6-luna",
            "gpt-5.5",
            "gpt-5.4",
            "gpt-5.4-mini",
            "gpt-5.2",
        ] {
            for verbosity in [
                CodexModelVerbosity::Low,
                CodexModelVerbosity::Medium,
                CodexModelVerbosity::High,
            ] {
                let selected = CodexModelSelection::new(
                    ModelRouteId::new("codex-model").unwrap(),
                    ModelRouteRevision::new("1").unwrap(),
                    ModelId::new(slug).unwrap(),
                );
                let profile = current
                    .prepare_structured_exec(
                        CodexExecProfileInput::new(
                            RequestId::new("exec-verbosity-matrix").unwrap(),
                            OperationContent::new("private prompt").unwrap(),
                            selected,
                            working_resource(),
                            ExternalNetworkPolicy::Denied,
                            ExternalSearchPolicy::Disabled,
                        )
                        .with_model_verbosity(verbosity),
                    )
                    .expect("admitted verbosity row prepares");
                assert_eq!(profile.model_verbosity(), Some(verbosity));
                assert_eq!(profile.evidence().model_verbosity(), Some(verbosity));
                assert_eq!(profile.plan().model_id().unwrap().as_str(), slug);
            }
        }
    }
}

#[test]
fn model_verbosity_rejects_older_or_foreign_models() {
    let recording = RecordingHostServices::default();

    let older = prepared(
        CodexPreparedDriver::StructuredExec,
        "0.145.0",
        &recording,
        false,
    );
    let older_failure = older
        .prepare_structured_exec(
            CodexExecProfileInput::new(
                RequestId::new("exec-old-verbosity").unwrap(),
                OperationContent::new("private prompt").unwrap(),
                model(),
                working_resource(),
                ExternalNetworkPolicy::Denied,
                ExternalSearchPolicy::Disabled,
            )
            .with_model_verbosity(CodexModelVerbosity::Low),
        )
        .expect_err("older version must fail closed");
    assert_eq!(
        older_failure.diagnostic().safe().code(),
        "swallowtail.codex.preparation.model_verbosity_unsupported"
    );

    let current = prepared(
        CodexPreparedDriver::StructuredExec,
        "0.149.1",
        &recording,
        false,
    );
    let foreign = CodexModelSelection::new(
        ModelRouteId::new("codex-model").unwrap(),
        ModelRouteRevision::new("1").unwrap(),
        ModelId::new("gpt-5-codex").unwrap(),
    );
    let foreign_failure = current
        .prepare_structured_exec(
            CodexExecProfileInput::new(
                RequestId::new("exec-foreign-verbosity").unwrap(),
                OperationContent::new("private prompt").unwrap(),
                foreign,
                working_resource(),
                ExternalNetworkPolicy::Denied,
                ExternalSearchPolicy::Disabled,
            )
            .with_model_verbosity(CodexModelVerbosity::High),
        )
        .expect_err("foreign model must fail closed");
    assert_eq!(
        foreign_failure.diagnostic().safe().code(),
        "swallowtail.codex.preparation.model_verbosity_unsupported"
    );
}
