use self::launch::arguments;
use crate::connection::OhMyPiConnection;
use crate::driver::handle::SessionCancellation;
use crate::driver::session::{ActiveSlot, OhMyPiSessionHandle};
use crate::driver::validation::validate_open;
use crate::failure::{failure, unsupported};
use std::sync::atomic::AtomicU32;
use std::sync::{Arc, Mutex};
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    BoxFuture, EnvironmentRef, ExecutableRef, HostServices, InteractiveSessionDriver,
    InteractiveSessionHandle, OpenSessionRequest, ProcessHandle, ProcessRequest, ResourceAccess,
    ResourceRepresentation, ResumeSessionRequest, RuntimeFailure, RuntimeSessionId, ScopeId,
    validate_session_resource_lease,
};

mod catalogue;
mod descriptor;
mod handle;
mod input;
mod launch;
mod run;
mod session;
mod startup;
pub(crate) mod validation;

pub struct OhMyPiRpcDriver {
    environment: EnvironmentRef,
}

impl OhMyPiRpcDriver {
    #[must_use]
    pub const fn new(environment: EnvironmentRef) -> Self {
        Self { environment }
    }
}

impl InteractiveSessionDriver for OhMyPiRpcDriver {
    fn open_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            validate_open(&plan, &request, &services)?;
            if request.deadline().is_some_and(|deadline| {
                services
                    .time()
                    .expect("validated OhMyPi time service")
                    .now()
                    >= deadline.instant()
            }) {
                return Err(failure(
                    "swallowtail.oh_my_pi.rpc.open_deadline_elapsed",
                    "OhMyPi RPC session deadline elapsed before startup",
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

impl OhMyPiRpcDriver {
    async fn start_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> Result<OhMyPiSessionHandle, RuntimeFailure> {
        let scope = ScopeId::new(format!(
            "oh-my-pi-rpc:session:{}",
            request.request_id().as_str()
        ))
        .map_err(|_| {
            failure(
                "swallowtail.oh_my_pi.rpc.scope_invalid",
                "OhMyPi RPC scope was invalid",
            )
        })?;
        let resource_service = services
            .working_resource()
            .cloned()
            .expect("validated OhMyPi working-resource service");
        let working_resource = request
            .working_resource()
            .expect("validated OhMyPi working resource")
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
            Err(error) => return Err(error),
        };
        if let Err(error) = validate_session_resource_lease(
            request.access_policy(),
            &working_resource,
            resource.as_ref().expect("OhMyPi resource was resolved"),
        ) {
            let _ = resource_service
                .release(resource.take().expect("OhMyPi resource was resolved"))
                .await;
            return Err(error);
        }
        let process_request = ProcessRequest::new(ExecutableRef::from_instance_target(
            plan.instance_target_ref(),
        ))
        .with_arguments(arguments(
            plan.provider_id()
                .expect("validated OhMyPi provider")
                .as_str(),
            plan.model_id().expect("validated OhMyPi model").as_str(),
        ))
        .with_environment([self.environment.clone()])
        .with_working_resource(working_resource);
        let process: Arc<dyn ProcessHandle> = match services
            .process()
            .expect("validated OhMyPi process service")
            .start(scope.clone(), process_request)
            .await
        {
            Ok(process) => Arc::from(process),
            Err(error) => {
                let _ = resource_service
                    .release(resource.take().expect("OhMyPi resource was resolved"))
                    .await;
                return Err(error);
            }
        };
        let connection = OhMyPiConnection::new(
            Arc::clone(&process),
            services
                .task()
                .cloned()
                .expect("validated OhMyPi task service"),
            services
                .time()
                .cloned()
                .expect("validated OhMyPi time service"),
        );
        let pump = Arc::clone(&connection);
        let pump_task = match services
            .task()
            .expect("validated OhMyPi task service")
            .spawn(scope, Box::pin(async move { pump.pump().await }))
        {
            Ok(task) => task,
            Err(error) => {
                let _ = process.force_stop().await;
                let _ = process.wait().await;
                let _ = resource_service
                    .release(resource.take().expect("OhMyPi resource was resolved"))
                    .await;
                return Err(error);
            }
        };
        if let Err(error) = startup::configure(
            &connection,
            plan.provider_id()
                .expect("validated OhMyPi provider")
                .as_str(),
            plan.model_id().expect("validated OhMyPi model").as_str(),
            request.options().reasoning_mode(),
        )
        .await
        {
            connection.begin_close().await;
            let _ = pump_task.join().await;
            let _ = resource_service
                .release(resource.take().expect("OhMyPi resource was resolved"))
                .await;
            return Err(error);
        }
        let runtime_id =
            RuntimeSessionId::new(format!("oh-my-pi-rpc:{}", request.request_id().as_str()))
                .expect("validated request id produces a valid OhMyPi runtime session id");
        let active: ActiveSlot = Arc::new(Mutex::new(None));
        Ok(OhMyPiSessionHandle {
            request_id: request.request_id().clone(),
            runtime_id,
            execution_host_id: plan.execution_host_id().clone(),
            connection: Arc::clone(&connection),
            cancellation: SessionCancellation::new(connection, Arc::clone(&active)),
            pump_task: Some(pump_task),
            services,
            resource,
            active,
            completed_prompts: Arc::new(AtomicU32::new(0)),
            image_attachments: plan
                .requirements()
                .capabilities()
                .any(|required| required.capability() == swallowtail_core::Capability::Attachments),
        })
    }
}

pub use descriptor::oh_my_pi_rpc_descriptor;
