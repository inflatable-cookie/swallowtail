//! Persistent-session driver for the source-tagged Pi SDK Node sidecar.
//!
//! The driver owns no launch argv: the application-approved launch recipe
//! (host-local interpreted Node plus the sidecar entry point) comes from the
//! bound instance target, and application-provisioned paths arrive through
//! the approved environment. New, load, and resume stay distinct: a fresh
//! session returns its initial durable binding, load attaches with bounded
//! typed replay before readiness, and resume attaches without replay.

use self::session::{ActiveSlot, PiSdkSidecarSessionHandle};
use self::validation::validate_open;
use crate::failure::failure;
use crate::sidecar::connection::SidecarConnection;
use std::sync::atomic::AtomicU32;
use std::sync::{Arc, Mutex};
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    BoxFuture, CredentialLease, EnvironmentRef, ExecutableRef, HostServices,
    InteractiveSessionDriver, InteractiveSessionHandle, JoinedTask, LoadSessionRequest,
    LoadedSession, OpenSessionRequest, ProcessHandle, ProcessRequest, ResourceAccess,
    ResourceLease, ResourceRepresentation, ResumeSessionRequest, RuntimeFailure, RuntimeSessionId,
    ScopeId, SessionResumeBinding, WorkingResourceRef, validate_session_resource_lease,
};

mod catalogue;
mod continuity;
mod descriptor;
mod handle;
mod input;
mod session;
mod startup;
mod validation;

pub(super) const SIDECAR_DRIVER_ID: &str = "swallowtail.pi.sdk-sidecar";

/// Low-level driver for persistent Pi SDK sidecar sessions.
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

/// One spawned, verified sidecar attachment before readiness.
pub(super) struct PendingSession {
    pub(super) request_id: swallowtail_runtime::RequestId,
    pub(super) connection: Arc<SidecarConnection>,
    pub(super) pump_task: Option<Box<dyn JoinedTask>>,
    pub(super) services: HostServices,
    pub(super) resource: Option<ResourceLease>,
    pub(super) credential: Option<CredentialLease>,
    pub(super) leased_cwd: String,
}

impl PendingSession {
    /// Closes and joins the sidecar, then releases the resource and
    /// credential leases in contract order. Used when readiness fails.
    pub(in crate::sidecar::driver) async fn abort(mut self) {
        self.connection.begin_close().await;
        if let Some(task) = self.pump_task.take() {
            let _ = task.join().await;
        }
        if let (Some(lease), Some(service)) =
            (self.resource.take(), self.services.working_resource())
        {
            let _ = service.release(lease).await;
        }
        if let (Some(lease), Some(service)) = (self.credential.take(), self.services.credential()) {
            let _ = service.release(lease).await;
        }
    }

