mod plan;

use super::{
    CallLog, FixtureServer, ServerScenario, ThreadServices, TrackingCredential, TrackingNetwork,
};
use std::sync::Arc;
use swallowtail_adapter_xai::{XaiPreparationInput, xai_responses_access_profile};
use swallowtail_core::{
    AccessStatus, CredentialState, EndpointAudience, EndpointAuthorization, ExecutionHostId,
    InstanceRevision, InstanceTargetRef, RuntimeReadiness, SupportAuthority,
};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    BlockingWorkService, CredentialRef, CredentialService, EndpointRef, HostServices,
    NetworkPolicyService, OperationContent, PreparedAccessEvidence, RuntimeTurnId,
    ScopedTaskService, TimeService, TurnRequest,
};

pub fn turn_request(turn_id: &str) -> TurnRequest {
    TurnRequest::new(
        RuntimeTurnId::new(turn_id).expect("turn id is valid"),
        OperationContent::new("fixture input").expect("content is valid"),
    )
}

pub struct DriverFixture {
    pub server: FixtureServer,
    pub calls: CallLog,
    host_id: ExecutionHostId,
    target: InstanceTargetRef,
    audience: EndpointAudience,
    credential: CredentialRef,
    host: LocalProcessHost,
    thread: ThreadServices,
}

impl DriverFixture {
    pub fn new(scenario: ServerScenario) -> Self {
        Self::for_host(
            scenario,
            ExecutionHostId::new("host.xai").expect("host id is valid"),
        )
    }

    pub fn for_host(scenario: ServerScenario, host_id: ExecutionHostId) -> Self {
        let server = FixtureServer::start(scenario);
        let target = InstanceTargetRef::new("xai-fixture-endpoint").expect("target is valid");
        let audience = EndpointAudience::new("api.x.ai").expect("audience is valid");
        let credential = CredentialRef::new("xai-fixture-key").expect("credential is valid");
        let host = LocalProcessHost::builder(LocalProcessLimits::default())
            .approve_endpoint(
                EndpointRef::from_instance_target(&target),
                audience.clone(),
                server.endpoint(),
            )
            .approve_secret_credential(
                credential.clone(),
                audience.clone(),
                b"fixture-secret".to_vec(),
            )
            .build();
        let calls = CallLog::default();
        Self {
            server,
            thread: ThreadServices::new(calls.clone()),
            calls,
            host_id,
            target,
            audience,
            credential,
            host,
        }
    }

    pub fn services(&self) -> HostServices {
        let thread = Arc::new(self.thread.clone());
        HostServices::new(self.host_id.clone())
            .with_task(Arc::clone(&thread) as Arc<dyn ScopedTaskService>)
            .with_blocking_work(Arc::clone(&thread) as Arc<dyn BlockingWorkService>)
            .with_time(thread as Arc<dyn TimeService>)
            .with_network(Arc::new(TrackingNetwork {
                inner: self.host.clone(),
                calls: self.calls.clone(),
            }) as Arc<dyn NetworkPolicyService>)
            .with_credential(Arc::new(TrackingCredential {
                inner: self.host.clone(),
                calls: self.calls.clone(),
            }) as Arc<dyn CredentialService>)
    }

    pub fn preparation_input(&self) -> XaiPreparationInput {
        let access = xai_responses_access_profile(self.credential.clone());
        let status = AccessStatus::new(
            access.id().clone(),
            CredentialState::Ready,
            swallowtail_core::EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ProviderSupported,
        );
        XaiPreparationInput::new(
            InstanceRevision::new("prepared-1").expect("revision is valid"),
            self.host_id.clone(),
            self.target.clone(),
            access,
            PreparedAccessEvidence::caller_asserted(status),
        )
    }
}
