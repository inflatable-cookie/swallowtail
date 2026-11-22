use crate::control_wait::{Controlled, wait_controlled_optional};
use crate::process_close::{AppServerTask, CloseBudget};
use std::collections::BTreeSet;

/// Upper bound on `model/list` pages, so a server that keeps issuing cursors
/// cannot keep one discovery call running indefinitely.
const MAXIMUM_MODEL_CATALOG_PAGES: usize = 64;

/// Caller bounds for the `initialize` handshake run while a connection starts.
pub(crate) struct InitBound<'a> {
    /// Whether the handshake requests experimental API access.
    pub(crate) experimental_api: bool,
    /// Cancellation that aborts the handshake, when the caller has one.
    pub(crate) cancellation: Option<&'a swallowtail_runtime::ImmediateCancellation>,
    /// Deadline that bounds the handshake.
    pub(crate) deadline: Option<swallowtail_runtime::Deadline>,
    /// Code and message reported when the deadline ends the handshake.
    pub(crate) timed_out: (&'static str, &'static str),
}

/// Control outcome that ended a connection handshake, so each operation can
/// report it in its own stage and namespace.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum InitControl {
    Cancelled,
    TimedOut,
}

/// Recognises a handshake that `start_connection` ended by cancellation or deadline.
pub(crate) fn initialize_control(error: &RuntimeFailure) -> Option<InitControl> {
    match error.diagnostic().code() {
        "swallowtail.codex.app_server.initialize_cancelled" => Some(InitControl::Cancelled),
        "swallowtail.codex.app_server.initialize_timed_out" => Some(InitControl::TimedOut),
        _ => None,
    }
}

/// Timeout report for a handshake that the caller deadline ends.
pub(crate) const INITIALIZE_TIMED_OUT: (&str, &str) = (
    "swallowtail.codex.app_server.initialize_timed_out",
    "Codex app-server initialization timed out",
);

impl CodexAppServerDriver {
    pub(crate) fn validate_plan(
        &self,
        plan: &PreflightPlan,
    ) -> Result<CodexAppServerBehavior, RuntimeFailure> {
        if plan.driver_identity().id().as_str() != "swallowtail.codex.app-server" {
            return Err(failure(
                "swallowtail.codex.app_server.plan_driver_mismatch",
                "Preflight plan is bound to a different driver",
            ));
        }
        let behavior = classify_app_server_plan(plan)?;
        validate_app_server_plan(plan, behavior)?;
        Ok(behavior)
    }

    pub(crate) async fn start_connection(
        &self,
        plan: &PreflightPlan,
        behavior: CodexAppServerBehavior,
        scope: ScopeId,
        working_resource: Option<WorkingResourceRef>,
        services: &HostServices,
        bound: InitBound<'_>,
    ) -> Result<(Arc<RpcConnection>, AppServerTask), RuntimeFailure> {
        let (connection, task) = self
            .spawn_connection(plan, behavior, scope, working_resource, services)
            .await?;
        let deadline = match bound.deadline {
            Some(deadline) => Some(
                services
                    .time()
                    .ok_or_else(|| {
                        failure(
                            "swallowtail.codex.app_server.time_service_missing",
                            "Codex app-server initialization deadline requires a time service",
                        )
                    })?
                    .wait_until(deadline),
            ),
            None => None,
        };
        let initialized = wait_controlled_optional(
            connection.initialize(bound.experimental_api),
            bound.cancellation,
            deadline,
        )
        .await;
        let error = match initialized {
            Controlled::Completed(Ok(())) => return Ok((connection, task)),
            Controlled::Completed(Err(error)) => error,
            Controlled::Cancelled => failure(
                "swallowtail.codex.app_server.initialize_cancelled",
                "Codex app-server initialization was cancelled",
            ),
            Controlled::Deadline => failure(bound.timed_out.0, bound.timed_out.1),
        };
        let _ = connection.cancel_session().await;
        let _ = task.join().await;
        Err(error)
    }

    pub(crate) async fn spawn_connection(
        &self,
        plan: &PreflightPlan,
        behavior: CodexAppServerBehavior,
        scope: ScopeId,
        working_resource: Option<WorkingResourceRef>,
        services: &HostServices,
    ) -> Result<(Arc<RpcConnection>, AppServerTask), RuntimeFailure> {
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
            .with_arguments(behavior.invocation())
            .with_environment([self.environment.clone()]);
        if let Some(resource) = working_resource {
            process_request = process_request.with_working_resource(resource);
        }
        let process: Arc<dyn ProcessHandle> = Arc::from(
            process_service
                .start(scope.clone(), process_request)
                .await?,
        );
        let connection = RpcConnection::new(Arc::clone(&process), services.clone());
        let pump_connection = Arc::clone(&connection);
        let task = match AppServerTask::spawn(
            task_service, scope, Box::pin(async move { pump_connection.pump().await }),
        )
        {
            Ok(task) => task,
            Err(error) => {
                let _ = process.force_stop().await;
                let _ = process.wait().await;
                return Err(error);
            }
        };
        Ok((connection, task))
    }

