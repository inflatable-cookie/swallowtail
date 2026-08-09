impl InteractiveSessionDriver for CodexAppServerDriver {
    fn open_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            validate_session_deadline(request.deadline().is_some())?;
            validate_session_plan_agreement(&plan, request.plan_agreement())?;
            let behavior = self.validate_plan(&plan)?;
            validate_workspace_behavior(&behavior, request.access_policy())?;
            let session_input = CodexSessionInput::for_open(&plan, request.options(), &services)?;
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
                    behavior,
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
            let mut params = serde_json::json!({"model": model.as_str()});
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

    fn load_session(
        &self,
        plan: PreflightPlan,
        request: LoadSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<LoadedSession, RuntimeFailure>> {
        Box::pin(async move {
            validate_session_deadline(request.deadline().is_some())?;
            validate_session_plan_agreement(&plan, request.plan_agreement())?;
            require_continuity_capabilities(&plan, Capability::LoadSession)?;
            let behavior = self.validate_plan(&plan)?;
            validate_workspace_behavior(&behavior, request.access_policy())?;
            let session_input = CodexSessionInput::for_resume(&plan, request.options(), &services)?;
            let working_resource = request.working_resource().ok_or_else(|| {
                failure(
                    "swallowtail.codex.app_server.workspace_required",
                    "Codex app-server session requires a filesystem working resource",
                )
            })?;
            validate_attachment_binding(
                &plan,
                request.resume_binding(),
                working_resource,
                request.access_policy(),
            )?;
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
            let scope = scope("load", request.request_id());
            let access = CodexSessionAccess::prepare(
                &plan,
                request.access_policy(),
                working_resource,
                scope.clone(),
                &services,
            )
            .await?;
            let experimental_api =
                session_input.requires_experimental_api() || access.requires_experimental_api();
            let connection = self
                .start_connection(
                    &plan,
                    behavior,
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
            .finish_loaded(
                &plan,
                response,
                request.provider_session_ref().as_provider_value(),
            )
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
            validate_session_plan_agreement(&plan, request.plan_agreement())?;
            let behavior = self.validate_plan(&plan)?;
            validate_workspace_behavior(&behavior, request.access_policy())?;
            let session_input = CodexSessionInput::for_resume(&plan, request.options(), &services)?;
            require_continuity_capabilities(&plan, Capability::Resume)?;
            validate_attachment_binding(
                &plan,
                request.resume_binding(),
                request.working_resource(),
                request.access_policy(),
            )?;
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
            let exclude_turns = supports_exclude_turns(&plan)?;
            let experimental_api = session_input.requires_experimental_api()
                || access.requires_experimental_api()
                || exclude_turns;
            let connection = self
                .start_connection(
                    &plan,
                    behavior,
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
            if exclude_turns {
                params["excludeTurns"] = Value::Bool(true);
            }
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
