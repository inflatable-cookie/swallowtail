use self::launch::arguments;
use crate::connection::PiConnection;
use crate::driver::handle::SessionCancellation;
use crate::driver::session::{ActiveSlot, PiSessionHandle};
use crate::driver::validation::validate_open;
use crate::failure::{failure, unsupported};
use std::sync::atomic::AtomicU32;
use std::sync::{Arc, Mutex};
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    BoxFuture, CredentialLease, EnvironmentRef, ExecutableRef, HostServices,
    InteractiveSessionDriver, InteractiveSessionHandle, OpenSessionRequest, ProcessHandle,
    ProcessRequest, ResourceAccess, ResourceRepresentation, ResumeSessionRequest, RuntimeFailure,
    RuntimeSessionId, ScopeId, validate_session_resource_lease,
};

mod descriptor;
mod handle;
mod launch;
mod session;
mod startup;
mod validation;

pub struct PiRpcDriver {
    environment: EnvironmentRef,
    credential: swallowtail_core::CredentialRef,
}

impl PiRpcDriver {
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

impl InteractiveSessionDriver for PiRpcDriver {
    fn open_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            validate_open(&plan, &request, &services, &self.credential)?;
            if request.deadline().is_some_and(|deadline| {
                services.time().expect("validated Pi time service").now() >= deadline.instant()
            }) {
                return Err(failure(
                    "swallowtail.pi.rpc.open_deadline_elapsed",
                    "Pi RPC session deadline elapsed before startup",
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

impl PiRpcDriver {
    async fn start_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> Result<PiSessionHandle, RuntimeFailure> {
        let scope = ScopeId::new(format!("pi-rpc:session:{}", request.request_id().as_str()))
            .map_err(|_| {
                failure(
                    "swallowtail.pi.rpc.scope_invalid",
                    "Pi RPC scope was invalid",
                )
            })?;
        let credential_service = services
            .credential()
            .cloned()
            .expect("validated Pi credential service");
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
                .release(credential.take().expect("Pi credential was acquired"))
                .await;
            return Err(failure(
                "swallowtail.pi.rpc.credential_lease_rejected",
                "Pi RPC requires a matching delegated credential lease",
            ));
        }
        let resource_service = services
            .working_resource()
            .cloned()
            .expect("validated Pi working-resource service");
        let working_resource = request
            .working_resource()
            .expect("validated Pi working resource")
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
                    .release(credential.take().expect("Pi credential was acquired"))
                    .await;
                return Err(error);
            }
        };
        if let Err(error) = validate_session_resource_lease(
            request.access_policy(),
            &working_resource,
            resource.as_ref().expect("Pi resource was resolved"),
        ) {
            let _ = resource_service
                .release(resource.take().expect("Pi resource was resolved"))
                .await;
            let _ = credential_service
                .release(credential.take().expect("Pi credential was acquired"))
                .await;
            return Err(error);
        }
        let process_request = ProcessRequest::new(ExecutableRef::from_instance_target(
            plan.instance_target_ref(),
        ))
        .with_arguments(arguments(
            plan.provider_id().expect("validated Pi provider").as_str(),
            plan.model_id().expect("validated Pi model").as_str(),
        ))
        .with_environment([self.environment.clone()])
        .with_working_resource(working_resource);
        let process: Arc<dyn ProcessHandle> = match services
            .process()
            .expect("validated Pi process service")
            .start(scope.clone(), process_request)
            .await
        {
            Ok(process) => Arc::from(process),
            Err(error) => {
                let _ = resource_service
                    .release(resource.take().expect("Pi resource was resolved"))
                    .await;
                let _ = credential_service
                    .release(credential.take().expect("Pi credential was acquired"))
                    .await;
                return Err(error);
            }
        };
        let connection = PiConnection::new(
            Arc::clone(&process),
            services.task().cloned().expect("validated Pi task service"),
            services.time().cloned().expect("validated Pi time service"),
        );
        let pump = Arc::clone(&connection);
        let pump_task = match services
            .task()
            .expect("validated Pi task service")
            .spawn(scope, Box::pin(async move { pump.pump().await }))
        {
            Ok(task) => task,
            Err(error) => {
                let _ = process.force_stop().await;
                let _ = process.wait().await;
                let _ = resource_service
                    .release(resource.take().expect("Pi resource was resolved"))
                    .await;
                let _ = credential_service
                    .release(credential.take().expect("Pi credential was acquired"))
                    .await;
                return Err(error);
            }
        };
        if let Err(error) = startup::configure(
            &connection,
            plan.provider_id().expect("validated Pi provider").as_str(),
            plan.model_id().expect("validated Pi model").as_str(),
        )
        .await
        {
            connection.begin_close().await;
            let _ = pump_task.join().await;
            let _ = resource_service
                .release(resource.take().expect("Pi resource was resolved"))
                .await;
            let _ = credential_service
                .release(credential.take().expect("Pi credential was acquired"))
                .await;
            return Err(error);
        }
        let runtime_id = RuntimeSessionId::new(format!("pi-rpc:{}", request.request_id().as_str()))
            .expect("validated request id produces a valid Pi runtime session id");
        let active: ActiveSlot = Arc::new(Mutex::new(None));
        Ok(PiSessionHandle {
            request_id: request.request_id().clone(),
            runtime_id,
            execution_host_id: plan.execution_host_id().clone(),
            connection: Arc::clone(&connection),
            cancellation: SessionCancellation::new(connection, Arc::clone(&active)),
            pump_task: Some(pump_task),
            services,
            resource,
            credential,
            active,
            completed_prompts: Arc::new(AtomicU32::new(0)),
        })
    }
}

pub use descriptor::pi_rpc_descriptor;
