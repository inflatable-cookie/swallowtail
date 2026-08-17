use super::transport::{WebApiTransport, require_loopback_endpoint};
use super::{
    DEEPSEEK_HARNESS_WEB_RELEASE_AXIS, DEEPSEEK_HARNESS_WEB_RELEASE_VERSION, WebMethod,
    parse_archive, parse_cancel, parse_fork, parse_history, parse_host_description, parse_models,
    parse_prompt, parse_search, parse_session_create, parse_session_list, request_body,
};
use serde_json::{Value, json};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, CancellationScope, Capability,
    CapabilityConstraint, DiscoveryOutcome, DiscoveryStatus, DriverDescriptor, DriverRole,
    ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation, HostServiceKind,
    InstalledExecutableObservation, InstanceOwnership, IntegrationFamilyId, InterfaceVersion,
    InterfaceVersionAxis, InterfaceVersionBinding, OperationShape, PreflightPlan,
    ProviderSessionActivityState, ProviderSessionAffectedScope, ProviderSessionImportAvailability,
    ProviderSessionImportUnavailableReason, ProviderSessionManagementAction,
    ProviderSessionManagementEffect, ResourceAccess, ResourceRepresentation, SafeDiagnostic,
    SessionRef, SupportAuthority, TransportFamilyId,
};
use swallowtail_runtime::{
    ArchiveProviderSessionRequest, BoxEventStream, BoxFuture, CancellationAcknowledgement,
    CancellationControl, CleanupOutcome, DeleteProviderSessionRequest, DiscoveryDriver,
    DiscoveryRequest, EndpointRef, EnvironmentRef, ExecutableRef, HostServices,
    ImmediateCancellation, InstalledExecutableDiscoveryRequest, JoinedTask, OperationContent,
    ProcessHandle, ProcessRequest, ProviderSessionCandidate, ProviderSessionCandidateId,
    ProviderSessionCatalogueDriver, ProviderSessionCatalogueOutcome, ProviderSessionCataloguePlan,
    ProviderSessionCatalogueRequest, ProviderSessionHistoryDriver, ProviderSessionHistoryPage,
    ProviderSessionHistoryPlan, ProviderSessionHistoryRequest, ProviderSessionHistoryTotal,
    ProviderSessionManagementDriver, ProviderSessionManagementOutcome,
    ProviderSessionManagementPlan, ProviderSessionOperationFailure,
    ProviderSessionOperationFailureStage, RequestId, RestoreProviderSessionRequest, RunHandle,
    RuntimeEvent, RuntimeEventKind, RuntimeFailure, RuntimeRunId, ScopeId, SessionReplayItem,
    StructuredRunDriver, StructuredRunRequest, TerminalOutcome, TerminalStatus,
    page_provider_session_history_window, runtime_event_channel, terminal_outcome_channel,
    validate_installed_executable_discovery_services,
    validate_provider_session_catalogue_execution, validate_provider_session_history_execution,
    validate_provider_session_management_request,
};

/// Returns the DeepSeek Harness Web `/api` descriptor.
#[must_use]
pub fn deepseek_harness_web_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(super::WEB_DRIVER_ID)
                .expect("static DeepSeek Harness Web driver id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("deepseek-harness")
            .expect("static DeepSeek Harness family id is valid"),
        TransportFamilyId::new("deepseek-harness-local-server-http-ws-v1")
            .expect("static DeepSeek Harness Web transport id is valid"),
    )
    .with_roles([
        DriverRole::Discovery,
        DriverRole::StructuredRun,
        DriverRole::ProviderSessionCatalogue,
        DriverRole::ProviderSessionHistory,
        DriverRole::ProviderSessionManagement,
    ])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([
        OperationShape::StructuredRun,
        OperationShape::ProviderSessionCatalogue,
        OperationShape::ProviderSessionHistory,
        OperationShape::ProviderSessionManagement,
    ])
    .with_required_host_services(
        DriverRole::Discovery,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::Time,
        ],
    )
    .with_required_host_services(
        DriverRole::StructuredRun,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::Time,
            HostServiceKind::BlockingWork,
            HostServiceKind::Network,
            HostServiceKind::WorkingResource,
        ],
    )
    .with_required_host_services(
        DriverRole::ProviderSessionCatalogue,
        [
            HostServiceKind::Task,
            HostServiceKind::BlockingWork,
            HostServiceKind::Time,
            HostServiceKind::Network,
            HostServiceKind::WorkingResource,
        ],
    )
    .with_required_host_services(
        DriverRole::ProviderSessionHistory,
        [
            HostServiceKind::BlockingWork,
            HostServiceKind::Time,
            HostServiceKind::Network,
            HostServiceKind::WorkingResource,
        ],
    )
    .with_required_host_services(
        DriverRole::ProviderSessionManagement,
        [
            HostServiceKind::Task,
            HostServiceKind::BlockingWork,
            HostServiceKind::Time,
            HostServiceKind::Network,
            HostServiceKind::WorkingResource,
        ],
    )
    .with_discovery_actions([swallowtail_core::DiscoveryAction::Probe])
    .with_interface_compatibility(super::web_claim())
}

/// Low-level DeepSeek Harness Web `/api` driver.
pub struct DeepSeekHarnessWebDriver {
    environment: EnvironmentRef,
    endpoint: String,
    transport: WebApiTransport,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// One bounded model option returned by the native Web session model method.
pub struct DeepSeekHarnessWebModel {
    provider: String,
    model: String,
    name: String,
}

impl DeepSeekHarnessWebModel {
    #[must_use]
    /// Returns the provider namespace for this model option.
    pub fn provider(&self) -> &str {
        &self.provider
    }

    #[must_use]
    /// Returns the provider model identifier.
    pub fn model(&self) -> &str {
        &self.model
    }

    #[must_use]
    /// Returns the bounded display name supplied by the Web API.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl DeepSeekHarnessWebDriver {
    /// Creates a driver using the loopback Web API default endpoint.
    pub fn new(environment: EnvironmentRef) -> Self {
        Self {
            environment,
            endpoint: "http://127.0.0.1:3080".to_owned(),
            transport: WebApiTransport,
        }
    }

    /// Returns a driver configured for one explicit loopback Web API endpoint.
    pub fn with_endpoint(
        mut self,
        endpoint: impl Into<String>,
    ) -> Result<Self, swallowtail_runtime::RuntimeFailure> {
        let endpoint = endpoint.into();
        require_loopback_endpoint(&endpoint)?;
        self.endpoint = endpoint;
        Ok(self)
    }

