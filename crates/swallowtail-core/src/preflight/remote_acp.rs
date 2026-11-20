use super::{PreflightDimension, PreflightFailure};
use crate::{
    CredentialState, DriverRole, ExecutionLayer, HostServiceKind, OperationRequirements,
    OperationShape,
};

/// Host services remote ACP requires before any network work (Contract 035).
///
/// The transport runs one operation-scoped executor through `BlockingWork`,
/// owns async tasks, observes monotonic deadlines, and performs the admitted
/// network exchange. It never gains process or credential authority.
const REMOTE_ACP_HOST_SERVICES: [HostServiceKind; 4] = [
    HostServiceKind::Task,
    HostServiceKind::BlockingWork,
    HostServiceKind::Time,
    HostServiceKind::Network,
];

pub(super) fn validate_remote_acp(
    requirements: &OperationRequirements,
) -> Result<(), PreflightFailure> {
    if requirements.remote_acp().is_none() {
        return Ok(());
    }
    if requirements.driver_role() != DriverRole::InteractiveSession
        || requirements.execution_layer() != ExecutionLayer::HarnessInteraction
        || requirements.operation_shape() != OperationShape::InteractiveSession
    {
        return Err(failure("Remote ACP binds one interactive harness session"));
    }
    if !REMOTE_ACP_HOST_SERVICES.iter().all(|service| {
        requirements
            .host_services()
            .any(|required| required == *service)
    }) {
        return Err(failure(
            "Remote ACP requires task, blocking-work, time, and network services",
        ));
    }
    if !requirements
        .access()
        .accepts_credential(CredentialState::NotRequired)
    {
        return Err(failure(
            "Remote ACP's first transport proof is unauthenticated",
        ));
    }
    Ok(())
}

fn failure(message: &'static str) -> PreflightFailure {
    PreflightFailure::new(PreflightDimension::RemoteAcp, message)
}

#[cfg(test)]
mod tests {
    use super::validate_remote_acp;
    use crate::{
        AccessProfileId, AccessRequirement, CredentialState, DriverRole, EntitlementState,
        ExecutionHostId, ExecutionLayer, HostServiceKind, OperationRequirements, OperationShape,
        PreflightDimension, REMOTE_ACP_WIRE_VERSION, RemoteAcpAffinityPolicy,
        RemoteAcpConnectionBounds, RemoteAcpCoreSdkVersion, RemoteAcpRequirements,
        RemoteAcpRfdRevision, RemoteAcpRfdStatus, RemoteAcpTransport, RemoteAcpTransportSdkVersion,
        RemoteAcpVersionEvidence,
    };
    use std::num::{NonZeroU32, NonZeroU64};

    const SERVICES: [HostServiceKind; 4] = [
        HostServiceKind::Task,
        HostServiceKind::BlockingWork,
        HostServiceKind::Time,
        HostServiceKind::Network,
    ];

