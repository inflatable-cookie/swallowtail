enum TurnSignal {
    Data(Vec<u8>),
    Failure(RuntimeFailure),
    Closed,
    Deadline,
}

async fn next_signal(
    subscription: &mut Subscription,
    deadline: &mut Option<BoxFuture<'static, DeadlineObservation>>,
) -> TurnSignal {
    poll_fn(|context| {
        if let Poll::Ready(item) = subscription.poll_next(context) {
            return Poll::Ready(match item {
                Some(Ok(data)) => TurnSignal::Data(data),
                Some(Err(error)) => TurnSignal::Failure(error),
                None => TurnSignal::Closed,
            });
        }
        if let Some(deadline) = deadline
            && deadline.as_mut().poll(context).is_ready()
        {
            return Poll::Ready(TurnSignal::Deadline);
        }
        Poll::Pending
    })
    .await
}

fn validate_open(
    plan: &PreflightPlan,
    request: &OpenSessionRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    require_services(services, true)?;
    swallowtail_runtime::validate_session_plan_agreement(plan, request.plan_agreement())?;
    let callbacks = provider_callbacks(plan)?;
    let expected_access = if callbacks {
        SessionAccessPolicy::ambient_harness_with_consumer_mediated_requests(
            ResourceAccess::ReadWrite,
            [
                callback::permission_namespace(),
                callback::question_namespace(),
            ],
        )
    } else {
        SessionAccessPolicy::ambient_harness(ResourceAccess::Read)
    };
    if request.access_policy() != &expected_access {
        return Err(unsupported("non-ambient read session access"));
    }
    validate_attachment_plan(plan, services)?;
    if request.working_resource().is_none() {
        return Err(unsupported("a resource-free session"));
    }
    if !request.options().is_empty() {
        return Err(unsupported("non-default session options"));
    }
    validate_deadline(request.deadline(), services)
}

pub(super) struct AttachmentValidation<'a> {
    binding: &'a SessionResumeBinding,
    working_resource: &'a swallowtail_runtime::WorkingResourceRef,
    access_policy: &'a SessionAccessPolicy,
    deadline: Option<Deadline>,
    options: &'a swallowtail_runtime::SessionOptions,
    agreement: &'a swallowtail_runtime::SessionPlanAgreement,
}

impl<'a> AttachmentValidation<'a> {
    pub(super) fn new(
        binding: &'a SessionResumeBinding,
        working_resource: &'a swallowtail_runtime::WorkingResourceRef,
        access_policy: &'a SessionAccessPolicy,
        deadline: Option<Deadline>,
        options: &'a swallowtail_runtime::SessionOptions,
        agreement: &'a swallowtail_runtime::SessionPlanAgreement,
    ) -> Self {
        Self {
            binding,
            working_resource,
            access_policy,
            deadline,
            options,
            agreement,
        }
    }
}

fn validate_attachment_request(
    plan: &PreflightPlan,
    request: AttachmentValidation<'_>,
    services: &HostServices,
) -> Result<(bool, bool), RuntimeFailure> {
    require_services(services, true)?;
    swallowtail_runtime::validate_session_plan_agreement(plan, request.agreement)?;
    if !request
        .binding
        .matches_attachment(plan, request.working_resource, request.access_policy)
    {
        return Err(failure(
            "swallowtail.opencode.session_binding_mismatch",
            "OpenCode session binding does not match the requested attachment",
        ));
    }
    let callbacks = provider_callbacks(plan)?;
    let expected_access = if callbacks {
        SessionAccessPolicy::ambient_harness_with_consumer_mediated_requests(
            ResourceAccess::ReadWrite,
            [
                callback::permission_namespace(),
                callback::question_namespace(),
            ],
        )
    } else {
        SessionAccessPolicy::ambient_harness(ResourceAccess::Read)
    };
    if request.access_policy != &expected_access {
        return Err(unsupported("non-ambient attachment access"));
    }
    if !request.options.is_empty() {
        return Err(unsupported("non-default attachment options"));
    }
    validate_deadline(request.deadline, services)?;
    Ok((callbacks, validate_attachment_plan(plan, services)?))
}