    /// Returns the host-approved Cordis configuration reference.
    pub const fn environment(&self) -> &EnvironmentRef {
        &self.environment
    }

    /// Returns the explicit loopback Web API endpoint.
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Searches provider-owned sessions through the allowlisted Web API.
    pub async fn search_sessions(
        &self,
        plan: &PreflightPlan,
        request_id: &RequestId,
        query: &str,
        services: &HostServices,
        deadline: Option<swallowtail_runtime::Deadline>,
    ) -> Result<Vec<(SessionRef, String)>, RuntimeFailure> {
        validate_web_route(plan)?;
        require_web_services(services, false)?;
        if query.is_empty()
            || query.trim() != query
            || query.len() > 500
            || query.chars().any(char::is_control)
        {
            return Err(malformed("DeepSeek Harness Web search query is invalid"));
        }
        let scope = operation_scope("search", request_id)?;
        let endpoint = self
            .authorize_endpoint(plan, scope.clone(), services)
            .await?;
        let value = call_web(
            &self.transport,
            scope,
            endpoint,
            WebMethod::SessionSearch,
            json!({ "query": query }),
            request_id,
            services,
            deadline,
            Arc::new(AtomicBool::new(false)),
        )
        .await?;
        let (items, has_more) = parse_search(&value)?;
        if has_more {
            return Err(malformed(
                "DeepSeek Harness Web search returned an unpageable result",
            ));
        }
        items
            .into_iter()
            .map(|item| {
                Ok((
                    SessionRef::new(item.session_id).map_err(|_| {
                        malformed("DeepSeek Harness Web search session identity is invalid")
                    })?,
                    item.snippet,
                ))
            })
            .collect()
    }

    /// Lists models available to one provider-owned session.
    pub async fn list_session_models(
        &self,
        plan: &PreflightPlan,
        request_id: &RequestId,
        session: &SessionRef,
        services: &HostServices,
        deadline: Option<swallowtail_runtime::Deadline>,
    ) -> Result<Vec<DeepSeekHarnessWebModel>, RuntimeFailure> {
        validate_web_route(plan)?;
        require_web_services(services, false)?;
        let scope = operation_scope("models", request_id)?;
        let endpoint = self
            .authorize_endpoint(plan, scope.clone(), services)
            .await?;
        let value = call_web(
            &self.transport,
            scope,
            endpoint,
            WebMethod::SessionModels,
            json!({ "sessionId": session.as_provider_value() }),
            request_id,
            services,
            deadline,
            Arc::new(AtomicBool::new(false)),
        )
        .await?;
        parse_models(&value).map(|models| {
            models
                .into_iter()
                .map(|model| DeepSeekHarnessWebModel {
                    provider: model.provider,
                    model: model.model,
                    name: model.name,
                })
                .collect()
        })
    }

    /// Forks one provider-owned session through the native Web API method.
    pub async fn fork_session(
        &self,
        plan: &PreflightPlan,
        request_id: &RequestId,
        session: &SessionRef,
        at_sequence: Option<u64>,
        services: &HostServices,
        deadline: Option<swallowtail_runtime::Deadline>,
    ) -> Result<SessionRef, RuntimeFailure> {
        validate_web_route(plan)?;
        require_web_services(services, false)?;
        let scope = operation_scope("fork", request_id)?;
        let endpoint = self
            .authorize_endpoint(plan, scope.clone(), services)
            .await?;
        let mut payload = json!({ "sessionId": session.as_provider_value() });
        if let Some(at_sequence) = at_sequence {
            payload["atSeq"] = Value::from(at_sequence);
        }
        let value = call_web(
            &self.transport,
            scope,
            endpoint,
            WebMethod::SessionFork,
            payload,
            request_id,
            services,
            deadline,
            Arc::new(AtomicBool::new(false)),
        )
        .await?;
        let forked = parse_fork(&value)?;
        SessionRef::new(forked)
            .map_err(|_| malformed("DeepSeek Harness Web fork session identity is invalid"))
    }
}

impl DiscoveryDriver for DeepSeekHarnessWebDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(failure(
                "swallowtail.deepseek_harness.web.discovery_target_required",
                "DeepSeek Harness Web discovery requires one exact host-approved dsh target",
            ))
        })
    }

    fn discover_installed_executable(
        &self,
        request: InstalledExecutableDiscoveryRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<DiscoveryOutcome, RuntimeFailure>> {
        Box::pin(async move {
            validate_installed_executable_discovery_services(&request, &services)?;
            let claim = super::web_claim();
            if request.target().version_axis() != claim.axis() {
                return Err(failure(
                    "swallowtail.deepseek_harness.web.discovery_axis_mismatch",
                    "DeepSeek Harness Web discovery target uses a different version axis",
                ));
            }
            if !super::target_is_exact(request.target().executable().as_host_value()) {
                return Err(failure(
                    "swallowtail.deepseek_harness.web.target_not_pinned",
                    "DeepSeek Harness Web discovery requires the exact dsh CLI target",
                ));
            }
            if request.cancellation().is_requested() {
                return Ok(DiscoveryOutcome::new(
                    DiscoveryStatus::Cancelled,
                    Some(SafeDiagnostic::new(
                        "swallowtail.deepseek_harness.web.discovery_cancelled",
                        "DeepSeek Harness Web installed discovery was cancelled",
                    )),
                ));
            }
            let binding = InterfaceVersionBinding::new(
                InterfaceVersionAxis::new(DEEPSEEK_HARNESS_WEB_RELEASE_AXIS)
                    .expect("static Web version axis is valid"),
                InterfaceVersion::new(DEEPSEEK_HARNESS_WEB_RELEASE_VERSION)
                    .expect("static Web version is valid"),
            );
            let observation = InstalledExecutableObservation::classify(
                request.execution_host_id().clone(),
                binding,
                &claim,
            )
            .map_err(|_| {
                failure(
                    "swallowtail.deepseek_harness.web.discovery_classification_failed",
                    "DeepSeek Harness Web version observation could not be classified",
                )
            })?;
            Ok(DiscoveryOutcome::installed_executable(observation))
        })
    }
}

