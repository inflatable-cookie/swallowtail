impl ModelCatalogDriver for OpenCodeHttpDriver {
    fn list_models(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        Box::pin(async move {
            let version = Self::validate_plan(&plan)?;
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
                require_health_matches(&health, &version)?;
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
            let version = Self::validate_plan(&plan)?;
            services.require_execution_host(plan.execution_host_id())?;
            validate_open(&plan, &request, &services)?;
            let callback_enabled = provider_callbacks(&plan)?;
            let image_attachments = validate_attachment_plan(&plan, &services)?;
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
                require_health_matches(&health, &version)?;
                let response = complete_before_deadline(
                    self.transport.request(
                        scope,
                        access.endpoint.clone(),
                        session_create(
                            provider_id.as_str(),
                            model_id.as_str(),
                            &directory,
                            callback_enabled,
                        ),
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
                parse_session_for_version(&response, version.binding())
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
                reasoning_mode: None,
                structured_output: None,
                image_attachments,
                provider_callbacks: callback_enabled,
                callback_run_id: None,
            }) as Box<dyn InteractiveSessionHandle>)
        })
    }

    fn load_session(
        &self,
        plan: PreflightPlan,
        request: LoadSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<LoadedSession, RuntimeFailure>> {
        Box::pin(async move {
            let version = Self::validate_plan(&plan)?;
            services.require_execution_host(plan.execution_host_id())?;
            require_continuity(&plan, Capability::LoadSession)?;
            let attachment = AttachmentValidation::new(
                request.resume_binding(),
                request.working_resource(),
                request.access_policy(),
                request.deadline(),
                request.options(),
                request.plan_agreement(),
            );
            let (callback_enabled, image_attachments) =
                validate_attachment_request(&plan, attachment, &services)?;
            let scope = scope("load", request.request_id().as_str())?;
            let mut access = AccessLeases::acquire(
                &plan,
                scope.clone(),
                &services,
                Some((request.working_resource(), request.access_policy())),
            )
            .await?;
            let directory = access.directory.clone().expect("resource was acquired");
            let cancelled = Arc::new(AtomicBool::new(false));
            let attached = async {
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
                    "swallowtail.opencode.session_load_timed_out",
                    "OpenCode session load timed out",
                )
                .await?;
                require_health_matches(&health, &version)?;
                let provider_ref = request.provider_session_ref().clone();
                let lookup = complete_before_deadline(
                    self.transport.request(
                        scope.clone(),
                        access.endpoint.clone(),
                        session_get(provider_ref.as_provider_value(), &directory),
                        &services,
                        Arc::clone(&cancelled),
                    ),
                    request.deadline(),
                    &services,
                    Arc::clone(&cancelled),
                    "swallowtail.opencode.session_load_timed_out",
                    "OpenCode session load timed out",
                )
                .await?;
                require_existing_session(
                    &lookup,
                    version.binding(),
                    provider_ref.as_provider_value(),
                )?;
                let replay = self
                    .load_replay(
                        scope,
                        ReplaySource::new(&access.endpoint, &directory, &provider_ref),
                        request.deadline(),
                        &services,
                        cancelled,
                    )
                    .await?;
                Ok((provider_ref, replay))
            }
            .await;
            let (provider_ref, replay) = match attached {
                Ok(attached) => attached,
                Err(error) => {
                    let _ = access.release(&services).await;
                    return Err(error);
                }
            };
            let handle = build_attached_handle(
                &plan,
                request.request_id().clone(),
                request.resume_binding().clone(),
                provider_ref,
                directory,
                services,
                self.transport.clone(),
                access,
                callback_enabled,
                image_attachments,
            )?;
            Ok(LoadedSession::new(replay, handle))
        })
    }

    fn resume_session(
        &self,
        plan: PreflightPlan,
        request: ResumeSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            let version = Self::validate_plan(&plan)?;
            services.require_execution_host(plan.execution_host_id())?;
            require_continuity(&plan, Capability::Resume)?;
            let attachment = AttachmentValidation::new(
                request.resume_binding(),
                request.working_resource(),
                request.access_policy(),
                request.deadline(),
                request.options(),
                request.plan_agreement(),
            );
            let (callback_enabled, image_attachments) =
                validate_attachment_request(&plan, attachment, &services)?;
            let scope = scope("resume", request.request_id().as_str())?;
            let mut access = AccessLeases::acquire(
                &plan,
                scope.clone(),
                &services,
                Some((request.working_resource(), request.access_policy())),
            )
            .await?;
            let directory = access.directory.clone().expect("resource was acquired");
            let cancelled = Arc::new(AtomicBool::new(false));
            let attached = async {
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
                    "swallowtail.opencode.session_resume_timed_out",
                    "OpenCode session resume timed out",
                )
                .await?;
                require_health_matches(&health, &version)?;
                let provider_ref = request.provider_session_ref().clone();
                let lookup = complete_before_deadline(
                    self.transport.request(
                        scope,
                        access.endpoint.clone(),
                        session_get(provider_ref.as_provider_value(), &directory),
                        &services,
                        Arc::clone(&cancelled),
                    ),
                    request.deadline(),
                    &services,
                    cancelled,
                    "swallowtail.opencode.session_resume_timed_out",
                    "OpenCode session resume timed out",
                )
                .await?;
                require_existing_session(
                    &lookup,
                    version.binding(),
                    provider_ref.as_provider_value(),
                )?;
                Ok(provider_ref)
            }
            .await;
            let provider_ref = match attached {
                Ok(provider_ref) => provider_ref,
                Err(error) => {
                    let _ = access.release(&services).await;
                    return Err(error);
                }
            };
            build_attached_handle(
                &plan,
                request.request_id().clone(),
                request.resume_binding().clone(),
                provider_ref,
                directory,
                services,
                self.transport.clone(),
                access,
                callback_enabled,
                image_attachments,
            )
        })
    }
}

