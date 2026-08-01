use serde_json::Value;
use swallowtail_core::{
    ProviderSessionActivityState, ProviderSessionDisplayContent, ProviderSessionImportAvailability,
    ProviderSessionImportUnavailableReason, SafeDiagnostic, SessionRef,
};
use swallowtail_runtime::{
    ProviderSessionCandidate, ProviderSessionCandidateId, ProviderSessionCataloguePlan,
    ProviderSessionImportRevalidation, ProviderSessionOperationFailure,
    ProviderSessionOperationFailureStage, WorkingResourceRef,
};

pub(super) struct ThreadPage {
    pub candidates: Vec<ProviderSessionCandidate>,
    pub next_cursor: Option<String>,
}

pub(super) struct ThreadRevalidation {
    pub provider_session_ref: SessionRef,
    pub updated_at_unix_milliseconds: u64,
    pub activity: ProviderSessionActivityState,
    pub availability: ProviderSessionImportAvailability,
}

pub(super) fn project_page(
    plan: &ProviderSessionCataloguePlan,
    response: &Value,
    expected_cwd: &str,
    first_ordinal: u32,
) -> Result<ThreadPage, ProviderSessionOperationFailure> {
    let data = response
        .get("data")
        .and_then(Value::as_array)
        .ok_or_else(malformed_catalogue)?;
    if data.len() > plan.agreement().bounds().maximum_page_size().get() as usize {
        return Err(projection_failure(
            "swallowtail.codex.thread_catalogue.page_limit_exceeded",
            "Codex thread catalogue page exceeds its planned bound",
        ));
    }
    let mut candidates = Vec::with_capacity(data.len());
    for (index, thread) in data.iter().enumerate() {
        let observation = project_thread(thread, expected_cwd)?;
        let ordinal = first_ordinal
            .checked_add(u32::try_from(index).map_err(|_| malformed_catalogue())?)
            .ok_or_else(malformed_catalogue)?;
        let candidate_id =
            ProviderSessionCandidateId::new(format!("codex-thread-candidate-{ordinal}"))
                .map_err(|_| malformed_catalogue())?;
        let display = ProviderSessionDisplayContent::new(
            optional_content(thread, "name")?,
            optional_content(thread, "preview")?,
        )
        .map_err(|_| malformed_catalogue())?;
        candidates.push(
            ProviderSessionCandidate::new(
                plan,
                candidate_id,
                observation.provider_session_ref,
                display,
                Some(observation.updated_at_unix_milliseconds),
                observation.activity,
                observation.availability,
            )
            .map_err(|error| {
                operation_failure(
                    ProviderSessionOperationFailureStage::CatalogueProjection,
                    error.diagnostic().clone(),
                )
            })?,
        );
    }
    let next_cursor = match response.get("nextCursor") {
        None | Some(Value::Null) => None,
        Some(Value::String(value)) if !value.trim().is_empty() => Some(value.clone()),
        Some(_) => return Err(malformed_catalogue()),
    };
    Ok(ThreadPage {
        candidates,
        next_cursor,
    })
}

pub(super) fn project_revalidation(
    response: &Value,
    expected_session: &SessionRef,
    expected_cwd: &str,
    expected_updated_at: Option<u64>,
    candidate_id: ProviderSessionCandidateId,
    working_resource: WorkingResourceRef,
) -> Result<ProviderSessionImportRevalidation, ProviderSessionOperationFailure> {
    let thread = response.get("thread").ok_or_else(malformed_revalidation)?;
    let observation = project_thread(thread, expected_cwd).map_err(|failure| {
        operation_failure(
            ProviderSessionOperationFailureStage::ImportRevalidation,
            failure.diagnostic().clone(),
        )
    })?;
    if &observation.provider_session_ref != expected_session
        || expected_updated_at != Some(observation.updated_at_unix_milliseconds)
    {
        return Err(operation_failure(
            ProviderSessionOperationFailureStage::ImportRevalidation,
            SafeDiagnostic::new(
                "swallowtail.codex.thread_import.candidate_changed",
                "Codex thread changed after catalogue observation",
            ),
        ));
    }
    Ok(ProviderSessionImportRevalidation::new(
        candidate_id,
        observation.provider_session_ref,
        working_resource,
        observation.activity,
        observation.availability,
    ))
}

