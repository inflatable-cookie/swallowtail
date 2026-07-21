use crate::rpc::{RpcConnection, failure};
use crate::session_access::{CodexSessionAccess, codex_provider_request_extensions};
use crate::session_input::CodexSessionInput;
use crate::session_open::PendingSessionOpen;
use serde_json::Value;
use std::future::poll_fn;
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, DriverDescriptor, DriverRole, ExecutionLayer,
    HostServiceKind, IntegrationFamilyId, ModelCatalogEntry, ModelId, ModelMetadata,
    OperationShape, PreflightPlan, ReasoningMetadata, ReasoningMode, TransportFamilyId,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, EnvironmentRef, ExecutableRef, HostServices,
    InteractiveSessionDriver, InteractiveSessionHandle, JoinedTask, ModelCatalogDriver,
    ModelCatalogRequest, OpenSessionRequest, ProcessHandle, ProcessRequest, RequestId,
    ResumeSessionRequest, RuntimeFailure, ScopeId, WorkingResourceRef,
};

pub struct CodexAppServerDriver {
    environment: EnvironmentRef,
}

impl CodexAppServerDriver {
    #[must_use]
    pub const fn new(environment: EnvironmentRef) -> Self {
        Self { environment }
    }
}

#[must_use]
pub fn codex_app_server_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new("swallowtail.codex.app-server").expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("codex").expect("static family id is valid"),
        TransportFamilyId::new("jsonl-rpc-stdio").expect("static transport id is valid"),
    )
    .with_roles([DriverRole::ModelCatalog, DriverRole::InteractiveSession])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([OperationShape::InteractiveSession])
    .with_required_host_services(
        DriverRole::ModelCatalog,
        [HostServiceKind::Task, HostServiceKind::Process],
    )
    .with_required_host_services(
        DriverRole::InteractiveSession,
        [HostServiceKind::Task, HostServiceKind::Process],
    )
    .with_extension_namespaces(codex_provider_request_extensions())
}

impl ModelCatalogDriver for CodexAppServerDriver {
    fn list_models(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        Box::pin(async move {
            self.validate_plan(&plan)?;
            let deadline = request
                .deadline()
                .map(|deadline| {
                    services
                        .time()
                        .ok_or_else(|| {
                            failure(
                                "swallowtail.codex.app_server.time_service_missing",
                                "Codex model discovery deadline requires a time service",
                            )
                        })
                        .map(|time| time.wait_until(deadline))
                })
                .transpose()?;
            let scope = scope("catalog", request.request_id());
            let (connection, task) = self
                .start_connection(&plan, scope, None, false, &services)
                .await?;
            let result = match deadline {
                Some(deadline) => {
                    catalog_before_deadline(self.read_catalog(&connection), deadline).await
                }
                None => self.read_catalog(&connection).await,
            };
            let cleanup = close_connection(&connection, task).await;
            match (result, cleanup) {
                (Ok(models), CleanupOutcome::Clean) => Ok(models),
                (Err(error), _) => Err(error),
                (Ok(_), _) => Err(failure(
                    "swallowtail.codex.app_server.catalog_cleanup_failed",
                    "Codex app-server catalog cleanup failed",
                )),
            }
        })
    }
}

async fn catalog_before_deadline<F>(
    catalog: F,
    mut deadline: BoxFuture<'static, swallowtail_runtime::DeadlineObservation>,
) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure>
where
    F: std::future::Future<Output = Result<Vec<ModelCatalogEntry>, RuntimeFailure>>,
{
    let mut catalog = Box::pin(catalog);
    poll_fn(|context| {
        if let Poll::Ready(result) = catalog.as_mut().poll(context) {
            return Poll::Ready(result);
        }
        if deadline.as_mut().poll(context).is_ready() {
            return Poll::Ready(Err(failure(
                "swallowtail.codex.app_server.catalog_timed_out",
                "Codex model discovery timed out",
            )));
        }
        Poll::Pending
    })
    .await
}

