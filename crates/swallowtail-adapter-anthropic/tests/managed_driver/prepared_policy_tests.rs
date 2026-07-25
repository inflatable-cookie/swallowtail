#[test]
fn managed_authority_drift_fails_before_endpoint_or_credential_effects() {
    let fixture = Fixture::new();
    let prepared =
        prepare_anthropic_managed_agent(fixture.preparation_input(), &fixture.services())
            .expect("managed integration prepares");
    let selection = || {
        AnthropicManagedModelSelection::new(
            ModelRouteId::new("anthropic-managed-fixture").expect("route id is valid"),
            ModelRouteRevision::new("prepared-1").expect("route revision is valid"),
            ModelId::new("claude-fixture-model").expect("model id is valid"),
        )
    };
    for input in [
        AnthropicManagedAgentRunInput::new(
            RequestId::new("retention").expect("request id"),
            selection(),
            OperationContent::new("fixture").expect("content"),
            fixture.deadline(),
            [],
            ProviderRetentionPolicy::Prohibited,
            ProviderRecoveryPolicy::ManagedAllowed,
            reattachment(),
        ),
        AnthropicManagedAgentRunInput::new(
            RequestId::new("recovery").expect("request id"),
            selection(),
            OperationContent::new("fixture").expect("content"),
            fixture.deadline(),
            [],
            ProviderRetentionPolicy::DurableAllowed,
            ProviderRecoveryPolicy::Prohibited,
            reattachment(),
        ),
        AnthropicManagedAgentRunInput::new(
            RequestId::new("reattachment").expect("request id"),
            selection(),
            OperationContent::new("fixture").expect("content"),
            fixture.deadline(),
            [],
            ProviderRetentionPolicy::DurableAllowed,
            ProviderRecoveryPolicy::ManagedAllowed,
            StreamReattachmentPolicy::Disabled,
        ),
    ] {
        assert!(prepared.prepare_managed_run(input).is_err());
    }

    let wrong_access = AccessProfile::new(
        AccessProfileId::new("anthropic.direct.public-api").expect("access id"),
        CredentialMechanism::ApiKey,
        EntitlementMetering::PayAsYouGo,
        EndpointAudience::new("api.anthropic.com").expect("audience"),
        SupportAuthority::ProviderSupported,
    )
    .with_credential_reference(fixture.credential.clone());
    let wrong_status = AccessStatus::new(
        wrong_access.id().clone(),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    );
    let wrong = AnthropicManagedPreparationInput::new(
        fixture.instance_id.clone(),
        InstanceRevision::new("wrong-access").expect("revision"),
        fixture.host_id.clone(),
        fixture.target.clone(),
        wrong_access,
        PreparedAccessEvidence::caller_asserted(wrong_status),
        ProviderAgentBinding::new(
            ProviderAgentId::new("agent_fixture").expect("agent"),
            ProviderAgentVersion::new("7").expect("version"),
        ),
    );
    assert!(prepare_anthropic_managed_agent(wrong, &fixture.services()).is_err());
    assert!(fixture.server.requests().is_empty());
    assert_eq!(fixture.credential_releases(), 0);
    assert_eq!(
        prepared.access_profile().id().as_str(),
        ANTHROPIC_MANAGED_ACCESS_PROFILE_ID
    );
}
