use super::lifecycle::{
    ProbeInterrupt, complete_probe_work, complete_probe_work_outcome, terminal_failure,
};
use super::{
    OpenCodePreparationInput, OpenCodePreparationProbe, OpenCodePreparedServerObservation, failure,
    runtime_failure, validation,
};
use crate::protocol::{Request, observe_health};
use crate::transport::CurlTransport;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use swallowtail_runtime::{
    CredentialLease, EndpointRef, HostServices, PreparationFailure, PreparationStage,
};

pub(super) async fn observe_server(
    input: &OpenCodePreparationInput,
    probe: &OpenCodePreparationProbe,
    services: &HostServices,
) -> Result<OpenCodePreparedServerObservation, PreparationFailure> {
    let endpoint_ref = EndpointRef::from_instance_target(&input.endpoint_target);
    let audience = input.access_profile.endpoint_audience().clone();
    let cancelled = Arc::new(AtomicBool::new(false));
    let network = services.network().expect("validated network service");
    let grant = complete_probe_work(
        network.authorize(
            probe.scope_id.clone(),
            endpoint_ref.clone(),
            audience.clone(),
        ),
        probe,
        services,
        Arc::clone(&cancelled),
        ProbeInterrupt::Abandon,
    )
    .await
    .map_err(|error| runtime_failure(PreparationStage::TargetSelection, error))?;
    if grant.scope() != &probe.scope_id
        || grant.endpoint() != &endpoint_ref
        || grant.audience() != &audience
    {
        return Err(failure(
            PreparationStage::TargetSelection,
            "swallowtail.opencode.preparation.network_grant_mismatch",
            "OpenCode preparation network grant did not match the selected target",
        ));
    }

    let credential_service = services.credential().expect("validated credential service");
    let credential_reference = input
        .access_profile
        .credential_reference()
        .expect("validated credential reference")
        .clone();
    let (credential, terminal) = complete_probe_work_outcome(
        credential_service.acquire(
            probe.scope_id.clone(),
            credential_reference.clone(),
            audience.clone(),
        ),
        probe,
        services,
        Arc::clone(&cancelled),
        ProbeInterrupt::Abandon,
    )
    .await;
    if let Some(terminal) = terminal {
        let terminal = runtime_failure(PreparationStage::BoundedOutput, terminal_failure(terminal));
        return match credential {
            Some(Ok(credential)) => match credential_service.release(credential).await {
                swallowtail_runtime::CleanupOutcome::Clean
                | swallowtail_runtime::CleanupOutcome::NotApplicable => Err(terminal),
                _ => Err(cleanup_failure(terminal)),
            },
            Some(Err(_)) | None => Err(terminal),
        };
    }
    let credential = credential
        .expect("uninterrupted credential acquisition yields a result")
        .map_err(|error| runtime_failure(PreparationStage::AccessEvidence, error))?;
    if credential.scope() != &probe.scope_id
        || credential.reference() != &credential_reference
        || credential.audience() != &audience
        || !matches!(credential, CredentialLease::Delegated(_))
    {
        let cleanup = credential_service.release(credential).await;
        let mismatch = failure(
            PreparationStage::AccessEvidence,
            "swallowtail.opencode.preparation.credential_lease_rejected",
            "OpenCode preparation requires a matching delegated credential lease",
        );
        return match cleanup {
            swallowtail_runtime::CleanupOutcome::Clean
            | swallowtail_runtime::CleanupOutcome::NotApplicable => Err(mismatch),
            _ => Err(cleanup_failure(mismatch)),
        };
    }

    let result = observe_health_response(
        grant.authorized().as_driver_value().to_owned(),
        probe,
        services,
        cancelled,
    )
    .await;
    let cleanup = credential_service.release(credential).await;
    match (result, cleanup) {
        (
            Ok(server),
            swallowtail_runtime::CleanupOutcome::Clean
            | swallowtail_runtime::CleanupOutcome::NotApplicable,
        ) => Ok(server),
        (Err(error), swallowtail_runtime::CleanupOutcome::Clean)
        | (Err(error), swallowtail_runtime::CleanupOutcome::NotApplicable) => Err(error),
        (Ok(_), _) => Err(failure(
            PreparationStage::Cleanup,
            "swallowtail.opencode.preparation.cleanup_failed",
            "OpenCode preparation credential cleanup failed",
        )),
        (Err(error), _) => Err(cleanup_failure(error)),
    }
}