impl InteractiveSessionDriver for CodexAppServerDriver {
    fn open_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            validate_session_deadline(request.deadline().is_some())?;
            self.validate_plan(&plan)?;
            let session_input = CodexSessionInput::for_open(&plan, request.options())?;
            let deadline_planned = plan
                .requirements()
                .host_services()
                .any(|service| service == HostServiceKind::Time);
            let model = plan.model_id().ok_or_else(|| {
                failure(
                    "swallowtail.codex.app_server.model_missing",
                    "Codex app-server session requires a preflight-bound model",
                )
            })?;
            let scope = scope("session", request.request_id());
            let access = CodexSessionAccess::prepare(
                &plan,
                request.access_policy(),
                request
                    .working_resource()
                    .ok_or_else(|| unsupported("a resource-free session"))?,
                scope.clone(),
                &services,
            )
            .await?;
            let experimental_api =
                session_input.requires_experimental_api() || access.requires_experimental_api();
            let connection = self
                .start_connection(
                    &plan,
                    scope,
                    Some(access.working_resource().clone()),
                    experimental_api,
                    &services,
                )
                .await;
            let (connection, task) = match connection {
                Ok(connection) => connection,
                Err(error) => {
                    let _ = access.release().await;
                    return Err(error);
                }
            };
            let mut params = serde_json::json!({
                "model": model.as_str(),
                "allowProviderModelFallback": false
            });
            access.apply_thread(&mut params);
            session_input.apply_open(&mut params);
            let response = connection.request("thread/start", params).await;
            PendingSessionOpen::new(
                request.request_id().clone(),
                connection,
                task,
                session_input,
                deadline_planned,
                access,
            )
            .finish(&plan, response, None)
            .await
        })
    }

    fn resume_session(
        &self,
        plan: PreflightPlan,
        request: ResumeSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            validate_session_deadline(request.deadline().is_some())?;
            self.validate_plan(&plan)?;
            let session_input = CodexSessionInput::for_resume(&plan, request.options())?;
            validate_resume_binding(&plan, &request)?;
            let deadline_planned = plan
                .requirements()
                .host_services()
                .any(|service| service == HostServiceKind::Time);
            let model = plan.model_id().ok_or_else(|| {
                failure(
                    "swallowtail.codex.app_server.model_missing",
                    "Codex app-server session requires a preflight-bound model",
                )
            })?;
            let scope = scope("resume", request.request_id());
            let access = CodexSessionAccess::prepare(
                &plan,
                request.access_policy(),
                request.working_resource(),
                scope.clone(),
                &services,
            )
            .await?;
            let experimental_api = access.requires_experimental_api();
            let connection = self
                .start_connection(
                    &plan,
                    scope,
                    Some(access.working_resource().clone()),
                    experimental_api,
                    &services,
                )
                .await;
            let (connection, task) = match connection {
                Ok(connection) => connection,
                Err(error) => {
                    let _ = access.release().await;
                    return Err(error);
                }
            };
            let mut params = serde_json::json!({
                "threadId": request.provider_session_ref().as_provider_value(),
                "model": model.as_str()
            });
            access.apply_thread(&mut params);
            session_input.apply_resume(&mut params);
            let response = connection.request("thread/resume", params).await;
            PendingSessionOpen::new(
                request.request_id().clone(),
                connection,
                task,
                session_input,
                deadline_planned,
                access,
            )
            .finish(
                &plan,
                response,
                Some(request.provider_session_ref().as_provider_value()),
            )
            .await
        })
    }
}

fn validate_session_deadline(has_deadline: bool) -> Result<(), RuntimeFailure> {
    if has_deadline {
        Err(unsupported("session deadlines"))
    } else {
        Ok(())
    }
}

impl CodexAppServerDriver {
    fn validate_plan(&self, plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
        if plan.driver_identity().id().as_str() == "swallowtail.codex.app-server" {
            Ok(())
        } else {
            Err(failure(
                "swallowtail.codex.app_server.plan_driver_mismatch",
                "Preflight plan is bound to a different driver",
            ))
        }
    }

    async fn start_connection(
        &self,
        plan: &PreflightPlan,
        scope: ScopeId,
        working_resource: Option<WorkingResourceRef>,
        experimental_api: bool,
        services: &HostServices,
    ) -> Result<(Arc<RpcConnection>, Box<dyn JoinedTask>), RuntimeFailure> {
        services.require_execution_host(plan.execution_host_id())?;
        let task_service = services.task().cloned().ok_or_else(|| {
            failure(
                "swallowtail.codex.app_server.task_service_missing",
                "Codex app-server requires a scoped task service",
            )
        })?;
        let process_service = services.process().cloned().ok_or_else(|| {
            failure(
                "swallowtail.codex.app_server.process_service_missing",
                "Codex app-server requires a process service",
            )
        })?;
        let executable = ExecutableRef::from_instance_target(plan.instance_target_ref());
        let mut process_request = ProcessRequest::new(executable)
            .with_arguments([
                "app-server".to_owned(),
                "--listen".to_owned(),
                "stdio://".to_owned(),
            ])
            .with_environment([self.environment.clone()]);
        if let Some(resource) = working_resource {
            process_request = process_request.with_working_resource(resource);
        }
        let process: Arc<dyn ProcessHandle> = Arc::from(
            process_service
                .start(scope.clone(), process_request)
                .await?,
        );
        let connection = RpcConnection::new(Arc::clone(&process));
        let pump_connection = Arc::clone(&connection);
        let task = match task_service
            .spawn(scope, Box::pin(async move { pump_connection.pump().await }))
        {
            Ok(task) => task,
            Err(error) => {
                let _ = process.force_stop().await;
                let _ = process.wait().await;
                return Err(error);
            }
        };
        if let Err(error) = connection.initialize(experimental_api).await {
            let _ = connection.cancel_session().await;
            let _ = task.join().await;
            return Err(error);
        }
        Ok((connection, task))
    }