fn project_thread(
    thread: &Value,
    expected_cwd: &str,
) -> Result<ThreadRevalidation, ProviderSessionOperationFailure> {
    let provider_session_ref = SessionRef::new(required_text(thread, "id")?.to_owned())
        .map_err(|_| malformed_catalogue())?;
    if required_text(thread, "cwd")? != expected_cwd {
        return Err(projection_failure(
            "swallowtail.codex.thread_catalogue.resource_mismatch",
            "Codex thread catalogue returned a thread from another working resource",
        ));
    }
    if !matches!(
        required_text(thread, "source")?,
        "cli" | "vscode" | "appServer"
    ) {
        return Err(projection_failure(
            "swallowtail.codex.thread_catalogue.source_mismatch",
            "Codex thread catalogue returned an excluded thread source",
        ));
    }
    let updated_at = thread
        .get("updatedAt")
        .and_then(Value::as_u64)
        .ok_or_else(malformed_catalogue)?
        .checked_mul(1_000)
        .ok_or_else(malformed_catalogue)?;
    let status = thread
        .get("status")
        .and_then(|status| status.get("type"))
        .and_then(Value::as_str)
        .ok_or_else(malformed_catalogue)?;
    let (activity, availability) = match status {
        "notLoaded" | "idle" => (
            ProviderSessionActivityState::Inactive,
            ProviderSessionImportAvailability::Available,
        ),
        "active" => (
            ProviderSessionActivityState::Active,
            ProviderSessionImportAvailability::Unavailable(
                ProviderSessionImportUnavailableReason::Active,
            ),
        ),
        "systemError" => (
            ProviderSessionActivityState::Unknown,
            ProviderSessionImportAvailability::Unavailable(
                ProviderSessionImportUnavailableReason::ProviderReportedUnavailable,
            ),
        ),
        _ => return Err(malformed_catalogue()),
    };
    Ok(ThreadRevalidation {
        provider_session_ref,
        updated_at_unix_milliseconds: updated_at,
        activity,
        availability,
    })
}

fn optional_content(
    value: &Value,
    field: &str,
) -> Result<Option<String>, ProviderSessionOperationFailure> {
    match value.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(text)) if text.trim().is_empty() => Ok(None),
        Some(Value::String(text)) => Ok(Some(text.clone())),
        Some(_) => Err(malformed_catalogue()),
    }
}

fn required_text<'a>(
    value: &'a Value,
    field: &str,
) -> Result<&'a str, ProviderSessionOperationFailure> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|text| !text.trim().is_empty())
        .ok_or_else(malformed_catalogue)
}

fn malformed_catalogue() -> ProviderSessionOperationFailure {
    projection_failure(
        "swallowtail.codex.thread_catalogue.malformed_response",
        "Codex app-server returned a malformed thread catalogue response",
    )
}

fn malformed_revalidation() -> ProviderSessionOperationFailure {
    operation_failure(
        ProviderSessionOperationFailureStage::ImportRevalidation,
        SafeDiagnostic::new(
            "swallowtail.codex.thread_import.malformed_response",
            "Codex app-server returned malformed thread revalidation evidence",
        ),
    )
}

fn projection_failure(
    code: &'static str,
    message: &'static str,
) -> ProviderSessionOperationFailure {
    operation_failure(
        ProviderSessionOperationFailureStage::CatalogueProjection,
        SafeDiagnostic::new(code, message),
    )
}

fn operation_failure(
    stage: ProviderSessionOperationFailureStage,
    diagnostic: SafeDiagnostic,
) -> ProviderSessionOperationFailure {
    ProviderSessionOperationFailure::new(stage, diagnostic)
}