impl ProviderSessionCatalogueDriver for DeepSeekHarnessWebDriver {
    fn list_provider_sessions(
        &self,
        plan: ProviderSessionCataloguePlan,
        request: ProviderSessionCatalogueRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionCatalogueOutcome, ProviderSessionOperationFailure>>
    {
        Box::pin(async move {
            validate_provider_session_catalogue_execution(&plan, &request, &services)?;
            validate_web_plan(
                plan.preflight(),
                DriverRole::ProviderSessionCatalogue,
                OperationShape::ProviderSessionCatalogue,
            )
            .map_err(|error| {
                operation_failure(ProviderSessionOperationFailureStage::BeforeDispatch, error)
            })?;
            require_web_services(&services, false).map_err(|error| {
                operation_failure(ProviderSessionOperationFailureStage::BeforeDispatch, error)
            })?;
            if let Some(error) = control_before(
                request.cancellation().as_ref(),
                request.agreement().deadline(),
                &services,
            )
            .map_err(|error| {
                operation_failure(ProviderSessionOperationFailureStage::BeforeDispatch, error)
            })? {
                return Err(operation_failure(
                    if request.cancellation().is_requested() {
                        ProviderSessionOperationFailureStage::Cancelled
                    } else {
                        ProviderSessionOperationFailureStage::TimedOut
                    },
                    error,
                ));
            }
            let scope = operation_scope("catalogue", request.request_id()).map_err(|error| {
                operation_failure(ProviderSessionOperationFailureStage::BeforeDispatch, error)
            })?;
            let endpoint = self
                .authorize_endpoint(plan.preflight(), scope.clone(), &services)
                .await
                .map_err(|error| {
                    operation_failure(
                        ProviderSessionOperationFailureStage::CatalogueDispatch,
                        error,
                    )
                })?;
            let cancelled = Arc::new(AtomicBool::new(false));
            let payload = request.cursor().map_or_else(
                || json!({}),
                |cursor| json!({ "cursor": cursor.as_provider_value() }),
            );
            let value = self
                .call(
                    scope,
                    endpoint,
                    WebMethod::SessionList,
                    payload,
                    request.request_id(),
                    &services,
                    request.agreement().deadline(),
                    Arc::clone(&cancelled),
                )
                .await
                .map_err(|error| {
                    operation_failure(
                        ProviderSessionOperationFailureStage::CatalogueDispatch,
                        error,
                    )
                })?;
            let page = parse_session_list(&value).map_err(|error| {
                operation_failure(
                    ProviderSessionOperationFailureStage::CatalogueProjection,
                    error,
                )
            })?;
            let summaries = page.items;
            let working_resource = request
                .agreement()
                .scope()
                .working_resource_ref()
                .as_host_value();
            let first_ordinal = request
                .cursor()
                .map_or(0, |cursor| cursor.observed_candidates());
            let mut candidates = Vec::new();
            for summary in &summaries {
                if summary.cwd.as_deref() != Some(working_resource) {
                    continue;
                }
                let ordinal = first_ordinal
                    .checked_add(u32::try_from(candidates.len()).map_err(|_| {
                        operation_failure(
                            ProviderSessionOperationFailureStage::CatalogueProjection,
                            malformed("DeepSeek Harness Web catalogue ordinal is invalid"),
                        )
                    })?)
                    .ok_or_else(|| {
                        operation_failure(
                            ProviderSessionOperationFailureStage::CatalogueProjection,
                            malformed("DeepSeek Harness Web catalogue ordinal overflowed"),
                        )
                    })?;
                let candidate_id = ProviderSessionCandidateId::new(format!(
                    "deepseek-harness-web-session-{ordinal}"
                ))
                .map_err(|_| {
                    operation_failure(
                        ProviderSessionOperationFailureStage::CatalogueProjection,
                        malformed("DeepSeek Harness Web candidate identity is invalid"),
                    )
                })?;
                let provider_session_ref =
                    SessionRef::new(summary.session_id.clone()).map_err(|_| {
                        operation_failure(
                            ProviderSessionOperationFailureStage::CatalogueProjection,
                            malformed("DeepSeek Harness Web session identity is invalid"),
                        )
                    })?;
                let availability = if summary.running {
                    ProviderSessionImportAvailability::Unavailable(
                        ProviderSessionImportUnavailableReason::Active,
                    )
                } else {
                    ProviderSessionImportAvailability::Available
                };
                let candidate = ProviderSessionCandidate::new(
                    &plan,
                    candidate_id,
                    provider_session_ref,
                    swallowtail_core::ProviderSessionDisplayContent::empty(),
                    Some(summary.updated_at),
                    if summary.running {
                        ProviderSessionActivityState::Active
                    } else {
                        ProviderSessionActivityState::Inactive
                    },
                    availability,
                )
                .map_err(|error| {
                    operation_failure(
                        ProviderSessionOperationFailureStage::CatalogueProjection,
                        RuntimeFailure::new(error.diagnostic().clone()),
                    )
                })?;
                candidates.push(candidate);
            }
            if candidates.len() > plan.agreement().bounds().maximum_page_size().get() as usize {
                return Err(operation_failure(
                    ProviderSessionOperationFailureStage::CatalogueProjection,
                    malformed("DeepSeek Harness Web catalogue page exceeds its bound"),
                ));
            }
            if candidates.is_empty() && page.next_cursor.is_some() {
                return Err(operation_failure(
                    ProviderSessionOperationFailureStage::CatalogueProjection,
                    malformed("DeepSeek Harness Web catalogue page has no in-scope candidates"),
                ));
            }
            ProviderSessionCatalogueOutcome::new(
                &plan,
                &request,
                candidates,
                page.next_cursor,
                CleanupOutcome::NotApplicable,
            )
        })
    }
}

impl ProviderSessionHistoryDriver for DeepSeekHarnessWebDriver {
    fn page_provider_session_history(
        &self,
        plan: ProviderSessionHistoryPlan,
        request: ProviderSessionHistoryRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionHistoryPage, RuntimeFailure>> {
        Box::pin(async move {
            validate_provider_session_history_execution(&plan, &request, &services)?;
            validate_web_plan(
                plan.preflight(),
                DriverRole::ProviderSessionHistory,
                OperationShape::ProviderSessionHistory,
            )?;
            require_web_services(&services, false)?;
            if let Some(error) = control_before(
                request.cancellation().as_ref(),
                request.agreement().deadline(),
                &services,
            )? {
                return Err(error);
            }
            let scope = operation_scope("history", request.request_id())?;
            let endpoint = self
                .authorize_endpoint(plan.preflight(), scope.clone(), &services)
                .await?;
            let session = plan
                .agreement()
                .binding()
                .provider_session_ref()
                .as_provider_value()
                .to_owned();
            let cancelled = Arc::new(AtomicBool::new(false));
            let mut all_records = Vec::new();
            let mut before_seq = None;
            loop {
                let mut payload = json!({
                    "sessionId": session,
                    "maxMessages": plan
                        .agreement()
                        .bounds()
                        .maximum_page_items()
                        .get()
                        .min(super::protocol::MAX_HISTORY_ENTRIES as u32),
                });
                if let Some(before) = before_seq {
                    payload["beforeSeq"] = Value::from(before);
                }
                let value = self
                    .call(
                        scope.clone(),
                        endpoint.clone(),
                        WebMethod::SessionHistory,
                        payload,
                        request.request_id(),
                        &services,
                        request.agreement().deadline(),
                        Arc::clone(&cancelled),
                    )
                    .await?;
                let (page, has_more) = parse_history(&value, &session)?;
                if page.is_empty() && has_more {
                    return Err(malformed(
                        "DeepSeek Harness Web history advertised an empty older page",
                    ));
                }
                let first_sequence = page.first().map(|record| record.sequence);
                all_records.extend(page);
                if all_records.len()
                    > plan.agreement().bounds().maximum_snapshot_items().get() as usize
                {
                    return Err(malformed(
                        "DeepSeek Harness Web history exceeds its snapshot bound",
                    ));
                }
                if !has_more {
                    break;
                }
                let first_sequence = first_sequence.ok_or_else(|| {
                    malformed("DeepSeek Harness Web history did not provide an older cursor")
                })?;
                if first_sequence == 0 || before_seq.is_some_and(|value| first_sequence >= value) {
                    return Err(malformed(
                        "DeepSeek Harness Web history cursor did not move older",
                    ));
                }
                before_seq = Some(first_sequence);
            }
            all_records.sort_by_key(|record| record.sequence);
            let mut replay = Vec::with_capacity(all_records.len());
            let mut previous = None;
            let provider_session_ref = SessionRef::new(session).map_err(|_| {
                malformed("DeepSeek Harness Web history session identity is invalid")
            })?;
            for record in all_records {
                if previous.is_some_and(|value| value >= record.sequence) {
                    return Err(malformed(
                        "DeepSeek Harness Web history contains duplicate sequence numbers",
                    ));
                }
                previous = Some(record.sequence);
                replay.push(SessionReplayItem::new(
                    provider_session_ref.clone(),
                    record.sequence,
                    record.kind,
                ));
            }
            let total = u32::try_from(replay.len()).map_err(|_| {
                malformed("DeepSeek Harness Web history total is outside its bound")
            })?;
            let window = page_provider_session_history_window(
                &plan,
                &request,
                replay,
                ProviderSessionHistoryTotal::Exact(total),
            )?;
            ProviderSessionHistoryPage::new(&plan, &request, window, CleanupOutcome::NotApplicable)
        })
    }
}

impl ProviderSessionManagementDriver for DeepSeekHarnessWebDriver {
    fn archive_session(
        &self,
        plan: ProviderSessionManagementPlan,
        request: ArchiveProviderSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        Box::pin(async move {
            validate_provider_session_management_request(&plan, request.agreement(), &services)?;
            validate_web_plan(
                plan.preflight(),
                DriverRole::ProviderSessionManagement,
                OperationShape::ProviderSessionManagement,
            )?;
            require_web_services(&services, false)?;
            let agreement = request.agreement();
            let action = ProviderSessionManagementAction::Archive;
            if agreement.action() != action
                || agreement.affected_scope() != ProviderSessionAffectedScope::TargetOnly
            {
                return Err(malformed(
                    "DeepSeek Harness Web archive agreement is outside its target-only route",
                ));
            }
            if let Some(error) =
                control_before(request.cancellation(), agreement.deadline(), &services)?
            {
                return Ok(ProviderSessionManagementOutcome::new(
                    agreement.binding().clone(),
                    ProviderSessionManagementEffect::failed_before_effect(action),
                )
                .with_diagnostic(error.diagnostic().clone()));
            }
            let scope = operation_scope("archive", request.request_id())?;
            let endpoint = self
                .authorize_endpoint(plan.preflight(), scope.clone(), &services)
                .await?;
            let session = agreement
                .binding()
                .provider_session_ref()
                .as_provider_value()
                .to_owned();
            let value = self
                .call(
                    scope,
                    endpoint,
                    WebMethod::WorkspaceArchiveSession,
                    json!({ "sessionId": session }),
                    request.request_id(),
                    &services,
                    agreement.deadline(),
                    Arc::new(AtomicBool::new(false)),
                )
                .await;
            match value {
                Ok(value) => {
                    parse_archive(
                        &value,
                        agreement
                            .binding()
                            .provider_session_ref()
                            .as_provider_value(),
                    )?;
                    Ok(ProviderSessionManagementOutcome::new(
                        agreement.binding().clone(),
                        ProviderSessionManagementEffect::applied(
                            action,
                            ProviderSessionAffectedScope::TargetOnly,
                        ),
                    ))
                }
                Err(error) => Ok(ProviderSessionManagementOutcome::new(
                    agreement.binding().clone(),
                    ProviderSessionManagementEffect::unconfirmed_after_effect(action),
                )
                .with_diagnostic(error.diagnostic().clone())),
            }
        })
    }

