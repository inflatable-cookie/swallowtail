//! Fresh-session driver for the source-tagged Pi SDK Node sidecar.
//!
//! The driver owns no launch argv: the application-approved launch recipe
//! (host-local interpreted Node plus the sidecar entry point) comes from the
//! bound instance target, and application-provisioned paths arrive through
//! the approved environment. The driver binds the exact leased working
//! directory, provider, model, and the four qualified-only version axes
//! before any provider work.

use self::session::{ActiveSlot, PiSdkSidecarSessionHandle};
use self::validation::validate_open;
use crate::failure::failure;
use crate::sidecar::connection::SidecarConnection;
use crate::sidecar::failure::unsupported;
use std::sync::atomic::AtomicU32;
use std::sync::{Arc, Mutex};
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    BoxFuture, CredentialLease, EnvironmentRef, ExecutableRef, HostServices,
    InteractiveSessionDriver, InteractiveSessionHandle, OpenSessionRequest, ProcessHandle,
    ProcessRequest, ResourceAccess, ResourceRepresentation, ResumeSessionRequest, RuntimeFailure,
    RuntimeSessionId, ScopeId, validate_session_resource_lease,
};

mod catalogue;
mod descriptor;
mod handle;
mod input;
mod session;
mod startup;
mod validation;

pub(super) const SIDECAR_DRIVER_ID: &str = "swallowtail.pi.sdk-sidecar";

/// Low-level driver for fresh Pi SDK sidecar sessions.
pub struct PiSdkSidecarDriver {
    environment: EnvironmentRef,
    credential: swallowtail_core::CredentialRef,
}

impl PiSdkSidecarDriver {
    /// Binds the host-private environment and delegated harness credential.
    #[must_use]
    pub const fn new(
        environment: EnvironmentRef,
        credential: swallowtail_core::CredentialRef,
    ) -> Self {
        Self {
            environment,
            credential,
        }
    }
}

impl InteractiveSessionDriver for PiSdkSidecarDriver {
    fn open_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            validate_open(&plan, &request, &services, &self.credential)?;
            if request.deadline().is_some_and(|deadline| {
                services
                    .time()
                    .expect("validated sidecar time service")
                    .now()
                    >= deadline.instant()
            }) {
                return Err(failure(
                    "swallowtail.pi.sdk-sidecar.open_deadline_elapsed",
                    "Pi SDK sidecar session deadline elapsed before startup",
                ));
            }
            self.start_session(plan, request, services)
                .await
                .map(|session| Box::new(session) as Box<dyn InteractiveSessionHandle>)
        })
    }

    fn resume_session(
        &self,
        _plan: PreflightPlan,
        _request: ResumeSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async { Err(unsupported("session resume")) })
    }
}

