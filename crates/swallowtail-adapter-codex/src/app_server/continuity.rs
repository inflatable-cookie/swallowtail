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

    async fn start_connection(
        &self,
        plan: &PreflightPlan,
        behavior: CodexAppServerBehavior,
        scope: ScopeId,
        working_resource: Option<WorkingResourceRef>,
        experimental_api: bool,
        services: &HostServices,
    ) -> Result<(Arc<RpcConnection>, Box<dyn JoinedTask>), RuntimeFailure> {
        let (connection, task) = self
            .spawn_connection(plan, behavior, scope, working_resource, services)
            .await?;
        if let Err(error) = connection.initialize(experimental_api).await {
            let _ = connection.cancel_session().await;
            let _ = task.join().await;
            return Err(error);
        }
        Ok((connection, task))
    }

    pub(crate) async fn spawn_connection(
        &self,
        plan: &PreflightPlan,
        behavior: CodexAppServerBehavior,
        scope: ScopeId,
        working_resource: Option<WorkingResourceRef>,
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
    let binding = plan.interface_versions().next().ok_or_else(|| {
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

pub(crate) fn scope(kind: &str, request_id: &RequestId) -> ScopeId {
    ScopeId::new(format!("codex-app-server:{kind}:{}", request_id.as_str()))
        .expect("request id produces a valid scope id")
}