fn validate_turn(
    request: &TurnRequest,
    services: &HostServices,
    image_attachments: bool,
) -> Result<(), RuntimeFailure> {
    validate_attachments(request.attachments(), services, image_attachments)?;
    if request.structured_output().is_some() {
        return Err(unsupported("structured turn output"));
    }
    validate_deadline(request.deadline(), services)
}

fn provider_callbacks(plan: &PreflightPlan) -> Result<bool, RuntimeFailure> {
    let namespaces = plan
        .requirements()
        .extension_namespaces()
        .collect::<Vec<_>>();
    match namespaces.as_slice() {
        [] => Ok(false),
        [permission, question]
            if **permission == callback::permission_namespace()
                && **question == callback::question_namespace() =>
        {
            Ok(true)
        }
        _ => Err(failure(
            "swallowtail.opencode.callback_plan_mismatch",
            "OpenCode callback namespaces do not match the immutable plan",
        )),
    }
}

fn validate_attachment_plan(
    plan: &PreflightPlan,
    services: &HostServices,
) -> Result<bool, RuntimeFailure> {
    let Some(requirement) = plan
        .requirements()
        .capabilities()
        .find(|required| required.capability() == Capability::Attachments)
    else {
        return Ok(false);
    };
    for constraint in [
        CapabilityConstraint::attachment_media_type("image/png")
            .expect("static media type is valid"),
        CapabilityConstraint::AttachmentMaximumBytes(1024 * 1024),
        CapabilityConstraint::AttachmentMaximumCount(1),
    ] {
        if !requirement
            .constraints()
            .any(|required| required == &constraint)
        {
            return Err(failure(
                "swallowtail.opencode.attachment_plan_mismatch",
                "OpenCode attachment constraints do not match the immutable plan",
            ));
        }
    }
    if requirement.constraints().count() != 3 {
        return Err(failure(
            "swallowtail.opencode.attachment_plan_mismatch",
            "OpenCode attachment constraints do not match the immutable plan",
        ));
    }
    if !plan
        .requirements()
        .host_services()
        .any(|service| service == HostServiceKind::Attachment)
        || services.attachment().is_none()
    {
        return Err(failure(
            "swallowtail.opencode.attachment_service_missing",
            "OpenCode attachment input requires its preflight-bound host service",
        ));
    }
    Ok(true)
}

fn validate_attachments<'a>(
    attachments: impl ExactSizeIterator<Item = &'a swallowtail_runtime::AttachmentDescriptor>,
    services: &HostServices,
    enabled: bool,
) -> Result<(), RuntimeFailure> {
    if (attachments.len() != 0) && !enabled {
        return Err(failure(
            "swallowtail.opencode.attachment_plan_mismatch",
            "OpenCode turn attachment was not authorized by its session plan",
        ));
    }
    if attachments.len() > 1 {
        return Err(unsupported("more than one turn attachment"));
    }
    for attachment in attachments {
        if attachment.media_type() != "image/png"
            || attachment.role() != swallowtail_runtime::AttachmentRole::Input
            || attachment
                .known_length()
                .is_some_and(|length| length > 1024 * 1024)
        {
            return Err(unsupported("non-PNG or oversized turn attachment"));
        }
    }
    if enabled && services.attachment().is_none() {
        return Err(failure(
            "swallowtail.opencode.attachment_service_missing",
            "OpenCode attachment input requires an attachment service",
        ));
    }
    Ok(())
}

fn validate_deadline(
    deadline: Option<Deadline>,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    if let Some(deadline) = deadline {
        let time = services.time().ok_or_else(|| {
            failure(
                "swallowtail.opencode.time_service_missing",
                "OpenCode deadline requires a time service",
            )
        })?;
        if time.now() >= deadline.instant() {
            return Err(failure(
                "swallowtail.opencode.deadline_elapsed",
                "OpenCode deadline elapsed before provider work",
            ));
        }
    }
    Ok(())
}

fn require_services(services: &HostServices, session: bool) -> Result<(), RuntimeFailure> {
    if services.blocking_work().is_none()
        || services.time().is_none()
        || services.network().is_none()
        || services.credential().is_none()
        || (session && (services.task().is_none() || services.working_resource().is_none()))
    {
        Err(failure(
            "swallowtail.opencode.host_services_missing",
            "OpenCode HTTP required host services are unavailable",
        ))
    } else {
        Ok(())
    }
}