impl OpenCodeHttpDriver {
    async fn load_replay(
        &self,
        scope: ScopeId,
        source: ReplaySource<'_>,
        deadline: Option<Deadline>,
        services: &HostServices,
        cancelled: Arc<AtomicBool>,
    ) -> Result<Vec<swallowtail_runtime::SessionReplayItem>, RuntimeFailure> {
        let mut pages = Vec::new();
        let mut before = None;
        let mut cursors = BTreeSet::new();
        let mut bytes = 0usize;
        loop {
            if pages.len() >= CONTINUITY_MAXIMUM_PAGES {
                return Err(continuity_limit());
            }
            let response = complete_before_deadline(
                self.transport.request(
                    scope.clone(),
                    source.endpoint.to_owned(),
                    session_messages(
                        source.session.as_provider_value(),
                        source.directory,
                        CONTINUITY_PAGE_LIMIT,
                        before.as_deref(),
                    ),
                    services,
                    Arc::clone(&cancelled),
                ),
                deadline,
                services,
                Arc::clone(&cancelled),
                "swallowtail.opencode.session_load_timed_out",
                "OpenCode session load timed out",
            )
            .await?;
            bytes = bytes.saturating_add(response.body.len());
            if bytes > CONTINUITY_MAXIMUM_BYTES {
                return Err(continuity_limit());
            }
            let next = response.next_cursor.clone();
            pages.push(response);
            let Some(next) = next else {
                break;
            };
            if !cursors.insert(next.clone()) {
                return Err(failure(
                    "swallowtail.opencode.pagination_cursor_repeated",
                    "OpenCode repeated a session-history pagination cursor",
                ));
            }
            before = Some(next);
        }
        let mut sequence = 0u64;
        let mut replay = Vec::new();
        for page in pages.iter().rev() {
            replay.extend(project_session_messages(
                page,
                source.session,
                &mut sequence,
            )?);
            if replay.len() > CONTINUITY_MAXIMUM_ITEMS {
                return Err(continuity_limit());
            }
        }
        Ok(replay)
    }
}

struct ReplaySource<'a> {
    endpoint: &'a str,
    directory: &'a str,
    session: &'a SessionRef,
}

impl<'a> ReplaySource<'a> {
    fn new(endpoint: &'a str, directory: &'a str, session: &'a SessionRef) -> Self {
        Self {
            endpoint,
            directory,
            session,
        }
    }
}

fn require_continuity(plan: &PreflightPlan, capability: Capability) -> Result<(), RuntimeFailure> {
    let requirement = plan
        .requirements()
        .capabilities()
        .find(|requirement| requirement.capability() == capability)
        .ok_or_else(|| {
            failure(
                "swallowtail.opencode.continuity_capability_mismatch",
                "OpenCode continuity capability does not match its preflight plan",
            )
        })?;
    if capability == Capability::LoadSession
        && (!requirement.constraints().any(|constraint| {
            constraint
                == &CapabilityConstraint::ReplayMaximumItems(CONTINUITY_MAXIMUM_ITEMS as u32)
        }) || !requirement.constraints().any(|constraint| {
            constraint
                == &CapabilityConstraint::ReplayMaximumBytes(CONTINUITY_MAXIMUM_BYTES as u64)
        }))
    {
        return Err(failure(
            "swallowtail.opencode.continuity_capability_mismatch",
            "OpenCode continuity bounds do not match its preflight plan",
        ));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn build_attached_handle(
    plan: &PreflightPlan,
    request_id: RequestId,
    resume_binding: SessionResumeBinding,
    provider_ref: SessionRef,
    directory: String,
    services: HostServices,
    transport: CurlTransport,
    access: AccessLeases,
    provider_callbacks: bool,
    image_attachments: bool,
) -> Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure> {
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
    let runtime_id = RuntimeSessionId::new(format!("opencode:{}", request_id.as_str())).map_err(
        |_| {
            failure(
                "swallowtail.opencode.session_invalid",
                "OpenCode runtime session identity was invalid",
            )
        },
    )?;
    let active = Arc::new(Mutex::new(None));
    let cancellation = SessionCancellation::new(Arc::clone(&active));
    Ok(Box::new(OpenCodeSessionHandle {
        request_id,
        runtime_id,
        resume_binding,
        provider_id,
        model_id,
        provider_session_id: provider_ref.as_provider_value().to_owned(),
        directory,
        endpoint: access.endpoint.clone(),
        services,
        transport,
        access: Some(access),
        active,
        cancellation,
        reasoning_mode: None,
        structured_output: None,
        image_attachments,
        provider_callbacks,
        callback_run_id: None,
    }))
}

fn continuity_limit() -> RuntimeFailure {
    failure(
        "swallowtail.opencode.replay_limit_exceeded",
        "OpenCode session history exceeded the adapter limit",
    )
}
