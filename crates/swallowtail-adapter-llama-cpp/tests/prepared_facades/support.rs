fn evidence(access: &AccessProfile) -> PreparedAccessEvidence {
    PreparedAccessEvidence::caller_asserted(AccessStatus::new(
        access.id().clone(),
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    ))
}

struct OwnedStart {
    fixture: OwnedFixture,
    prepared: swallowtail_adapter_llama_cpp::LlamaCppOwnedPreparedIntegration,
    evidence: swallowtail_adapter_llama_cpp::LlamaCppOwnedPreparedEvidence,
}

impl OwnedStart {
    fn evidence(&self) -> &swallowtail_adapter_llama_cpp::LlamaCppOwnedPreparedEvidence {
        &self.evidence
    }
}

fn owned_start(
    context_size: Option<LlamaCppContextSize>,
    reasoning: Option<LlamaCppReasoningSelection>,
) -> OwnedStart {
    let server =
        FixtureServer::start_with(PropertiesFixture::VersionMismatch, StreamFixture::Success);
    let startup = STARTUP_SUCCESS.replace("{{ENDPOINT}}", server.endpoint());
    let fixture = OwnedFixture::new(
        server,
        ScriptedOwnedServices::new(startup, ProcessStop::Graceful),
    );
    let services = fixture.services();
    let access = llama_cpp_owned_access_profile();
    let mut serving =
        LlamaCppOwnedServingSelection::new(fixture.artifact(), model_selection("llama-cpp-b10069"));
    if let Some(context_size) = context_size {
        serving = serving.with_context_size(context_size);
    }
    if let Some(reasoning) = reasoning {
        serving = serving.with_reasoning(reasoning);
    }
    let prepared = prepare_llama_cpp_owned(
        LlamaCppOwnedPreparationInput::new(
            ConfiguredInstanceId::new("llama-cpp.owned.ctx").unwrap(),
            InstanceRevision::new("1").unwrap(),
            fixture.host_id(),
            InstanceTargetRef::new("llama-server.b10069").unwrap(),
            access.clone(),
            evidence(&access),
            serving,
        ),
        &services,
    )
    .expect("owned integration prepares");
    let start = prepared
        .prepare_serving_start(
            ScopeId::new("owned-scope-ctx").unwrap(),
            ServingInstanceId::new("owned-instance-ctx").unwrap(),
            Deadline::at(MonotonicInstant::from_ticks(10_000)),
        )
        .expect("serving start prepares");
    assert_eq!(start.evidence().context_size(), context_size);
    assert_eq!(start.evidence().reasoning(), reasoning);
    let handle = block_on(start.start(services)).expect("ready handle is returned");
    assert_eq!(block_on(handle.stop()), CleanupOutcome::Clean);
    OwnedStart {
        fixture,
        prepared,
        evidence: start.evidence().clone(),
    }
}

fn model_selection(prefix: &str) -> LlamaCppModelSelection {
    LlamaCppModelSelection::new(
        ModelRouteId::new(format!("{prefix}/stories260k")).unwrap(),
        ModelRouteRevision::new("1").unwrap(),
        ModelId::new("swallowtail-fixture-stories260k").unwrap(),
    )
}

fn position(calls: &[OwnedCall], expected: OwnedCall) -> usize {
    calls
        .iter()
        .position(|call| *call == expected)
        .expect("expected call exists")
}