async fn complete_before_deadline<T, F>(
    work: F,
    deadline: Option<Deadline>,
    services: &HostServices,
    cancelled: Arc<AtomicBool>,
    timeout_code: &'static str,
    timeout_message: &'static str,
) -> Result<T, RuntimeFailure>
where
    F: Future<Output = Result<T, RuntimeFailure>>,
{
    let Some(deadline) = deadline else {
        return work.await;
    };
    let time = services.time().ok_or_else(|| {
        failure(
            "swallowtail.opencode.time_service_missing",
            "OpenCode deadline requires a time service",
        )
    })?;
    let mut work = Box::pin(work);
    let mut wait = time.wait_until(deadline);
    let mut timed_out = false;
    let result = poll_fn(|context| {
        if let Poll::Ready(result) = work.as_mut().poll(context) {
            return Poll::Ready(result);
        }
        if !timed_out && wait.as_mut().poll(context).is_ready() {
            timed_out = true;
            cancelled.store(true, Ordering::SeqCst);
            context.waker().wake_by_ref();
        }
        Poll::Pending
    })
    .await;
    if timed_out {
        Err(failure(timeout_code, timeout_message))
    } else {
        result
    }
}

async fn reap_finished(active: &ActiveSlot) -> Result<(), RuntimeFailure> {
    let finished = {
        let mut state = active.lock().expect("active turn lock poisoned");
        if state
            .as_ref()
            .is_some_and(|turn| turn.terminal.load(Ordering::SeqCst))
        {
            state.take()
        } else {
            None
        }
    };
    if let Some(mut turn) = finished {
        if let Some(task) = turn.task.take() {
            task.join().await?;
        }
        if matches!(turn.attachment.release().await, CleanupOutcome::Failed(_)) {
            return Err(failure(
                "swallowtail.opencode.attachment_cleanup_failed",
                "OpenCode attachment cleanup failed",
            ));
        }
    }
    Ok(())
}

async fn join_active(active: &ActiveSlot) -> CleanupOutcome {
    let turn = active.lock().expect("active turn lock poisoned").take();
    match turn {
        Some(mut turn) => {
            let task = match turn.task.take() {
                Some(task) => cleanup_from_result(task.join().await),
                None => CleanupOutcome::NotApplicable,
            };
            merge_cleanup(task, turn.attachment.release().await)
        }
        None => CleanupOutcome::NotApplicable,
    }
}

async fn close_active(active: &ActiveSlot) -> CleanupOutcome {
    let cancellation = active
        .lock()
        .expect("active turn lock poisoned")
        .as_ref()
        .filter(|turn| !turn.terminal.load(Ordering::SeqCst))
        .map(|turn| Arc::clone(&turn.cancellation));
    let cancel_cleanup = match cancellation {
        Some(cancellation) => cleanup_from_result(cancellation.request().await.map(|_| ())),
        None => CleanupOutcome::NotApplicable,
    };
    merge_cleanup(cancel_cleanup, join_active(active).await)
}

fn cleanup_from_result(result: Result<(), RuntimeFailure>) -> CleanupOutcome {
    match result {
        Ok(()) => CleanupOutcome::Clean,
        Err(error) => CleanupOutcome::Failed(error.diagnostic().clone()),
    }
}

fn merge_cleanup(current: CleanupOutcome, next: CleanupOutcome) -> CleanupOutcome {
    match (&current, &next) {
        (CleanupOutcome::Failed(_), _) => current,
        (_, CleanupOutcome::Failed(_)) => next,
        (CleanupOutcome::Degraded(_), _) => current,
        (_, CleanupOutcome::Degraded(_)) => next,
        (CleanupOutcome::Clean, _) => current,
        (CleanupOutcome::NotApplicable, CleanupOutcome::Clean) => next,
        _ => current,
    }
}

fn scope(kind: &str, id: &str) -> Result<ScopeId, RuntimeFailure> {
    ScopeId::new(format!("opencode-http:{kind}:{id}")).map_err(|_| {
        failure(
            "swallowtail.opencode.scope_invalid",
            "OpenCode operation scope was invalid",
        )
    })
}