    fn restore_session(
        &self,
        _plan: ProviderSessionManagementPlan,
        _request: RestoreProviderSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        Box::pin(async { Err(crate::failure::unsupported("DeepSeek Harness Web restore")) })
    }

    fn delete_session(
        &self,
        _plan: ProviderSessionManagementPlan,
        _request: DeleteProviderSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        Box::pin(async { Err(crate::failure::unsupported("DeepSeek Harness Web delete")) })
    }
}

impl DeepSeekHarnessWebDriver {
    async fn authorize_endpoint(
        &self,
        plan: &PreflightPlan,
        scope: ScopeId,
        services: &HostServices,
    ) -> Result<String, RuntimeFailure> {
        let network = services.network().ok_or_else(|| {
            failure(
                "swallowtail.deepseek_harness.web.network_service_missing",
                "DeepSeek Harness Web requires a network-policy service",
            )
        })?;
        let endpoint_ref = EndpointRef::from_instance_target(plan.instance_target_ref());
        let audience = plan.endpoint_audience().clone();
        let grant = network
            .authorize(scope.clone(), endpoint_ref.clone(), audience.clone())
            .await?;
        if grant.scope() != &scope
            || grant.endpoint() != &endpoint_ref
            || grant.audience() != &audience
        {
            return Err(failure(
                "swallowtail.deepseek_harness.web.network_grant_mismatch",
                "DeepSeek Harness Web network grant does not match its immutable plan",
            ));
        }
        let endpoint = grant.authorized().as_driver_value().to_owned();
        require_loopback_endpoint(&endpoint)?;
        if self.endpoint != "http://127.0.0.1:3080" && self.endpoint != endpoint {
            return Err(failure(
                "swallowtail.deepseek_harness.web.endpoint_mismatch",
                "DeepSeek Harness Web endpoint differs from its host-authorized endpoint",
            ));
        }
        Ok(endpoint)
    }

