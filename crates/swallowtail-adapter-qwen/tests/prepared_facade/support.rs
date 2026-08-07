fn preparation_input(host: ExecutionHostId) -> QwenPreparationInput {
    QwenPreparationInput::new(
        ConfiguredInstanceId::new("qwen.prepared").expect("valid instance"),
        InstanceRevision::new("1").expect("valid revision"),
        host,
        InstalledExecutableTarget::new(
            ExecutableRef::new("qwen.prepared.executable").expect("valid executable"),
            InterfaceVersionAxis::new(QWEN_CODE_AXIS).expect("valid axis"),
        ),
        EnvironmentRef::new("qwen.prepared.environment").expect("valid environment"),
        AccessProfile::new(
            AccessProfileId::new("qwen.prepared.access").expect("valid access"),
            CredentialMechanism::ProviderSpecific(
                ExtensionNamespace::new("qwen-code/delegated-harness-auth")
                    .expect("valid namespace"),
            ),
            EntitlementMetering::Unknown,
            EndpointAudience::new("qwen-code").expect("valid audience"),
            SupportAuthority::IntegrationMaintainerSupported,
        ),
        PreparedAccessEvidence::caller_asserted(access_status()),
    )
}

fn access_status() -> AccessStatus {
    AccessStatus::new(
        AccessProfileId::new("qwen.prepared.access").expect("valid access"),
        CredentialState::Ready,
        EntitlementState::Unknown,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    )
}

fn probe() -> QwenPreparationProbe {
    QwenPreparationProbe::new(
        RequestId::new("qwen-prepared-probe").expect("valid request"),
        ScopeId::new("qwen-prepared-probe").expect("valid scope"),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    )
}