    async fn read_catalog(
        &self,
        connection: &RpcConnection,
    ) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
        let mut models = Vec::new();
        let mut cursor: Option<String> = None;
        loop {
            let response = connection
                .request(
                    "model/list",
                    serde_json::json!({"cursor": cursor, "includeHidden": false}),
                )
                .await?;
            let data = response
                .get("data")
                .and_then(Value::as_array)
                .ok_or_else(malformed_response)?;
            for model in data {
                let id = required_text(model, "model")?;
                models.push(ModelCatalogEntry::new(
                    ModelId::new(id).map_err(|_| malformed_response())?,
                    model_metadata(model)?,
                ));
            }
            cursor = response
                .get("nextCursor")
                .and_then(Value::as_str)
                .map(str::to_owned);
            if cursor.is_none() {
                return Ok(models);
            }
        }
    }
}

fn validate_resume_binding(
    plan: &PreflightPlan,
    request: &ResumeSessionRequest,
) -> Result<(), RuntimeFailure> {
    if request.resume_binding().matches_attachment(
        plan,
        request.working_resource(),
        request.access_policy(),
    ) {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.codex.app_server.resume_binding_mismatch",
            "Codex app-server resume binding does not match the preflight plan",
        ))
    }
}

fn model_metadata(model: &Value) -> Result<ModelMetadata, RuntimeFailure> {
    let display_name = required_text(model, "displayName")?;
    let description = required_text(model, "description")?;
    let is_default = model
        .get("isDefault")
        .and_then(Value::as_bool)
        .ok_or_else(malformed_response)?;
    let options = model
        .get("supportedReasoningEfforts")
        .and_then(Value::as_array)
        .ok_or_else(malformed_response)?;
    let supported = options
        .iter()
        .map(|option| {
            required_text(option, "reasoningEffort")
                .and_then(|value| ReasoningMode::new(value).map_err(|_| malformed_response()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let default = ReasoningMode::new(required_text(model, "defaultReasoningEffort")?)
        .map_err(|_| malformed_response())?;
    let reasoning = ReasoningMetadata::new(supported, Some(default.clone()));
    if !reasoning.supports(&default) {
        return Err(malformed_response());
    }
    Ok(ModelMetadata::with_display_name(display_name)
        .and_then(|metadata| metadata.with_description(description))
        .map_err(|_| malformed_response())?
        .with_default(is_default)
        .with_reasoning(reasoning))
}

async fn close_connection(
    connection: &Arc<RpcConnection>,
    task: Box<dyn JoinedTask>,
) -> CleanupOutcome {
    let close = connection.close_input().await;
    let join = task.join().await;
    if close.is_err() || join.is_err() {
        CleanupOutcome::Failed(swallowtail_core::SafeDiagnostic::new(
            "swallowtail.codex.app_server.close_failed",
            "Codex app-server connection cleanup failed",
        ))
    } else {
        connection.cleanup_outcome()
    }
}

fn required_text<'a>(value: &'a Value, field: &str) -> Result<&'a str, RuntimeFailure> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(malformed_response)
}

fn malformed_response() -> RuntimeFailure {
    failure(
        "swallowtail.codex.app_server.malformed_response",
        "Codex app-server returned a malformed response",
    )
}

fn unsupported(feature: &str) -> RuntimeFailure {
    failure(
        "swallowtail.codex.app_server.unsupported_input",
        format!("Codex app-server proof driver does not support {feature}"),
    )
}

fn scope(kind: &str, request_id: &RequestId) -> ScopeId {
    ScopeId::new(format!("codex-app-server:{kind}:{}", request_id.as_str()))
        .expect("request id produces a valid scope id")
}