    #[allow(clippy::too_many_arguments)]
    async fn call(
        &self,
        scope: ScopeId,
        endpoint: String,
        method: WebMethod,
        payload: Value,
        request_id: &RequestId,
        services: &HostServices,
        deadline: Option<swallowtail_runtime::Deadline>,
        cancelled: Arc<AtomicBool>,
    ) -> Result<Value, RuntimeFailure> {
        let rpc_id = format!(
            "dsh-web-{}-{}",
            method.as_str().replace('.', "-"),
            request_id.as_str()
        );
        let body = request_body(method, &rpc_id, payload)?;
        self.transport
            .post_json(
                scope,
                endpoint,
                format!("/api/{}", method.as_str()),
                body,
                rpc_id,
                services,
                deadline,
                cancelled,
            )
            .await
    }
}

impl StructuredRunDriver for DeepSeekHarnessWebDriver {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        Box::pin(async move { self.start_structured_run(plan, request, services).await })
    }
}

impl DeepSeekHarnessWebDriver {
    async fn start_structured_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> Result<Box<dyn RunHandle>, RuntimeFailure> {
        validate_web_structured(&plan, &request, &services)?;
        let task_service = services.task().cloned().expect("validated task service");
        let process_service = services
            .process()
            .cloned()
            .expect("validated process service");
        let scope = operation_scope("run", request.request_id())?;
        let endpoint = self
            .authorize_endpoint(&plan, scope.clone(), &services)
            .await?;
        let working_resource = request
            .working_resource()
            .cloned()
            .expect("validated working resource");
        let deadline = request.deadline().expect("validated deadline");
        let expected_provider = plan
            .provider_id()
            .expect("validated provider route")
            .as_str()
            .to_owned();
        let expected_model = plan
            .model_id()
            .expect("validated model route")
            .as_str()
            .to_owned();
        let run_id = RuntimeRunId::new(format!(
            "deepseek-harness-web:{}",
            request.request_id().as_str()
        ))
        .map_err(|_| malformed("DeepSeek Harness Web run identity is invalid"))?;
        let process_request = ProcessRequest::new(ExecutableRef::from_instance_target(
            plan.instance_target_ref(),
        ))
        .with_arguments(["web".to_owned()])
        .with_environment([self.environment.clone()])
        .with_working_resource(working_resource.clone());
        let process: Arc<dyn ProcessHandle> = Arc::from(
            process_service
                .start(scope.clone(), process_request)
                .await?,
        );
        let (event_sender, event_stream) = runtime_event_channel(16_384)?;
        if let Err(error) = event_sender.send(RuntimeEvent::new(0, RuntimeEventKind::Started)) {
            cleanup_process(process.as_ref()).await;
            return Err(error);
        }
        let (terminal_sender, terminal_future) = terminal_outcome_channel();
        let cancellation = Arc::new(DeepSeekHarnessWebCancellation::new(
            self.transport.clone(),
            endpoint.clone(),
            scope.clone(),
            services.clone(),
        ));
        let task_cancellation = Arc::clone(&cancellation);
        let task_process = Arc::clone(&process);
        let task_transport = self.transport.clone();
        let task_services = services.clone();
        let task_request_id = request.request_id().clone();
        let task_prompt = request.content().as_str().to_owned();
        let task_cwd = working_resource.as_host_value().to_owned();
        let task_events = event_sender.clone();
        let task_scope = scope.clone();
        let task = task_service.spawn(
            scope,
            Box::pin(async move {
                let outcome = run_web_operation(
                    task_process,
                    task_transport,
                    task_cancellation,
                    task_scope,
                    endpoint,
                    task_request_id,
                    task_cwd,
                    task_prompt,
                    expected_provider,
                    expected_model,
                    deadline,
                    task_services,
                    task_events.clone(),
                )
                .await;
                let _ = terminal_sender.complete(outcome);
                task_events.mark_terminal();
            }),
        );
        let task = match task {
            Ok(task) => task,
            Err(error) => {
                let _ = process.force_stop().await;
                let _ = process.wait().await;
                return Err(error);
            }
        };
        Ok(Box::new(DeepSeekHarnessWebRunHandle::new(
            request.request_id().clone(),
            run_id,
            Box::pin(event_stream),
            Box::pin(terminal_future),
            cancellation,
            task,
        )))
    }
}

fn validate_web_structured(
    plan: &PreflightPlan,
    request: &StructuredRunRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    validate_web_plan(
        plan,
        DriverRole::StructuredRun,
        OperationShape::StructuredRun,
    )?;
    services.require_execution_host(plan.execution_host_id())?;
    require_web_services(services, true)?;
    if plan.ownership() != InstanceOwnership::HostOwnedEphemeral
        || plan.model_id().is_none()
        || plan.model_route_id().is_none()
        || plan.provider_id().is_none()
    {
        return Err(malformed(
            "DeepSeek Harness Web structured plan is missing its model route",
        ));
    }
    swallowtail_runtime::validate_harness_configuration_policy(plan, request.policy())
        .map_err(|_| malformed("DeepSeek Harness Web configuration posture is not admitted"))?;
    swallowtail_runtime::validate_harness_isolation_policy(plan, request.policy())
        .map_err(|_| malformed("DeepSeek Harness Web isolation is not admitted"))?;
    if request.policy().harness_configuration_posture()
        != Some(HarnessConfigurationPosture::Ambient)
        || request.policy().harness_isolation() != Some(HarnessIsolation::AmbientHost)
        || request.policy().provider_execution()
            != swallowtail_runtime::ProviderExecutionPolicy::Attached
        || request.policy().provider_retention()
            != swallowtail_runtime::ProviderRetentionPolicy::Prohibited
        || request.policy().provider_recovery()
            != swallowtail_runtime::ProviderRecoveryPolicy::Prohibited
        || request.policy().stream_reattachment()
            != swallowtail_runtime::StreamReattachmentPolicy::Disabled
        || request.policy().external_network() != swallowtail_runtime::ExternalNetworkPolicy::Denied
        || request.policy().external_search() != swallowtail_runtime::ExternalSearchPolicy::Disabled
        || request.policy().reasoning_mode().is_some()
    {
        return Err(malformed(
            "DeepSeek Harness Web structured policy is outside its fixed boundary",
        ));
    }
    if request.working_resource().is_none()
        || request.deadline().is_none()
        || request.attachments().len() != 0
        || request.tools().len() != 0
        || request.structured_output().is_some()
        || request.maximum_output_tokens().is_some()
    {
        return Err(malformed(
            "DeepSeek Harness Web structured request is outside its fixed boundary",
        ));
    }
    for capability in [
        Capability::StructuredRun,
        Capability::StreamingEvents,
        Capability::ObservableActivity,
        Capability::UsageReporting,
    ] {
        if !plan
            .requirements()
            .capabilities()
            .any(|required| required.capability() == capability)
        {
            return Err(malformed(
                "DeepSeek Harness Web structured plan is missing a required capability",
            ));
        }
    }
    let interruption = plan.requirements().capabilities().any(|required| {
        required.capability() == Capability::Interruption
            && required.constraints().any(|constraint| {
                constraint
                    == &CapabilityConstraint::CancellationScope(CancellationScope::StructuredRun)
            })
    });
    let has_read_resource = plan.requirements().capabilities().any(|required| {
        required.capability() == Capability::WorkingResource
            && required.constraints().any(|constraint| {
                constraint == &CapabilityConstraint::ResourceAccess(ResourceAccess::Read)
            })
    });
    let has_filesystem_resource = plan.requirements().capabilities().any(|required| {
        required.capability() == Capability::WorkingResource
            && required.constraints().any(|constraint| {
                constraint
                    == &CapabilityConstraint::ResourceRepresentation(
                        ResourceRepresentation::Filesystem,
                    )
            })
    });
    if !interruption || !has_read_resource || !has_filesystem_resource {
        return Err(malformed(
            "DeepSeek Harness Web structured plan is missing its bounded resource constraints",
        ));
    }
    Ok(())
}

const MAX_RUN_OUTPUT_BYTES: usize = super::protocol::MAX_HTTP_BODY_BYTES;

#[allow(clippy::too_many_arguments)]
async fn run_web_operation(
    process: Arc<dyn ProcessHandle>,
    transport: WebApiTransport,
    cancellation: Arc<DeepSeekHarnessWebCancellation>,
    scope: ScopeId,
    endpoint: String,
    request_id: RequestId,
    cwd: String,
    prompt: String,
    expected_provider: String,
    expected_model: String,
    deadline: swallowtail_runtime::Deadline,
    services: HostServices,
    events: swallowtail_runtime::RuntimeEventSender,
) -> TerminalOutcome {
    let result = run_web_prompt(
        transport,
        Arc::clone(&cancellation),
        scope,
        endpoint,
        request_id,
        cwd,
        prompt,
        expected_provider,
        expected_model,
        deadline,
        services.clone(),
        events,
    )
    .await;
    let (status, output) = match result {
        Ok(output) => (TerminalStatus::Completed, output),
        Err(_error) if cancellation.is_requested() => (TerminalStatus::Cancelled, None),
        Err(_error)
            if services
                .time()
                .is_some_and(|time| time.now() >= deadline.instant()) =>
        {
            let _ = cancellation.request_native().await;
            (TerminalStatus::TimedOut, None)
        }
        Err(error) => (
            TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
            None,
        ),
    };
    let cleanup = cleanup_process(process.as_ref()).await;
    let mut outcome = TerminalOutcome::new(status, cleanup);
    if matches!(outcome.status(), TerminalStatus::Completed)
        && let Some(output) = output
    {
        outcome = outcome.with_output(output);
    }
    outcome
}

#[allow(clippy::too_many_arguments)]
async fn run_web_prompt(
    transport: WebApiTransport,
    cancellation: Arc<DeepSeekHarnessWebCancellation>,
    scope: ScopeId,
    endpoint: String,
    request_id: RequestId,
    cwd: String,
    prompt: String,
    expected_provider: String,
    expected_model: String,
    deadline: swallowtail_runtime::Deadline,
    services: HostServices,
    events: swallowtail_runtime::RuntimeEventSender,
) -> Result<Option<OperationContent>, RuntimeFailure> {
    let host = call_web(
        &transport,
        scope.clone(),
        endpoint.clone(),
        WebMethod::HostDescribe,
        json!({}),
        &request_id,
        &services,
        Some(deadline),
        Arc::new(AtomicBool::new(false)),
    )
    .await?;
    let host = parse_host_description(&host)?;
    if host.provider.as_deref() != Some(expected_provider.as_str())
        || host.model.as_deref() != Some(expected_model.as_str())
    {
        return Err(malformed(
            "DeepSeek Harness Web host model does not match its prepared route",
        ));
    }
    if cancellation.is_requested() {
        return Err(failure(
            "swallowtail.deepseek_harness.web.cancelled",
            "DeepSeek Harness Web run was cancelled before session creation",
        ));
    }
    let created = call_web(
        &transport,
        scope.clone(),
        endpoint.clone(),
        WebMethod::SessionCreate,
        json!({ "cwd": cwd }),
        &request_id,
        &services,
        Some(deadline),
        Arc::new(AtomicBool::new(false)),
    )
    .await?;
    let session_id = parse_session_create(&created)?.session_id;
    cancellation.set_session(session_id.clone());
    if cancellation.is_requested() {
        return Err(failure(
            "swallowtail.deepseek_harness.web.cancelled",
            "DeepSeek Harness Web run was cancelled before prompt dispatch",
        ));
    }
    let stream_cancelled = Arc::new(AtomicBool::new(false));
    cancellation.set_stream(Arc::clone(&stream_cancelled));
    let mut stream = super::transport::WebSocketSubscription::open(
        scope.clone(),
        endpoint.clone(),
        "/api/events.mux".to_owned(),
        &services,
        Some(deadline),
        stream_cancelled,
    )
    .await?;
    let prompted = call_web(
        &transport,
        scope,
        endpoint,
        WebMethod::SessionPrompt,
        json!({
            "sessionId": session_id,
            "mode": "queue",
            "content": [{ "type": "text", "text": prompt }],
        }),
        &request_id,
        &services,
        Some(deadline),
        Arc::new(AtomicBool::new(false)),
    )
    .await;
    let prompted = match prompted {
        Ok(value) => value,
        Err(error) => {
            let _ = stream.close().await;
            return Err(error);
        }
    };
    if let Err(error) = parse_prompt(&prompted) {
        let _ = stream.close().await;
        return Err(error);
    }
    let loop_result = async {
        let mut subscribed = false;
        let mut previous_sequence = None;
        let mut output = String::new();
        let mut completed = false;
        let mut usage_seen = false;
        let mut live_events = 0usize;
        loop {
            let Some(bytes) = stream.next_controlled(Some(deadline), &services).await? else {
                break;
            };
            match super::decode_mux_frame(&bytes)? {
                super::MuxFrame::Subscribed {
                    session_id: subscribed_session,
                    last_seq,
                } => {
                    if subscribed_session == session_id {
                        if last_seq < -1 {
                            return Err(malformed(
                                "DeepSeek Harness Web subscription sequence is invalid",
                            ));
                        }
                        subscribed = true;
                        events.send(RuntimeEvent::new(1, RuntimeEventKind::Progress))?;
                    }
                }
                super::MuxFrame::Event(event) => {
                    if event.session_id != session_id {
                        continue;
                    }
                    if !subscribed {
                        return Err(malformed(
                            "DeepSeek Harness Web session event preceded its subscription",
                        ));
                    }
                    if previous_sequence.is_some_and(|sequence| sequence >= event.sequence) {
                        return Err(malformed(
                            "DeepSeek Harness Web live event sequence moved backwards",
                        ));
                    }
                    previous_sequence = Some(event.sequence);
                    live_events = live_events.saturating_add(1);
                    if live_events > super::protocol::MAX_LIVE_EVENTS {
                        return Err(malformed(
                            "DeepSeek Harness Web live event stream exceeds its bound",
                        ));
                    }
                    if let Some(usage) = event.usage
                        && !usage_seen
                    {
                        usage_seen = true;
                        events.send(RuntimeEvent::new(
                            event.sequence,
                            RuntimeEventKind::ProviderObservation(
                                swallowtail_runtime::ProviderObservation::Usage(usage),
                            ),
                        ))?;
                    }
                    if let Some(delta) = event.output_delta {
                        if output.len().saturating_add(delta.len()) > MAX_RUN_OUTPUT_BYTES {
                            return Err(malformed(
                                "DeepSeek Harness Web live output exceeds its bound",
                            ));
                        }
                        output.push_str(&delta);
                        let content = OperationContent::new(delta).map_err(|_| {
                            malformed("DeepSeek Harness Web output content is invalid")
                        })?;
                        events.send(RuntimeEvent::with_content(
                            event.sequence,
                            RuntimeEventKind::OutputDelta,
                            content,
                        ))?;
                    } else {
                        events.send(RuntimeEvent::new(
                            event.sequence,
                            RuntimeEventKind::Progress,
                        ))?;
                    }
                    if event.terminal {
                        completed = true;
                        break;
                    }
                }
            }
        }
        if !completed {
            return Err(malformed(
                "DeepSeek Harness Web event stream ended before turn completion",
            ));
        }
        Ok((output, previous_sequence))
    }
    .await;
    let close_result = stream.close().await;
    let (output, previous_sequence) = match (loop_result, close_result) {
        (Err(error), _) => return Err(error),
        (Ok(_), Err(error)) => return Err(error),
        (Ok(value), Ok(())) => value,
    };
    let output = if output.is_empty() {
        None
    } else {
        let content = OperationContent::new(output)
            .map_err(|_| malformed("DeepSeek Harness Web final output is invalid"))?;
        events.send(RuntimeEvent::with_content(
            previous_sequence.unwrap_or(0).saturating_add(1),
            RuntimeEventKind::OutputAvailable,
            content.clone(),
        ))?;
        Some(content)
    };
    Ok(output)
}

#[allow(clippy::too_many_arguments)]
async fn call_web(
    transport: &WebApiTransport,
    scope: ScopeId,
    endpoint: String,
    method: WebMethod,
    payload: Value,
    request_id: &RequestId,
    services: &HostServices,
    deadline: Option<swallowtail_runtime::Deadline>,
    cancelled: Arc<AtomicBool>,
) -> Result<Value, RuntimeFailure> {
    let rpc_id = format!(
        "dsh-web-{}-{}",
        method.as_str().replace('.', "-"),
        request_id.as_str()
    );
    let body = request_body(method, &rpc_id, payload)?;
    transport
        .post_json(
            scope,
            endpoint,
            format!("/api/{}", method.as_str()),
            body,
            rpc_id,
            services,
            deadline,
            cancelled,
        )
        .await
}

async fn cleanup_process(process: &dyn ProcessHandle) -> CleanupOutcome {
    let stopped = process.force_stop().await;
    let waited = process.wait().await;
    if stopped.is_ok() && waited.is_ok() {
        CleanupOutcome::Clean
    } else {
        CleanupOutcome::Failed(SafeDiagnostic::new(
            "swallowtail.deepseek_harness.web.process_cleanup_failed",
            "DeepSeek Harness Web process cleanup could not be joined",
        ))
    }
}

struct DeepSeekHarnessWebCancellation {
    transport: WebApiTransport,
    endpoint: String,
    scope: ScopeId,
    services: HostServices,
    session_id: std::sync::Mutex<Option<String>>,
    stream: std::sync::Mutex<Option<Arc<AtomicBool>>>,
    requested: AtomicBool,
}

impl DeepSeekHarnessWebCancellation {
    fn new(
        transport: WebApiTransport,
        endpoint: String,
        scope: ScopeId,
        services: HostServices,
    ) -> Self {
        Self {
            transport,
            endpoint,
            scope,
            services,
            session_id: std::sync::Mutex::new(None),
            stream: std::sync::Mutex::new(None),
            requested: AtomicBool::new(false),
        }
    }

