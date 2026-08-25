use super::*;

#[test]
fn model_verbosity_prepares_on_admitted_ceiling_and_rejects_older_or_foreign_models() {
    let recording = RecordingHostServices::default();
    let current = prepared(
        CodexPreparedDriver::StructuredExec,
        "0.149.1",
        &recording,
        false,
    );
    let profile = current
        .prepare_structured_exec(
            CodexExecProfileInput::new(
                RequestId::new("exec-verbosity").unwrap(),
                OperationContent::new("private prompt").unwrap(),
                model(),
                working_resource(),
                ExternalNetworkPolicy::Denied,
                ExternalSearchPolicy::Disabled,
            )
            .with_model_verbosity(CodexModelVerbosity::Medium),
        )
        .expect("admitted verbosity prepares");
    assert_eq!(profile.model_verbosity(), Some(CodexModelVerbosity::Medium));
    assert_eq!(
        profile.evidence().model_verbosity(),
        Some(CodexModelVerbosity::Medium)
    );

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
