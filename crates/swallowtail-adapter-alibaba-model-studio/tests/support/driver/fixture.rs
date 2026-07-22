use super::server::{FixtureRequest, FixtureServer, ServerScenario};
use super::services::{CallLog, DriverCall, ThreadServices, TrackingCredential};
use std::sync::{Arc, Mutex};
use swallowtail_adapter_alibaba_model_studio::{
    alibaba_model_studio_access_profile, alibaba_model_studio_descriptor,
    alibaba_model_studio_instance, alibaba_model_studio_requirements, alibaba_model_studio_route,
};
use swallowtail_core::{
    AccessProfileId, AccessStatus, CredentialState, EndpointAuthorization, EntitlementState,
    ExecutionHostId, PreflightContext, PreflightPlan, RuntimeReadiness, SupportAuthority,
    preflight,
};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    BlockingWorkService, CredentialService, EndpointRef, HostServices, NetworkPolicyService,
    ScopedTaskService, TimeService,
};

pub struct DriverFixture {
    pub server: FixtureServer,
    pub calls: CallLog,
    host_id: ExecutionHostId,
    host: LocalProcessHost,
    thread: ThreadServices,
    release_after_blocking: Arc<Mutex<Vec<usize>>>,
}

impl DriverFixture {
    pub fn new(scenario: ServerScenario) -> Self {
        Self::for_host(
            scenario,
            ExecutionHostId::new("fixture.host.local").expect("host id is valid"),
        )
    }

    pub fn for_host(scenario: ServerScenario, host_id: ExecutionHostId) -> Self {
        let server = FixtureServer::start(scenario);
        let instance = alibaba_model_studio_instance(host_id.clone());
        let access = alibaba_model_studio_access_profile();
        let credential = access
            .credential_reference()
            .expect("credential is bound")
            .clone();
        let host = LocalProcessHost::builder(LocalProcessLimits::default())
            .approve_endpoint(
                EndpointRef::from_instance_target(instance.target_reference()),
                access.endpoint_audience().clone(),
                server.endpoint(),
            )
            .approve_secret_credential(
                credential,
                access.endpoint_audience().clone(),
                b"fixture-secret".to_vec(),
            )
            .build();
        let calls = CallLog::default();
        let thread = ThreadServices::new(calls.clone());
        Self {
            server,
            calls,
            host_id,
            host,
            thread,
            release_after_blocking: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn services(&self) -> HostServices {
        let thread = Arc::new(self.thread.clone());
        HostServices::new(self.host_id.clone())
            .with_task(Arc::clone(&thread) as Arc<dyn ScopedTaskService>)
            .with_blocking_work(Arc::clone(&thread) as Arc<dyn BlockingWorkService>)
            .with_time(thread as Arc<dyn TimeService>)
            .with_network(Arc::new(self.host.clone()) as Arc<dyn NetworkPolicyService>)
            .with_credential(Arc::new(TrackingCredential {
                inner: self.host.clone(),
                calls: self.calls.clone(),
                blocking: self.thread.clone(),
                releases_after_blocking: Arc::clone(&self.release_after_blocking),
            }) as Arc<dyn CredentialService>)
    }

    pub fn plan(&self) -> PreflightPlan {
        let descriptor = alibaba_model_studio_descriptor();
        let instance = alibaba_model_studio_instance(self.host_id.clone());
        let access = alibaba_model_studio_access_profile();
        let route = alibaba_model_studio_route();
        let requirements = alibaba_model_studio_requirements(self.host_id.clone());
        let status = ready_status(access.id().clone());
        let services: Vec<_> = descriptor
            .required_host_services(swallowtail_core::DriverRole::InteractiveSession)
            .collect();
        preflight(
            &PreflightContext::new(&descriptor, &instance, &access, &status, services)
                .with_model_route(&route),
            &requirements,
        )
        .expect("Alibaba Model Studio preflight succeeds")
    }

    pub fn requests(&self) -> Vec<FixtureRequest> {
        self.server.requests()
    }
    pub fn release_after_blocking(&self) -> Vec<usize> {
        self.release_after_blocking
            .lock()
            .expect("release-order lock")
            .clone()
    }
    pub fn releases(&self) -> usize {
        self.calls.count(DriverCall::CredentialRelease)
    }

    pub fn deadline_after(&self, milliseconds: u64) -> swallowtail_runtime::Deadline {
        swallowtail_runtime::Deadline::at(swallowtail_runtime::MonotonicInstant::from_ticks(
            self.thread.now().ticks().saturating_add(milliseconds),
        ))
    }
}

fn ready_status(profile_id: AccessProfileId) -> AccessStatus {
    AccessStatus::new(
        profile_id,
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    )
}