    fn is_requested(&self) -> bool {
        self.requested.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn set_session(&self, session_id: String) {
        *self.session_id.lock().expect("session lock poisoned") = Some(session_id);
    }

    fn set_stream(&self, stream: Arc<AtomicBool>) {
        *self.stream.lock().expect("stream lock poisoned") = Some(stream);
    }

    async fn request_native(&self) -> Result<(), RuntimeFailure> {
        let session_id = self
            .session_id
            .lock()
            .expect("session lock poisoned")
            .clone();
        let Some(session_id) = session_id else {
            return Ok(());
        };
        let request_id =
            RequestId::new("cancel").map_err(|_| malformed("cancel request id is invalid"))?;
        let value = call_web(
            &self.transport,
            self.scope.clone(),
            self.endpoint.clone(),
            WebMethod::SessionCancel,
            json!({ "sessionId": session_id }),
            &request_id,
            &self.services,
            None,
            Arc::new(AtomicBool::new(false)),
        )
        .await?;
        parse_cancel(&value)
    }
}

impl CancellationControl for DeepSeekHarnessWebCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::StructuredRun
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let already_requested = self
            .requested
            .swap(true, std::sync::atomic::Ordering::SeqCst);
        if let Some(stream) = self.stream.lock().expect("stream lock poisoned").as_ref() {
            stream.store(true, std::sync::atomic::Ordering::SeqCst);
        }
        Box::pin(async move {
            if already_requested {
                return Ok(CancellationAcknowledgement::AlreadyRequested);
            }
            self.request_native().await?;
            Ok(CancellationAcknowledgement::Requested)
        })
    }
}