    pub(in crate::sidecar::driver) fn into_handle(
        self,
        plan: &PreflightPlan,
        binding: SessionResumeBinding,
    ) -> PiSdkSidecarSessionHandle {
        let active: ActiveSlot = Arc::new(Mutex::new(None));
        PiSdkSidecarSessionHandle {
            request_id: self.request_id.clone(),
            runtime_id: RuntimeSessionId::new(format!(
                "pi-sdk-sidecar:{}",
                self.request_id.as_str()
            ))
            .expect("validated request id produces a valid sidecar runtime session id"),
            execution_host_id: plan.execution_host_id().clone(),
            binding,
            connection: Arc::clone(&self.connection),
            cancellation: handle::SessionCancellation::new(
                Arc::clone(&self.connection),
                Arc::clone(&active),
            ),
            pump_task: self.pump_task,
            services: self.services,
            resource: self.resource,
            credential: self.credential,
            active,
            completed_prompts: Arc::new(AtomicU32::new(0)),
            image_attachments: plan
                .requirements()
                .capabilities()
                .any(|required| required.capability() == swallowtail_core::Capability::Attachments),
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
            let pending = self
                .spawn_session(
                    &plan,
                    request.request_id().clone(),
                    request
                        .working_resource()
                        .expect("validated sidecar working resource")
                        .clone(),
                    request.access_policy(),
                    "session",
                    services,
                )
                .await?;
            let session_ref = match startup::bootstrap(
                &pending.connection,
                &plan,
                &pending.leased_cwd,
                request.options(),
            )
            .await
            {
                Ok(session_ref) => session_ref,
                Err(error) => {
                    pending.abort().await;
                    return Err(error);
                }
            };
            if let Err(error) = startup::check_state(
                &pending.connection,
                &plan,
                &pending.leased_cwd,
                request.options(),
                Some(&session_ref),
            )
            .await
            {
                pending.abort().await;
                return Err(error);
            }
            let binding = session_binding(
                &plan,
                &session_ref,
                request
                    .working_resource()
                    .expect("validated sidecar working resource"),
                request.access_policy(),
            )?;
            Ok(Box::new(pending.into_handle(&plan, binding)) as Box<dyn InteractiveSessionHandle>)
        })
    }

    fn load_session(
        &self,
        plan: PreflightPlan,
        request: LoadSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<LoadedSession, RuntimeFailure>> {
        Box::pin(async move {
            let Some(working_resource) = request.working_resource().cloned() else {
                return Err(crate::sidecar::failure::unsupported(
                    "resource-free session load",
                ));
            };
            let attachment = continuity::AttachmentRequest {
                request_id: request.request_id().clone(),
                binding: request.resume_binding().clone(),
                working_resource,
                deadline: request.deadline(),
                plan_agreement: request.plan_agreement().clone(),
                options: request.options().clone(),
            };
            let attached = self.attach(plan, attachment, services, true).await?;
            Ok(LoadedSession::new(attached.replay, attached.handle))
        })
    }

    fn resume_session(
        &self,
        plan: PreflightPlan,
        request: ResumeSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            let attachment = continuity::AttachmentRequest {
                request_id: request.request_id().clone(),
                binding: request.resume_binding().clone(),
                working_resource: request.working_resource().clone(),
                deadline: request.deadline(),
                plan_agreement: request.plan_agreement().clone(),
                options: request.options().clone(),
            };
            let attached = self.attach(plan, attachment, services, false).await?;
            Ok(attached.handle)
        })
    }
}

impl PiSdkSidecarDriver {
    /// Acquires the credential and resource leases, spawns the sidecar
    /// process, and starts the protocol pump, releasing leases in order on
    /// any failure.
    pub(super) async fn spawn_session(
        &self,
        plan: &PreflightPlan,
        request_id: swallowtail_runtime::RequestId,
        working_resource: WorkingResourceRef,
        access_policy: &swallowtail_core::SessionAccessPolicy,
        kind: &str,
        services: HostServices,
    ) -> Result<PendingSession, RuntimeFailure> {
        let scope = ScopeId::new(format!("pi-sdk-sidecar:{kind}:{}", request_id.as_str()))
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
            access_policy,
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
        Ok(PendingSession {
            request_id,
            connection,
            pump_task: Some(pump_task),
            services,
            resource,
            credential,
            leased_cwd,
        })
    }
}

pub(super) fn session_binding(
    plan: &PreflightPlan,
    session_ref: &str,
    working_resource: &WorkingResourceRef,
    access_policy: &swallowtail_core::SessionAccessPolicy,
) -> Result<SessionResumeBinding, RuntimeFailure> {
    let provider_ref = swallowtail_core::SessionRef::new(session_ref).map_err(|_| {
        failure(
            "swallowtail.pi.sdk-sidecar.session_reference_invalid",
            "Pi SDK sidecar returned an invalid provider session reference",
        )
    })?;
    Ok(SessionResumeBinding::new(
        provider_ref,
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id()
            .expect("validated sidecar model route")
            .clone(),
        plan.model_id().expect("validated sidecar model").clone(),
        working_resource.clone(),
        access_policy.clone(),
    ))
}

pub use descriptor::pi_sdk_sidecar_descriptor;