    fn requirements() -> OperationRequirements {
        OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            OperationShape::InteractiveSession,
            DriverRole::InteractiveSession,
            ExecutionHostId::new("fixture.host.remote").expect("host id is valid"),
            AccessRequirement::new(AccessProfileId::new("fixture.access.remote").expect("profile"))
                .with_credential_states([CredentialState::NotRequired])
                .with_entitlement_states([EntitlementState::Unknown]),
        )
        .with_host_services(SERVICES)
        .with_remote_acp(remote_acp_requirements())
    }

    fn remote_acp_requirements() -> RemoteAcpRequirements {
        RemoteAcpRequirements::new(
            RemoteAcpTransport::StreamableHttpSse,
            RemoteAcpAffinityPolicy::ConnectionScopedCookies {
                maximum_cookie_count: NonZeroU32::new(8).expect("non-zero"),
                maximum_cookie_bytes: NonZeroU64::new(8_192).expect("non-zero"),
            },
            RemoteAcpConnectionBounds::new(
                NonZeroU64::new(65_536).expect("non-zero"),
                NonZeroU32::new(8).expect("non-zero"),
                NonZeroU32::new(4).expect("non-zero"),
                NonZeroU32::new(256).expect("non-zero"),
                NonZeroU32::new(512).expect("non-zero"),
            ),
            RemoteAcpVersionEvidence::new(
                NonZeroU32::new(REMOTE_ACP_WIRE_VERSION).expect("non-zero"),
                RemoteAcpRfdRevision::new("2026-07-21-active").expect("revision"),
                RemoteAcpRfdStatus::Active,
                RemoteAcpTransportSdkVersion::new("2.0.0").expect("version"),
                RemoteAcpCoreSdkVersion::new("2.0.0").expect("version"),
            ),
        )
        .expect("static remote ACP requirements are valid")
    }

    #[test]
    fn a_remote_acp_harness_requirement_passes_preflight() {
        validate_remote_acp(&requirements()).expect("remote ACP harness is admitted");
    }

    #[test]
    fn absent_remote_acp_requirements_are_not_validated() {
        let requirements = OperationRequirements::new(
            ExecutionLayer::DirectModelInference,
            OperationShape::StructuredRun,
            DriverRole::StructuredRun,
            ExecutionHostId::new("fixture.host.local").expect("host id is valid"),
            AccessRequirement::new(AccessProfileId::new("fixture.access.local").expect("profile")),
        );
        validate_remote_acp(&requirements).expect("no remote ACP claim means nothing to validate");
    }

    #[test]
    fn a_missing_remote_acp_host_service_fails_preflight() {
        for missing in SERVICES {
            let services: Vec<_> = SERVICES
                .iter()
                .copied()
                .filter(|service| *service != missing)
                .collect();
            let requirements = OperationRequirements::new(
                ExecutionLayer::HarnessInteraction,
                OperationShape::InteractiveSession,
                DriverRole::InteractiveSession,
                ExecutionHostId::new("fixture.host.remote").expect("host id is valid"),
                AccessRequirement::new(
                    AccessProfileId::new("fixture.access.remote").expect("profile"),
                )
                .with_credential_states([CredentialState::NotRequired]),
            )
            .with_host_services(services)
            .with_remote_acp(remote_acp_requirements());
            let failure = validate_remote_acp(&requirements)
                .expect_err("every remote ACP host service is required");
            assert_eq!(failure.dimension(), PreflightDimension::RemoteAcp);
        }
    }

    #[test]
    fn a_non_interactive_shape_cannot_claim_remote_acp() {
        let requirements = OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            OperationShape::StructuredRun,
            DriverRole::StructuredRun,
            ExecutionHostId::new("fixture.host.remote").expect("host id is valid"),
            AccessRequirement::new(AccessProfileId::new("fixture.access.remote").expect("profile"))
                .with_credential_states([CredentialState::NotRequired]),
        )
        .with_host_services(SERVICES)
        .with_remote_acp(remote_acp_requirements());
        let failure = validate_remote_acp(&requirements)
            .expect_err("remote ACP binds an interactive harness session");
        assert_eq!(failure.dimension(), PreflightDimension::RemoteAcp);
    }

    #[test]
    fn authenticated_access_cannot_claim_remote_acp() {
        let requirements = OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            OperationShape::InteractiveSession,
            DriverRole::InteractiveSession,
            ExecutionHostId::new("fixture.host.remote").expect("host id is valid"),
            AccessRequirement::new(AccessProfileId::new("fixture.access.remote").expect("profile"))
                .with_credential_states([CredentialState::Ready]),
        )
        .with_host_services(SERVICES)
        .with_remote_acp(remote_acp_requirements());
        let failure = validate_remote_acp(&requirements)
            .expect_err("the first remote ACP proof is unauthenticated");
        assert_eq!(failure.dimension(), PreflightDimension::RemoteAcp);
    }
}