async fn observe_health_response(
    endpoint: String,
    probe: &OpenCodePreparationProbe,
    services: &HostServices,
    cancelled: Arc<AtomicBool>,
) -> Result<OpenCodePreparedServerObservation, PreparationFailure> {
    let response = complete_probe_work(
        CurlTransport.request(
            probe.scope_id.clone(),
            endpoint,
            Request::get("/global/health"),
            services,
            Arc::clone(&cancelled),
        ),
        probe,
        services,
        cancelled,
        ProbeInterrupt::Drain,
    )
    .await
    .map_err(|error| runtime_failure(PreparationStage::BoundedOutput, error))?;
    let observation = observe_health(&response).map_err(validation::health_failure)?;
    Ok(OpenCodePreparedServerObservation::new(
        observation.binding().clone(),
        observation.assessment().clone(),
    ))
}

fn cleanup_failure(cause: PreparationFailure) -> PreparationFailure {
    failure(
        PreparationStage::Cleanup,
        "swallowtail.opencode.preparation.cleanup_failed",
        "OpenCode preparation credential cleanup failed",
    )
    .with_cause(cause)
}

#[cfg(test)]
mod tests {
    use super::super::{
        OpenCodePreparationInput, OpenCodePreparationProbe, prepare_opencode_attached,
    };
    use std::sync::Arc;
    use swallowtail_core::{
        AccessProfile, AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialMechanism,
        CredentialRef, CredentialState, EndpointAudience, EndpointAuthorization,
        EntitlementMetering, EntitlementState, ExecutionHostId, ExtensionNamespace,
        InstanceRevision, InstanceTargetRef, RuntimeReadiness, SupportAuthority,
    };
    use swallowtail_runtime::{
        AuthorizedEndpoint, BlockingJob, BlockingWorkService, BoxFuture, CleanupOutcome,
        CredentialLease, CredentialService, Deadline, DeadlineObservation, DiscoveryCancellation,
        EndpointRef, HostServices, MonotonicInstant, NetworkGrant, NetworkPolicyService,
        PreparationStage, PreparedAccessEvidence, RuntimeFailure, ScopeId, TimeService,
    };

    struct PastClock;

    impl TimeService for PastClock {
        fn now(&self) -> MonotonicInstant {
            MonotonicInstant::from_ticks(10)
        }

        fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
            Box::pin(async move {
                DeadlineObservation::new(deadline, MonotonicInstant::from_ticks(u64::MAX))
            })
        }
    }

    struct UnusedBlocking;

    impl BlockingWorkService for UnusedBlocking {
        fn run(
            &self,
            _scope: ScopeId,
            _job: BlockingJob,
        ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            Box::pin(std::future::pending())
        }
    }

    struct StalledNetwork;

    impl NetworkPolicyService for StalledNetwork {
        fn authorize(
            &self,
            _scope: ScopeId,
            _endpoint: EndpointRef,
            _audience: EndpointAudience,
        ) -> BoxFuture<'static, Result<NetworkGrant, RuntimeFailure>> {
            Box::pin(std::future::pending())
        }
    }

    struct OkNetwork;

    impl NetworkPolicyService for OkNetwork {
        fn authorize(
            &self,
            scope: ScopeId,
            endpoint: EndpointRef,
            audience: EndpointAudience,
        ) -> BoxFuture<'static, Result<NetworkGrant, RuntimeFailure>> {
            Box::pin(async move {
                Ok(NetworkGrant::new(
                    scope,
                    endpoint,
                    audience,
                    AuthorizedEndpoint::new("http://127.0.0.1:4096").expect("valid endpoint"),
                ))
            })
        }
    }

    struct StalledCredential;

    impl CredentialService for StalledCredential {
        fn acquire(
            &self,
            _scope: ScopeId,
            _reference: CredentialRef,
            _audience: EndpointAudience,
        ) -> BoxFuture<'static, Result<CredentialLease, RuntimeFailure>> {
            Box::pin(std::future::pending())
        }

        fn release(&self, _lease: CredentialLease) -> BoxFuture<'static, CleanupOutcome> {
            Box::pin(async { CleanupOutcome::Clean })
        }
    }

    fn preparation_input(host_id: &ExecutionHostId) -> OpenCodePreparationInput {
        let audience = EndpointAudience::new("opencode.local").expect("valid audience");
        let credential_reference =
            CredentialRef::new("opencode.delegated.token").expect("valid credential ref");
        let access_profile = AccessProfile::new(
            AccessProfileId::new("opencode.delegated").expect("valid access profile id"),
            CredentialMechanism::ProviderSpecific(
                ExtensionNamespace::new("opencode/delegated-auth").expect("valid namespace"),
            ),
            EntitlementMetering::Unknown,
            audience,
            SupportAuthority::IntegrationMaintainerSupported,
        )
        .with_credential_reference(credential_reference);
        let access_evidence = PreparedAccessEvidence::caller_asserted(AccessStatus::new(
            access_profile.id().clone(),
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::IntegrationMaintainerSupported,
        ));
        OpenCodePreparationInput::new(
            ConfiguredInstanceId::new("opencode.stalled-probe").expect("valid instance id"),
            InstanceRevision::new("rev-1").expect("valid revision"),
            host_id.clone(),
            InstanceTargetRef::new("http://127.0.0.1:4096").expect("valid target"),
            access_profile,
            access_evidence,
        )
    }

    #[test]
    fn stalled_network_authorize_is_abandoned_at_probe_deadline() {
        let host_id = ExecutionHostId::new("opencode.stalled.network").expect("valid host id");
        let services = HostServices::new(host_id.clone())
            .with_time(Arc::new(PastClock))
            .with_blocking_work(Arc::new(UnusedBlocking))
            .with_network(Arc::new(StalledNetwork))
            .with_credential(Arc::new(StalledCredential));
        let probe = OpenCodePreparationProbe::new(
            ScopeId::new("opencode.probe.stalled-net").expect("valid scope"),
            Deadline::at(MonotonicInstant::from_ticks(100)),
            DiscoveryCancellation::new(),
        );
        let error = futures_executor::block_on(prepare_opencode_attached(
            preparation_input(&host_id),
            probe,
            services,
        ))
        .expect_err("stalled network.authorize must time out rather than hang");
        assert_eq!(error.stage(), PreparationStage::TargetSelection);
        assert_eq!(
            error.diagnostic().safe().code(),
            "swallowtail.opencode.preparation.timed_out"
        );
    }

    #[test]
    fn stalled_credential_acquire_is_abandoned_at_probe_deadline() {
        let host_id = ExecutionHostId::new("opencode.stalled.credential").expect("valid host id");
        let services = HostServices::new(host_id.clone())
            .with_time(Arc::new(PastClock))
            .with_blocking_work(Arc::new(UnusedBlocking))
            .with_network(Arc::new(OkNetwork))
            .with_credential(Arc::new(StalledCredential));
        let probe = OpenCodePreparationProbe::new(
            ScopeId::new("opencode.probe.stalled-cred").expect("valid scope"),
            Deadline::at(MonotonicInstant::from_ticks(100)),
            DiscoveryCancellation::new(),
        );
        let error = futures_executor::block_on(prepare_opencode_attached(
            preparation_input(&host_id),
            probe,
            services,
        ))
        .expect_err("stalled credential_service.acquire must time out rather than hang");
        assert_eq!(error.stage(), PreparationStage::BoundedOutput);
        assert_eq!(
            error.diagnostic().safe().code(),
            "swallowtail.opencode.preparation.timed_out"
        );
    }
}
