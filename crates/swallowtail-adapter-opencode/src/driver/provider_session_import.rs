impl ProviderSessionCatalogueDriver for OpenCodeHttpDriver {
    fn list_provider_sessions(
        &self,
        plan: ProviderSessionCataloguePlan,
        request: ProviderSessionCatalogueRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionCatalogueOutcome, ProviderSessionOperationFailure>> {
        Box::pin(async move {
            validate_provider_session_catalogue_execution(&plan, &request, &services)?;
            self.execute_provider_session_catalogue(plan, request, services).await
        })
    }
}

impl ProviderSessionImportDriver for OpenCodeHttpDriver {
    fn import_provider_session(
        &self,
        plan: ProviderSessionImportPlan,
        request: ProviderSessionImportRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionImportOutcome, ProviderSessionOperationFailure>> {
        Box::pin(async move {
            validate_provider_session_import_execution(&plan, &request, &services)?;
            self.execute_provider_session_import(plan, request, services).await
        })
    }
}

impl OpenCodeHttpDriver {
    async fn execute_provider_session_catalogue(
        &self,
        plan: ProviderSessionCataloguePlan,
        request: ProviderSessionCatalogueRequest,
        services: HostServices,
    ) -> Result<ProviderSessionCatalogueOutcome, ProviderSessionOperationFailure> {
        let version = qualified_plan(plan.preflight()).map_err(before_dispatch)?;
        let scope = scope("session-catalogue", request.request_id().as_str()).map_err(before_dispatch)?;
        let policy = SessionAccessPolicy::ambient_harness(ResourceAccess::Read);
        let mut access = AccessLeases::acquire(
            plan.preflight(), scope.clone(), &services,
            Some((plan.agreement().scope().working_resource_ref(), &policy)),
        ).await.map_err(catalogue_dispatch)?;
        let result = async {
            let directory = access.directory.as_deref().ok_or_else(|| catalogue_projection(failure(
                "swallowtail.opencode.session_catalogue.resource_invalid",
                "OpenCode session catalogue requires a filesystem working resource",
            )))?;
            let health = controlled_request(
                &self.transport, scope.clone(), access.endpoint.clone(), Request::get("/global/health"),
                &services, request.cancellation(), request.agreement().deadline(),
            ).await?;
            require_health_matches(&health, &version).map_err(catalogue_dispatch)?;
            let statuses = controlled_request(
                &self.transport, scope.clone(), access.endpoint.clone(), session_status(directory),
                &services, request.cancellation(), request.agreement().deadline(),
            ).await.and_then(|response| parse_session_statuses(&response).map_err(catalogue_projection))?;
            let start = parse_cursor(request.cursor().map(|cursor| cursor.as_provider_value()))?;
            let limit = plan.agreement().bounds().maximum_page_size().get();
            let response = controlled_request(
                &self.transport, scope, access.endpoint.clone(), session_list(directory, start, limit),
                &services, request.cancellation(), request.agreement().deadline(),
            ).await?;
            let observations = parse_session_list(&response).map_err(catalogue_projection)?;
            project_opencode_page(&plan, observations, &statuses, start, limit, directory, version.binding().version().as_str())
        }.await;
        let cleanup = access.release(&services).await;
        let (candidates, cursor) = finish(result, cleanup.clone())?;
        ProviderSessionCatalogueOutcome::new(&plan, &request, candidates, cursor, cleanup)
    }

    async fn execute_provider_session_import(
        &self,
        plan: ProviderSessionImportPlan,
        request: ProviderSessionImportRequest,
        services: HostServices,
    ) -> Result<ProviderSessionImportOutcome, ProviderSessionOperationFailure> {
        let version = qualified_plan(plan.preflight()).map_err(before_dispatch)?;
        let scope = scope("session-import", request.request_id().as_str()).map_err(before_dispatch)?;
        let policy = SessionAccessPolicy::ambient_harness(ResourceAccess::Read);
        let mut access = AccessLeases::acquire(
            plan.preflight(), scope.clone(), &services,
            Some((plan.agreement().working_resource(), &policy)),
        ).await.map_err(import_revalidation)?;
        let result = async {
            let directory = access.directory.as_deref().ok_or_else(|| import_revalidation(failure(
                "swallowtail.opencode.session_import.resource_invalid",
                "OpenCode session import requires a filesystem working resource",
            )))?;
            let health = controlled_request(
                &self.transport, scope.clone(), access.endpoint.clone(), Request::get("/global/health"),
                &services, request.cancellation(), request.agreement().deadline(),
            ).await?;
            require_health_matches(&health, &version).map_err(import_revalidation)?;
            let lookup = controlled_request(
                &self.transport, scope.clone(), access.endpoint.clone(), session_get(request.provider_session_ref().as_provider_value(), directory),
                &services, request.cancellation(), request.agreement().deadline(),
            ).await?;
            let observed = parse_session_lookup(&lookup).map_err(import_revalidation)?;
            let statuses = controlled_request(
                &self.transport, scope, access.endpoint.clone(), session_status(directory),
                &services, request.cancellation(), request.agreement().deadline(),
            ).await.and_then(|response| parse_session_statuses(&response).map_err(import_revalidation))?;
            revalidate_opencode_candidate(&plan, &request, observed, &statuses, directory, version.binding().version().as_str())
        }.await;
        let cleanup = access.release(&services).await;
        let revalidation = finish(result, cleanup.clone())?;
        ProviderSessionImportOutcome::new(&plan, &request, revalidation, cleanup)
    }
}

fn qualified_plan(plan: &swallowtail_core::PreflightPlan) -> Result<OpenCodePlanVersion, RuntimeFailure> {
    let selected = OpenCodeHttpDriver::validate_plan(plan)?;
    if !matches!(selected.assessment(), InterfaceCompatibilityAssessment::Qualified(_)) {
        return Err(failure(
            "swallowtail.opencode.session_catalogue.version_unsupported",
            "OpenCode session catalogue and import require a qualified server version",
        ));
    }
    Ok(selected)
}

fn project_opencode_page(
    plan: &ProviderSessionCataloguePlan,
    observations: Vec<OpenCodeSessionObservation>,
    statuses: &std::collections::BTreeMap<String, OpenCodeSessionStatus>,
    start: u32,
    limit: u32,
    directory: &str,
    expected_version: &str,
) -> Result<(Vec<ProviderSessionCandidate>, Option<String>), ProviderSessionOperationFailure> {
    if observations.len() > limit as usize { return Err(catalogue_limit()); }
    let raw_len = observations.len();
    let mut candidates = Vec::with_capacity(raw_len);
    for (index, observation) in observations.into_iter().enumerate() {
        if observation.directory != directory { continue; }
        let ordinal = start.checked_add(u32::try_from(index).map_err(|_| catalogue_limit())?).ok_or_else(catalogue_limit)?;
        let candidate_id = ProviderSessionCandidateId::new(format!("opencode-session-candidate-{ordinal}")).map_err(|_| catalogue_malformed())?;
        let provider_ref = SessionRef::new(observation.id.clone()).map_err(|_| catalogue_malformed())?;
        let display = ProviderSessionDisplayContent::new(Some(observation.title), None).map_err(|_| catalogue_malformed())?;
        let status = statuses.get(&observation.id).copied().unwrap_or(OpenCodeSessionStatus::Unavailable);
        let activity = match status { OpenCodeSessionStatus::Idle => ProviderSessionActivityState::Inactive, OpenCodeSessionStatus::Active => ProviderSessionActivityState::Active, OpenCodeSessionStatus::Unavailable => ProviderSessionActivityState::Unknown };
        let availability = if observation.version != expected_version {
            ProviderSessionImportAvailability::Unavailable(ProviderSessionImportUnavailableReason::IncompatibleInterface)
        } else if observation.archived {
            ProviderSessionImportAvailability::Unavailable(ProviderSessionImportUnavailableReason::Archived)
        } else if observation.parent {
            ProviderSessionImportAvailability::Unavailable(ProviderSessionImportUnavailableReason::ProviderReportedUnavailable)
        } else { match status {
            OpenCodeSessionStatus::Idle => ProviderSessionImportAvailability::Available,
            OpenCodeSessionStatus::Active => ProviderSessionImportAvailability::Unavailable(ProviderSessionImportUnavailableReason::Active),
            OpenCodeSessionStatus::Unavailable => ProviderSessionImportAvailability::Unavailable(ProviderSessionImportUnavailableReason::ProviderReportedUnavailable),
        }};
        candidates.push(ProviderSessionCandidate::new(plan, candidate_id, provider_ref, display, Some(observation.updated_at), activity, availability).map_err(catalogue_projection)?);
    }
    let cursor = (raw_len == limit as usize && !candidates.is_empty()).then(|| start.saturating_add(limit).to_string());
    Ok((candidates, cursor))
}

fn revalidate_opencode_candidate(
    plan: &ProviderSessionImportPlan,
    request: &ProviderSessionImportRequest,
    observed: OpenCodeSessionObservation,
    statuses: &std::collections::BTreeMap<String, OpenCodeSessionStatus>,
    directory: &str,
    expected_version: &str,
) -> Result<ProviderSessionImportRevalidation, ProviderSessionOperationFailure> {
    let candidate = plan.agreement().candidate();
    if observed.id != request.provider_session_ref().as_provider_value()
        || observed.directory != directory
        || observed.title.as_str() != candidate.display().title().unwrap_or_default()
        || Some(observed.updated_at) != candidate.updated_at_unix_milliseconds()
        || observed.version != expected_version || observed.parent || observed.archived
        || statuses.get(&observed.id) != Some(&OpenCodeSessionStatus::Idle)
    {
        return Err(operation_failure(ProviderSessionOperationFailureStage::ImportRevalidation, "swallowtail.opencode.session_import.candidate_changed", "OpenCode session changed after catalogue observation"));
    }
    Ok(ProviderSessionImportRevalidation::new(
        candidate.candidate_id().clone(), request.provider_session_ref().clone(), plan.agreement().working_resource().clone(),
        ProviderSessionActivityState::Inactive, ProviderSessionImportAvailability::Available,
    ))
}

fn parse_cursor(value: Option<&str>) -> Result<u32, ProviderSessionOperationFailure> {
    value.map_or(Ok(0), |value| value.parse().map_err(|_| catalogue_malformed()))
}

async fn controlled_request(
    transport: &CurlTransport, scope: ScopeId, endpoint: String, request: Request,
    services: &HostServices, cancellation: &swallowtail_runtime::ImmediateCancellation,
    deadline: Option<swallowtail_runtime::Deadline>,
) -> Result<Response, ProviderSessionOperationFailure> {
    if cancellation.is_requested() { return Err(cancelled()); }
    if deadline.is_some_and(|deadline| services.time().is_some_and(|time| time.now() >= deadline.instant())) { return Err(timed_out()); }
    let cancelled_flag = Arc::new(AtomicBool::new(false));
    let mut work = Box::pin(transport.request(scope, endpoint, request, services, Arc::clone(&cancelled_flag)));
    let mut cancellation_wait = cancellation.wait_requested();
    let mut deadline_wait = deadline.map(|deadline| services.time().expect("validated time service").wait_until(deadline));
    let mut control = None;
    let result = std::future::poll_fn(|context| {
        if let Poll::Ready(result) = work.as_mut().poll(context) { return Poll::Ready(result); }
        if control.is_none() && cancellation_wait.as_mut().poll(context).is_ready() {
            control = Some(ProviderSessionOperationFailureStage::Cancelled);
            cancelled_flag.store(true, Ordering::SeqCst);
            context.waker().wake_by_ref();
        }
        if control.is_none() && deadline_wait.as_mut().is_some_and(|wait| wait.as_mut().poll(context).is_ready()) {
            control = Some(ProviderSessionOperationFailureStage::TimedOut);
            cancelled_flag.store(true, Ordering::SeqCst);
            context.waker().wake_by_ref();
        }
        Poll::Pending
    }).await;
    match control {
        Some(ProviderSessionOperationFailureStage::Cancelled) => Err(cancelled()),
        Some(ProviderSessionOperationFailureStage::TimedOut) => Err(timed_out()),
        _ => result.map_err(catalogue_dispatch),
    }
}

fn finish<T>(result: Result<T, ProviderSessionOperationFailure>, cleanup: CleanupOutcome) -> Result<T, ProviderSessionOperationFailure> {
    match cleanup {
        CleanupOutcome::Clean | CleanupOutcome::NotApplicable => result,
        CleanupOutcome::Degraded(diagnostic) | CleanupOutcome::Failed(diagnostic) => Err(ProviderSessionOperationFailure::new(ProviderSessionOperationFailureStage::Cleanup, diagnostic)),
    }
}

fn before_dispatch(error: RuntimeFailure) -> ProviderSessionOperationFailure { from_runtime(ProviderSessionOperationFailureStage::BeforeDispatch, error) }
fn catalogue_dispatch(error: RuntimeFailure) -> ProviderSessionOperationFailure { from_runtime(ProviderSessionOperationFailureStage::CatalogueDispatch, error) }
fn catalogue_projection(error: RuntimeFailure) -> ProviderSessionOperationFailure { from_runtime(ProviderSessionOperationFailureStage::CatalogueProjection, error) }
fn import_revalidation(error: RuntimeFailure) -> ProviderSessionOperationFailure { from_runtime(ProviderSessionOperationFailureStage::ImportRevalidation, error) }
fn from_runtime(stage: ProviderSessionOperationFailureStage, error: RuntimeFailure) -> ProviderSessionOperationFailure { ProviderSessionOperationFailure::new(stage, error.diagnostic().clone()) }
fn operation_failure(stage: ProviderSessionOperationFailureStage, code: &'static str, message: &'static str) -> ProviderSessionOperationFailure { ProviderSessionOperationFailure::new(stage, SafeDiagnostic::new(code, message)) }
fn catalogue_malformed() -> ProviderSessionOperationFailure { operation_failure(ProviderSessionOperationFailureStage::CatalogueProjection, "swallowtail.opencode.session_catalogue.invalid_response", "OpenCode returned malformed session catalogue evidence") }
fn catalogue_limit() -> ProviderSessionOperationFailure { operation_failure(ProviderSessionOperationFailureStage::CatalogueProjection, "swallowtail.opencode.session_catalogue.limit_exceeded", "OpenCode session catalogue exceeded its planned bound") }
fn cancelled() -> ProviderSessionOperationFailure { operation_failure(ProviderSessionOperationFailureStage::Cancelled, "swallowtail.opencode.provider_session.cancelled", "OpenCode provider-session operation was cancelled") }
fn timed_out() -> ProviderSessionOperationFailure { operation_failure(ProviderSessionOperationFailureStage::TimedOut, "swallowtail.opencode.provider_session.timed_out", "OpenCode provider-session operation timed out") }