impl PiSdkSidecarDriver {
    async fn start_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> Result<PiSdkSidecarSessionHandle, RuntimeFailure> {
        let scope = ScopeId::new(format!(
            "pi-sdk-sidecar:session:{}",
            request.request_id().as_str()
        ))
        .map_err(|_| {
            failure(
                "swallowtail.pi.sdk-sidecar.scope_invalid",
                "Pi SDK sidecar scope was invalid",
            )
        })?;
        let credential_service = services
            .credential()
            .cloned()
            .expect("validated sidecar credential service");
        let mut credential = Some(
            credential_service
                .acquire(
                    scope.clone(),
                    self.credential.clone(),
                    plan.endpoint_audience().clone(),
                )
                .await?,
        );
        if !matches!(credential.as_ref(), Some(CredentialLease::Delegated(_)))
            || credential.as_ref().is_some_and(|lease| {
                lease.scope() != &scope
                    || lease.reference() != &self.credential
                    || lease.audience() != plan.endpoint_audience()
            })
        {
            let _ = credential_service
                .release(credential.take().expect("sidecar credential was acquired"))
                .await;
            return Err(failure(
                "swallowtail.pi.sdk-sidecar.credential_lease_rejected",
                "Pi SDK sidecar requires a matching delegated credential lease",
            ));
        }
        let resource_service = services
            .working_resource()
            .cloned()
            .expect("validated sidecar working-resource service");
        let working_resource = request
            .working_resource()
            .expect("validated sidecar working resource")
            .clone();
        let mut resource = match resource_service
            .resolve(
                scope.clone(),
                working_resource.clone(),
                ResourceAccess::Read,
                ResourceRepresentation::Filesystem,
            )
            .await
        {
            Ok(resource) => Some(resource),
            Err(error) => {
                let _ = credential_service
                    .release(credential.take().expect("sidecar credential was acquired"))
                    .await;
                return Err(error);
            }
        };
        if let Err(error) = validate_session_resource_lease(
            request.access_policy(),
            &working_resource,
            resource.as_ref().expect("sidecar resource was resolved"),
        ) {
            let _ = resource_service
                .release(resource.take().expect("sidecar resource was resolved"))
                .await;
            let _ = credential_service
                .release(credential.take().expect("sidecar credential was acquired"))
                .await;
            return Err(error);
        }
        let leased_cwd = resource
            .as_ref()
            .expect("sidecar resource was resolved")
            .filesystem()
            .expect("validated sidecar filesystem lease exposes a root")
            .as_driver_value()
            .to_owned();
        let process_request = ProcessRequest::new(ExecutableRef::from_instance_target(
            plan.instance_target_ref(),
        ))
        .with_environment([self.environment.clone()])
        .with_working_resource(working_resource);
        let process: Arc<dyn ProcessHandle> = match services
            .process()
            .expect("validated sidecar process service")
            .start(scope.clone(), process_request)
            .await
        {
            Ok(process) => Arc::from(process),
            Err(error) => {
                let _ = resource_service
                    .release(resource.take().expect("sidecar resource was resolved"))
                    .await;
                let _ = credential_service
                    .release(credential.take().expect("sidecar credential was acquired"))
                    .await;
                return Err(error);
            }
        };
        let connection = SidecarConnection::new(Arc::clone(&process), services.clone());
        let pump = Arc::clone(&connection);
        let pump_task = match services
            .task()
            .expect("validated sidecar task service")
            .spawn(scope, Box::pin(async move { pump.pump().await }))
        {
            Ok(task) => task,
            Err(error) => {
                let _ = process.force_stop().await;
                let _ = process.wait().await;
                let _ = resource_service
                    .release(resource.take().expect("sidecar resource was resolved"))
                    .await;
                let _ = credential_service
                    .release(credential.take().expect("sidecar credential was acquired"))
                    .await;
                return Err(error);
            }
        };
        if let Err(error) = startup::configure(&connection, &plan, &leased_cwd).await {
            connection.begin_close().await;
            let _ = pump_task.join().await;
            let _ = resource_service
                .release(resource.take().expect("sidecar resource was resolved"))
                .await;
            let _ = credential_service
                .release(credential.take().expect("sidecar credential was acquired"))
                .await;
            return Err(error);
        }
        let runtime_id =
            RuntimeSessionId::new(format!("pi-sdk-sidecar:{}", request.request_id().as_str()))
                .expect("validated request id produces a valid sidecar runtime session id");
        let active: ActiveSlot = Arc::new(Mutex::new(None));
        Ok(PiSdkSidecarSessionHandle {
            request_id: request.request_id().clone(),
            runtime_id,
            execution_host_id: plan.execution_host_id().clone(),
            connection: Arc::clone(&connection),
            cancellation: handle::SessionCancellation::new(connection, Arc::clone(&active)),
            pump_task: Some(pump_task),
            services,
            resource,
            credential,
            active,
            completed_prompts: Arc::new(AtomicU32::new(0)),
            image_attachments: plan
                .requirements()
                .capabilities()
                .any(|required| required.capability() == swallowtail_core::Capability::Attachments),
        })
    }
}

pub use descriptor::pi_sdk_sidecar_descriptor;