struct DeepSeekHarnessWebRunHandle {
    request_id: RequestId,
    run_id: RuntimeRunId,
    events: Option<BoxEventStream>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: Arc<DeepSeekHarnessWebCancellation>,
    task: Box<dyn JoinedTask>,
}

impl DeepSeekHarnessWebRunHandle {
    fn new(
        request_id: RequestId,
        run_id: RuntimeRunId,
        events: BoxEventStream,
        terminal: BoxFuture<'static, TerminalOutcome>,
        cancellation: Arc<DeepSeekHarnessWebCancellation>,
        task: Box<dyn JoinedTask>,
    ) -> Self {
        Self {
            request_id,
            run_id,
            events: Some(events),
            terminal: Some(terminal),
            cancellation,
            task,
        }
    }
}

impl RunHandle for DeepSeekHarnessWebRunHandle {
    fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    fn run_id(&self) -> &RuntimeRunId {
        &self.run_id
    }

    fn provider_run_ref(&self) -> Option<&swallowtail_core::RunRef> {
        None
    }

    fn take_events(&mut self) -> Option<BoxEventStream> {
        self.events.take()
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        self.cancellation.as_ref()
    }

    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>> {
        self.terminal.take()
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            match self.task.join().await {
                Ok(()) => CleanupOutcome::Clean,
                Err(_) => CleanupOutcome::Failed(SafeDiagnostic::new(
                    "swallowtail.deepseek_harness.web.task_join_failed",
                    "DeepSeek Harness Web operation task could not be joined",
                )),
            }
        })
    }
}