    async fn read_catalog(
        &self,
        connection: &RpcConnection,
    ) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
        collect_model_pages(|cursor| async move {
            connection
                .request(
                    "model/list",
                    serde_json::json!({"cursor": cursor, "includeHidden": false}),
                )
                .await
        })
        .await
    }
}

/// Walks `model/list` pages through `fetch_page`, bounding both the page count
/// and the cursor chain: a page count over [`MAXIMUM_MODEL_CATALOG_PAGES`] or a
/// cursor the server has already issued is a malformed catalogue.
async fn collect_model_pages<F, Fut>(mut fetch_page: F) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure>
where
    F: FnMut(Option<String>) -> Fut,
    Fut: std::future::Future<Output = Result<Value, RuntimeFailure>>,
{
    let mut models = Vec::new();
    let mut cursor: Option<String> = None;
    let mut issued = BTreeSet::new();
    for _ in 0..MAXIMUM_MODEL_CATALOG_PAGES {
        let response = fetch_page(cursor.clone()).await?;
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
        let Some(next) = response
            .get("nextCursor")
            .and_then(Value::as_str)
            .map(str::to_owned)
        else {
            return Ok(models);
        };
        if !issued.insert(next.clone()) {
            return Err(failure(
                "swallowtail.codex.app_server.catalog_cursor_repeated",
                "Codex app-server model catalogue repeated a page cursor",
            ));
        }
        cursor = Some(next);
    }
    Err(failure(
        "swallowtail.codex.app_server.catalog_page_limit",
        "Codex app-server model catalogue exceeded its page limit",
    ))
}

fn validate_attachment_binding(
    plan: &PreflightPlan,
    binding: &swallowtail_runtime::SessionResumeBinding,
    working_resource: &WorkingResourceRef,
    access_policy: &swallowtail_core::SessionAccessPolicy,
) -> Result<(), RuntimeFailure> {
    if binding.matches_attachment(plan, working_resource, access_policy) {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.codex.app_server.resume_binding_mismatch",
            "Codex app-server resume binding does not match the preflight plan",
        ))
    }
}

fn require_continuity_capabilities(
    plan: &PreflightPlan,
    capability: Capability,
) -> Result<(), RuntimeFailure> {
    let requirement = plan
        .requirements()
        .capabilities()
        .find(|requirement| requirement.capability() == capability)
        .ok_or_else(|| {
            failure(
                "swallowtail.codex.app_server.continuity_capability_mismatch",
                "Codex app-server continuity capability does not match its preflight plan",
            )
        })?;
    if capability == Capability::LoadSession
        && (!requirement.constraints().any(|constraint| {
            constraint == &CapabilityConstraint::ReplayMaximumItems(MAXIMUM_REPLAY_ITEMS as u32)
        }) || !requirement.constraints().any(|constraint| {
            constraint == &CapabilityConstraint::ReplayMaximumBytes(MAXIMUM_REPLAY_BYTES as u64)
        }))
    {
        return Err(failure(
            "swallowtail.codex.app_server.continuity_capability_mismatch",
            "Codex app-server continuity bounds do not match its preflight plan",
        ));
    }
    Ok(())
}

fn supports_exclude_turns(plan: &PreflightPlan) -> Result<bool, RuntimeFailure> {
    let binding = crate::cli_version::bound_codex_cli_version(plan).ok_or_else(|| {
        failure(
            "swallowtail.codex.app_server.version_missing",
            "Codex app-server continuity requires an exact executable version",
        )
    })?;
    let version = semver::Version::parse(binding.version().as_str()).map_err(|_| {
        failure(
            "swallowtail.codex.app_server.version_malformed",
            "Codex app-server continuity version is malformed",
        )
    })?;
    Ok(version >= semver::Version::new(0, 129, 0))
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

pub(crate) async fn close_connection(
    connection: &Arc<RpcConnection>,
    task: AppServerTask,
    budget: CloseBudget,
) -> CleanupOutcome {
    let close = connection.close_input().await;
    let join = task
        .join_or_escalate_with_abort(connection.process(), budget, || connection.abort_pump())
        .await;
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
        format!("Codex app-server driver does not support {feature}"),
    )
}

pub(crate) fn scope(kind: &str, request_id: &RequestId) -> ScopeId {
    ScopeId::new(format!("codex-app-server:{kind}:{}", request_id.as_str()))
        .expect("request id produces a valid scope id")
}
