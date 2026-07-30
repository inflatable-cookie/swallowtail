fn validate_attachments<'a>(
    plan: &PreflightPlan,
    attachments: impl ExactSizeIterator<Item = &'a swallowtail_runtime::AttachmentDescriptor>,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    let count = attachments.len();
    let planned = plan
        .requirements()
        .capabilities()
        .find(|requirement| requirement.capability() == Capability::Attachments);
    if (count != 0) != planned.is_some() {
        return Err(plan_mismatch("attachments"));
    }
    if count == 0 {
        return Ok(());
    }
    let requirement = planned.expect("attachment request is planned");
    if count > 1
        || !requirement
            .constraints()
            .any(|constraint| matches!(constraint, CapabilityConstraint::AttachmentMaximumCount(1)))
    {
        return Err(plan_mismatch("attachment count"));
    }
    for attachment in attachments {
        if attachment.media_type() != "image/png"
            || !requirement.constraints().any(|constraint| {
                matches!(constraint, CapabilityConstraint::AttachmentMediaType(media) if media == "image/png")
            })
        {
            return Err(unsupported("non-PNG attachment"));
        }
        if attachment
            .known_length()
            .is_some_and(|length| length > 1024 * 1024)
        {
            return Err(unsupported("attachment larger than one MiB"));
        }
    }
    for service in [HostServiceKind::Attachment, HostServiceKind::BlockingWork] {
        if !plan
            .requirements()
            .host_services()
            .any(|required| required == service)
        {
            return Err(plan_mismatch("attachment host service"));
        }
    }
    if services.attachment().is_none() || services.blocking_work().is_none() {
        return Err(plan_mismatch("attachment host service"));
    }
    Ok(())
}


fn validate_planned_attachment_services(
    plan: &PreflightPlan,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    if !plan
        .requirements()
        .capabilities()
        .any(|required| required.capability() == Capability::Attachments)
    {
        return Ok(());
    }
    let attachment = plan
        .requirements()
        .capabilities()
        .find(|required| required.capability() == Capability::Attachments)
        .expect("attachment capability was found");
    for constraint in [
        CapabilityConstraint::attachment_media_type("image/png")
            .expect("static media type is valid"),
        CapabilityConstraint::AttachmentMaximumBytes(1024 * 1024),
        CapabilityConstraint::AttachmentMaximumCount(1),
    ] {
        if !attachment
            .constraints()
            .any(|required| required == &constraint)
        {
            return Err(plan_mismatch("attachment constraint"));
        }
    }
    for service in [HostServiceKind::Attachment, HostServiceKind::BlockingWork] {
        if !plan
            .requirements()
            .host_services()
            .any(|required| required == service)
        {
            return Err(plan_mismatch("attachment host service"));
        }
    }
    if services.attachment().is_none() || services.blocking_work().is_none() {
        return Err(plan_mismatch("attachment host service"));
    }
    Ok(())
}

