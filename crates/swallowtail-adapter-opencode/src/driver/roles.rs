impl ModelCatalogDriver for OpenCodeHttpDriver {
    fn list_models(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        Box::pin(async move {
            Self::validate_plan(&plan)?;
            services.require_execution_host(plan.execution_host_id())?;
            require_services(&services, false)?;
            let scope = scope("catalog", request.request_id().as_str())?;
            let mut access = AccessLeases::acquire(&plan, scope.clone(), &services, None).await?;
            let cancelled = Arc::new(AtomicBool::new(false));
            let result = async {
                let health = complete_before_deadline(
                    self.transport.request(
                        scope.clone(),
                        access.endpoint.clone(),
                        Request::get("/global/health"),
                        &services,
                        Arc::clone(&cancelled),
                    ),
                    request.deadline(),
                    &services,
                    Arc::clone(&cancelled),
                    "swallowtail.opencode.catalog_timed_out",
                    "OpenCode model discovery timed out",
                )
                .await?;
                parse_health(&health)?;
                let response = complete_before_deadline(
                    self.transport.request(
                        scope,
                        access.endpoint.clone(),
                        Request::get("/provider"),
                        &services,
                        Arc::clone(&cancelled),
                    ),
                    request.deadline(),
                    &services,
                    cancelled,
                    "swallowtail.opencode.catalog_timed_out",
                    "OpenCode model discovery timed out",
                )
                .await?;
                parse_catalog(&response)
            }
            .await;
            let cleanup = access.release(&services).await;
            match (result, cleanup) {
                (Ok(models), CleanupOutcome::Clean | CleanupOutcome::NotApplicable) => Ok(models),
                (Err(error), _) => Err(error),
                (Ok(_), _) => Err(failure(
                    "swallowtail.opencode.catalog_cleanup_failed",
                    "OpenCode catalogue credential cleanup failed",
                )),
            }
        })
    }
}

impl InteractiveSessionDriver for OpenCodeHttpDriver {
    fn open_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            Self::validate_plan(&plan)?;
            services.require_execution_host(plan.execution_host_id())?;
            validate_open(&plan, &request, &services)?;
            let provider_id = plan.provider_id().cloned().ok_or_else(|| {
                failure(
                    "swallowtail.opencode.provider_missing",
                    "OpenCode session requires a preflight-bound provider",
                )
            })?;
            let model_id = plan.model_id().cloned().ok_or_else(|| {
                failure(
                    "swallowtail.opencode.model_missing",
                    "OpenCode session requires a preflight-bound model",
                )
            })?;
            let model_route_id = plan.model_route_id().cloned().ok_or_else(|| {
                failure(
                    "swallowtail.opencode.model_route_missing",
                    "OpenCode session requires a preflight-bound model route",
                )
            })?;
            let scope = scope("session", request.request_id().as_str())?;
            let mut access = AccessLeases::acquire(
                &plan,
                scope.clone(),
                &services,
                Some((
                    request.working_resource().expect("validated resource"),
                    request.access_policy(),
                )),
            )
            .await?;
            let directory = access
                .directory
                .clone()
                .expect("session resource was acquired");
            let cancelled = Arc::new(AtomicBool::new(false));
            let open = async {
                let health = complete_before_deadline(
                    self.transport.request(
                        scope.clone(),
                        access.endpoint.clone(),
                        Request::get("/global/health"),
                        &services,
                        Arc::clone(&cancelled),
                    ),
                    request.deadline(),
                    &services,
                    Arc::clone(&cancelled),
                    "swallowtail.opencode.session_open_timed_out",
                    "OpenCode session open timed out",
                )
                .await?;
                parse_health(&health)?;
                let response = complete_before_deadline(
                    self.transport.request(
                        scope,
                        access.endpoint.clone(),
                        session_create(provider_id.as_str(), model_id.as_str(), &directory),
                        &services,
                        Arc::clone(&cancelled),
                    ),
                    request.deadline(),
                    &services,
                    cancelled,
                    "swallowtail.opencode.session_open_timed_out",
                    "OpenCode session open timed out",
                )
                .await?;
                parse_session(&response)
            }
            .await;
            let provider_session_id = match open {
                Ok(id) => id,
                Err(error) => {
                    let _ = access.release(&services).await;
                    return Err(error);
                }
            };
            let provider_ref = SessionRef::new(&provider_session_id).map_err(|_| {
                failure(
                    "swallowtail.opencode.session_invalid",
                    "OpenCode returned an invalid session identity",
                )
            })?;
            let resume_binding = SessionResumeBinding::new(
                provider_ref,
                plan.instance_id().clone(),
                plan.execution_host_id().clone(),
                model_route_id,
                model_id.clone(),
                request
                    .working_resource()
                    .expect("validated resource")
                    .clone(),
                request.access_policy().clone(),
            );
            let runtime_id =
                RuntimeSessionId::new(format!("opencode:{}", request.request_id().as_str()))
                    .map_err(|_| {
                        failure(
                            "swallowtail.opencode.session_invalid",
                            "OpenCode runtime session identity was invalid",
                        )
                    })?;
            let active = Arc::new(Mutex::new(None));
            let cancellation = SessionCancellation::new(Arc::clone(&active));
            Ok(Box::new(OpenCodeSessionHandle {
                request_id: request.request_id().clone(),
                runtime_id,
                resume_binding,
                provider_id,
                model_id,
                provider_session_id,
                directory,
                endpoint: access.endpoint.clone(),
                services,
                transport: self.transport.clone(),
                access: Some(access),
                active,
                cancellation,
            }) as Box<dyn InteractiveSessionHandle>)
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
