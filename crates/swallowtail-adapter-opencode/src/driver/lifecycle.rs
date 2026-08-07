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

fn active_turn_detachment(plan: &PreflightPlan) -> Result<bool, RuntimeFailure> {
    let detachment = plan
        .requirements()
        .capabilities()
        .find(|required| required.capability() == Capability::ActiveOperationDetachment);
    let Some(detachment) = detachment else {
        return Ok(false);
    };
    if !matches!(
        classify_plan(plan)?.assessment(),
        InterfaceCompatibilityAssessment::Qualified(_)
    ) {
        return Err(failure(
            "swallowtail.opencode.detachment_version_unsupported",
            "OpenCode active-turn detachment requires a qualified server version",
        ));
    }
    let constraints = detachment.constraints().collect::<Vec<_>>();
    let durable = plan
        .requirements()
        .capabilities()
        .any(|required| required.capability() == Capability::ProviderDurableRetention);
    if constraints.as_slice()
        != [&CapabilityConstraint::OperationDetachmentScope(
            swallowtail_core::OperationDetachmentScope::ActiveTurn,
        )]
        || !durable
        || plan.requirements().session_provider_state_policy()
            != Some(SessionProviderStatePolicy::DurableProviderSessionPreserved)
        || provider_callbacks(plan)?
    {
        return Err(failure(
            "swallowtail.opencode.detachment_plan_mismatch",
            "OpenCode active-turn detachment does not match the immutable session plan",
        ));
    }
    Ok(true)
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

