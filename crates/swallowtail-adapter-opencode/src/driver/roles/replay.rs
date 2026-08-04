impl OpenCodeHttpDriver {
    async fn load_replay(
        &self,
        scope: ScopeId,
        source: ReplaySource<'_>,
        deadline: Option<Deadline>,
        services: &HostServices,
        cancelled: Arc<AtomicBool>,
        reconciliation_cancellation: Option<&swallowtail_runtime::ImmediateCancellation>,
    ) -> Result<Vec<swallowtail_runtime::SessionReplayItem>, RuntimeFailure> {
        let mut pages = Vec::new();
        let mut before = None;
        let mut cursors = BTreeSet::new();
        let mut bytes = 0usize;
        loop {
            if pages.len() >= CONTINUITY_MAXIMUM_PAGES {
                return Err(continuity_limit());
            }
            let message_request = session_messages(
                source.session.as_provider_value(),
                source.directory,
                CONTINUITY_PAGE_LIMIT,
                before.as_deref(),
            );
            let response = match reconciliation_cancellation {
                Some(cancellation) => {
                    reconciliation_request(
                        &self.transport,
                        scope.clone(),
                        source.endpoint.to_owned(),
                        message_request,
                        ReconciliationControl::new(
                            services,
                            cancellation,
                            deadline,
                            Arc::clone(&cancelled),
                        ),
                    )
                    .await?
                }
                None => {
                    complete_before_deadline(
                        self.transport.request(
                            scope.clone(),
                            source.endpoint.to_owned(),
                            message_request,
                            services,
                            Arc::clone(&cancelled),
                        ),
                        deadline,
                        services,
                        Arc::clone(&cancelled),
                        "swallowtail.opencode.session_load_timed_out",
                        "OpenCode session load timed out",
                    )
                    .await?
                }
            };
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
    let active_turn_detachment = active_turn_detachment(plan)?;
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
        active_turn_detachment,
        callback_run_id: None,
    }))
}

fn continuity_limit() -> RuntimeFailure {
    failure(
        "swallowtail.opencode.replay_limit_exceeded",
        "OpenCode session history exceeded the adapter limit",
    )
}
