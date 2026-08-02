use super::{from_runtime, import_revalidation, operation_failure};
use std::collections::BTreeSet;
use std::sync::Arc;
use swallowtail_core::{
    ProviderSessionActivityState, ProviderSessionDisplayContent, ProviderSessionImportAvailability,
    ProviderSessionImportUnavailableReason, SessionRef,
};
use swallowtail_protocol_acp::{
    AcpSessionInfo, AcpSessionListCapabilities, AcpSessionListLimits, AcpSessionListPage,
    DEFAULT_MAX_FRAME_BYTES, decode_session_list_capabilities,
};
use swallowtail_runtime::{
    ProviderSessionCandidate, ProviderSessionCandidateId, ProviderSessionCataloguePlan,
    ProviderSessionImportPlan, ProviderSessionImportRequest, ProviderSessionImportRevalidation,
    ProviderSessionOperationFailure, ProviderSessionOperationFailureStage,
};

pub(super) async fn find_candidate(
    plan: &ProviderSessionImportPlan,
    request: &ProviderSessionImportRequest,
    connection: Arc<crate::connection::AcpConnection>,
    cwd: String,
    capabilities: AcpSessionListCapabilities,
) -> Result<ProviderSessionImportRevalidation, ProviderSessionOperationFailure> {
    let mut cursor = None;
    let mut observed = 0_u32;
    let mut seen_sessions = BTreeSet::new();
    let mut seen_cursors = BTreeSet::new();
    loop {
        let page = connection
            .list_sessions(
                capabilities,
                cwd.clone(),
                cursor.clone(),
                limits(plan.source_catalogue()),
            )
            .await
            .map_err(import_revalidation)?;
        for session in page.sessions() {
            observed = observed.checked_add(1).ok_or_else(import_traversal_limit)?;
            if observed
                > plan
                    .source_catalogue()
                    .agreement()
                    .bounds()
                    .maximum_total_candidates()
                    .get()
                || !seen_sessions.insert(session.session_id().to_owned())
            {
                return Err(import_traversal_limit());
            }
            if session.session_id() == request.provider_session_ref().as_provider_value() {
                return revalidate_session(plan, request, session, &cwd);
            }
        }
        let Some(next) = page.next_cursor().map(str::to_owned) else {
            return Err(operation_failure(
                ProviderSessionOperationFailureStage::ImportRevalidation,
                "swallowtail.kimi.session_import.candidate_missing",
                "Kimi Code session is no longer present in its scoped catalogue",
            ));
        };
        if !seen_cursors.insert(next.clone()) {
            return Err(import_traversal_limit());
        }
        cursor = Some(next);
    }
}

pub(super) fn negotiated_capabilities(
    initialize: &serde_json::Value,
    stage: ProviderSessionOperationFailureStage,
) -> Result<AcpSessionListCapabilities, ProviderSessionOperationFailure> {
    let capabilities = decode_session_list_capabilities(initialize).map_err(|_| {
        operation_failure(
            stage,
            "swallowtail.kimi.session_catalogue.capability_invalid",
            "Kimi Code did not negotiate valid ACP session-list support",
        )
    })?;
    if !capabilities.list() {
        return Err(operation_failure(
            stage,
            "swallowtail.kimi.session_catalogue.unsupported",
            "Kimi Code did not advertise ACP session listing",
        ));
    }
    Ok(capabilities)
}

pub(super) fn project_page(
    plan: &ProviderSessionCataloguePlan,
    page: &AcpSessionListPage,
    first_ordinal: u32,
) -> Result<(Vec<ProviderSessionCandidate>, Option<String>), ProviderSessionOperationFailure> {
    let mut candidates = Vec::with_capacity(page.sessions().len());
    for (index, session) in page.sessions().enumerate() {
        let ordinal = first_ordinal
            .checked_add(u32::try_from(index).map_err(|_| traversal_limit())?)
            .ok_or_else(traversal_limit)?;
        let candidate_id =
            ProviderSessionCandidateId::new(format!("kimi-acp-session-candidate-{ordinal}"))
                .map_err(|_| malformed_catalogue())?;
        let provider_session_ref =
            SessionRef::new(session.session_id()).map_err(|_| malformed_catalogue())?;
        let display = ProviderSessionDisplayContent::new(session.title().map(str::to_owned), None)
            .map_err(|_| malformed_catalogue())?;
        let availability = if session.additional_directories().len() == 0 {
            ProviderSessionImportAvailability::Available
        } else {
            ProviderSessionImportAvailability::Unavailable(
                ProviderSessionImportUnavailableReason::ResourceMismatch,
            )
        };
        candidates.push(
            ProviderSessionCandidate::new(
                plan,
                candidate_id,
                provider_session_ref,
                display,
                session.updated_at_unix_milliseconds(),
                ProviderSessionActivityState::Unknown,
                availability,
            )
            .map_err(|error| {
                from_runtime(
                    ProviderSessionOperationFailureStage::CatalogueProjection,
                    error,
                )
            })?,
        );
    }
    Ok((candidates, page.next_cursor().map(str::to_owned)))
}

fn revalidate_session(
    plan: &ProviderSessionImportPlan,
    request: &ProviderSessionImportRequest,
    session: &AcpSessionInfo,
    expected_cwd: &str,
) -> Result<ProviderSessionImportRevalidation, ProviderSessionOperationFailure> {
    let candidate = plan.agreement().candidate();
    if session.cwd() != expected_cwd
        || session.additional_directories().len() != 0
        || session.title() != candidate.display().title()
        || session.updated_at_unix_milliseconds() != candidate.updated_at_unix_milliseconds()
    {
        return Err(operation_failure(
            ProviderSessionOperationFailureStage::ImportRevalidation,
            "swallowtail.kimi.session_import.candidate_changed",
            "Kimi Code session changed after catalogue observation",
        ));
    }
    Ok(ProviderSessionImportRevalidation::new(
        candidate.candidate_id().clone(),
        request.provider_session_ref().clone(),
        plan.agreement().working_resource().clone(),
        ProviderSessionActivityState::Unknown,
        ProviderSessionImportAvailability::Available,
    ))
}

pub(super) fn limits(plan: &ProviderSessionCataloguePlan) -> AcpSessionListLimits {
    let bounds = plan.agreement().bounds();
    AcpSessionListLimits::new(
        DEFAULT_MAX_FRAME_BYTES,
        bounds.maximum_page_size().get() as usize,
        bounds.maximum_provider_reference_bytes().get() as usize,
        16 * 1024,
        bounds.maximum_content_bytes().get() as usize,
        bounds.maximum_cursor_bytes().get() as usize,
        bounds.maximum_content_bytes().get() as usize,
        64,
    )
}

fn malformed_catalogue() -> ProviderSessionOperationFailure {
    operation_failure(
        ProviderSessionOperationFailureStage::CatalogueProjection,
        "swallowtail.kimi.session_catalogue.malformed_response",
        "Kimi Code returned malformed session catalogue evidence",
    )
}

fn traversal_limit() -> ProviderSessionOperationFailure {
    operation_failure(
        ProviderSessionOperationFailureStage::CatalogueProjection,
        "swallowtail.kimi.session_catalogue.traversal_limit_exceeded",
        "Kimi Code session catalogue traversal exceeded its planned bound",
    )
}

fn import_traversal_limit() -> ProviderSessionOperationFailure {
    operation_failure(
        ProviderSessionOperationFailureStage::ImportRevalidation,
        "swallowtail.kimi.session_import.traversal_limit_exceeded",
        "Kimi Code session import revalidation exceeded its planned bound",
    )
}