fn validate_web_plan(
    plan: &PreflightPlan,
    role: DriverRole,
    shape: OperationShape,
) -> Result<(), RuntimeFailure> {
    validate_web_route(plan)?;
    if plan.requirements().driver_role() != role || plan.requirements().operation_shape() != shape {
        return Err(malformed(
            "DeepSeek Harness Web plan is bound to another route",
        ));
    }
    Ok(())
}

fn validate_web_route(plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
    if plan.driver_identity().id().as_str() != super::WEB_DRIVER_ID
        || plan.requirements().execution_layer() != ExecutionLayer::HarnessInteraction
    {
        return Err(malformed(
            "DeepSeek Harness Web plan is bound to another route",
        ));
    }
    super::validate_plan(plan)?;
    if plan.credential_mechanism() != &swallowtail_core::CredentialMechanism::LocalUnauthenticated
        || plan.credential_reference().is_some()
        || plan.endpoint_audience().as_str() != crate::DEEPSEEK_HARNESS_CONFIG_AUDIENCE
        || plan.access_status().support_authority() != SupportAuthority::ProviderSupported
    {
        return Err(malformed(
            "DeepSeek Harness Web plan has an unsupported access boundary",
        ));
    }
    Ok(())
}

fn require_web_services(services: &HostServices, process: bool) -> Result<(), RuntimeFailure> {
    if services.blocking_work().is_none()
        || services.time().is_none()
        || services.network().is_none()
        || services.working_resource().is_none()
        || (process && services.process().is_none())
    {
        return Err(malformed(
            "DeepSeek Harness Web requires its preflight-bound host services",
        ));
    }
    Ok(())
}

fn operation_scope(prefix: &str, request_id: &RequestId) -> Result<ScopeId, RuntimeFailure> {
    ScopeId::new(format!(
        "deepseek-harness-web-{prefix}-{}",
        request_id.as_str()
    ))
    .map_err(|_| malformed("DeepSeek Harness Web operation scope is invalid"))
}

fn control_before(
    cancellation: &ImmediateCancellation,
    deadline: Option<swallowtail_runtime::Deadline>,
    services: &HostServices,
) -> Result<Option<RuntimeFailure>, RuntimeFailure> {
    if cancellation.is_requested() {
        return Ok(Some(failure(
            "swallowtail.deepseek_harness.web.cancelled",
            "DeepSeek Harness Web operation was cancelled before dispatch",
        )));
    }
    if deadline.is_some_and(|deadline| {
        services
            .time()
            .is_some_and(|time| time.now() >= deadline.instant())
    }) {
        return Ok(Some(failure(
            "swallowtail.deepseek_harness.web.timed_out",
            "DeepSeek Harness Web operation reached its deadline before dispatch",
        )));
    }
    Ok(None)
}

fn operation_failure(
    stage: ProviderSessionOperationFailureStage,
    error: RuntimeFailure,
) -> ProviderSessionOperationFailure {
    ProviderSessionOperationFailure::new(stage, error.diagnostic().clone())
}

fn malformed(message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(
        "swallowtail.deepseek_harness.web.projection_invalid",
        message,
    ))
}

fn failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

#[cfg(test)]
mod tests {
    use super::deepseek_harness_web_descriptor;
    use swallowtail_core::{DriverRole, ExecutionLayer, OperationShape};

    #[test]
    fn descriptor_is_one_web_api_harness_route() {
        let descriptor = deepseek_harness_web_descriptor();
        assert_eq!(descriptor.integration_family().as_str(), "deepseek-harness");
        assert_eq!(
            descriptor.transport_family().as_str(),
            "deepseek-harness-local-server-http-ws-v1"
        );
        assert!(descriptor.supports_role(DriverRole::ProviderSessionCatalogue));
        assert!(descriptor.supports_role(DriverRole::ProviderSessionHistory));
        assert!(descriptor.supports_role(DriverRole::ProviderSessionManagement));
        assert!(descriptor.supports_execution_layer(ExecutionLayer::HarnessInteraction));
        assert!(descriptor.supports_operation_shape(OperationShape::StructuredRun));
        assert!(!descriptor.supports_role(DriverRole::InteractiveSession));
    }
}
