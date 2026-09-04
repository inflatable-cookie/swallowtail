use super::{
    ProviderSessionCandidate, ProviderSessionCataloguePlan, ProviderSessionCatalogueRequest,
    ProviderSessionCursor, ProviderSessionImportPlan, ProviderSessionImportRequest,
    ProviderSessionOperationFailure, ProviderSessionOperationFailureStage,
    validate_provider_session_catalogue_request, validate_provider_session_import_request,
};
use crate::{CleanupOutcome, SessionResumeBinding, WorkingResourceRef};
use std::collections::BTreeSet;
use std::fmt;
use swallowtail_core::{
    ProviderSessionActivityState, ProviderSessionBindingOrigin, ProviderSessionImportAvailability,
    SafeDiagnostic, SessionRef,
};

/// One bounded provider-session catalogue page and its continuation evidence.
#[derive(Clone)]
pub struct ProviderSessionCatalogueOutcome {
    source_plan: ProviderSessionCataloguePlan,
    candidates: Vec<ProviderSessionCandidate>,
    next_cursor: Option<ProviderSessionCursor>,
    cleanup: CleanupOutcome,
}

impl ProviderSessionCatalogueOutcome {
    /// Validates and creates a complete catalogue-page outcome.
    pub fn new(
        plan: &ProviderSessionCataloguePlan,
        request: &ProviderSessionCatalogueRequest,
        candidates: Vec<ProviderSessionCandidate>,
        next_cursor_value: Option<String>,
        cleanup: CleanupOutcome,
    ) -> Result<Self, ProviderSessionOperationFailure> {
        validate_provider_session_catalogue_request(plan, request).map_err(|failure| {
            ProviderSessionOperationFailure::from_runtime(
                ProviderSessionOperationFailureStage::BeforeDispatch,
                failure,
            )
        })?;
        require_clean(&cleanup)?;

        let page_bound = plan.agreement().bounds().maximum_page_size().get() as usize;
        if candidates.len() > page_bound {
            return Err(projection_failure(
                "swallowtail.provider_session_catalogue.page_limit_exceeded",
                "Provider-session catalogue page exceeds its planned bound",
            ));
        }
        if candidates.is_empty() && next_cursor_value.is_some() {
            return Err(projection_failure(
                "swallowtail.provider_session_catalogue.empty_page_cursor",
                "Provider-session catalogue cannot advance an empty page",
            ));
        }
        if candidates
            .iter()
            .any(|candidate| !candidate.matches_catalogue_plan(plan))
        {
            return Err(projection_failure(
                "swallowtail.provider_session_catalogue.candidate_plan_mismatch",
                "Provider-session catalogue returned a candidate from another plan",
            ));
        }

        let mut seen_candidate_ids = request
            .cursor()
            .map_or_else(BTreeSet::new, |cursor| cursor.seen_candidate_ids().clone());
        let mut seen_provider_refs = request
            .cursor()
            .map_or_else(BTreeSet::new, |cursor| cursor.seen_provider_refs().clone());
        for candidate in &candidates {
            if !seen_candidate_ids.insert(candidate.candidate_id().clone())
                || !seen_provider_refs.insert(candidate.provider_session_ref().clone())
            {
                return Err(projection_failure(
                    "swallowtail.provider_session_catalogue.duplicate_candidate",
                    "Provider-session catalogue traversal contains duplicate candidates",
                ));
            }
        }
        let observed = u32::try_from(seen_candidate_ids.len()).map_err(|_| {
            projection_failure(
                "swallowtail.provider_session_catalogue.traversal_limit_exceeded",
                "Provider-session catalogue traversal exceeds its planned bound",
            )
        })?;
        let total_bound = plan.agreement().bounds().maximum_total_candidates().get();
        if observed > total_bound || (observed == total_bound && next_cursor_value.is_some()) {
            return Err(projection_failure(
                "swallowtail.provider_session_catalogue.traversal_limit_exceeded",
                "Provider-session catalogue traversal exceeds its planned bound",
            ));
        }
        let next_cursor = next_cursor_value
            .map(|value| {
                ProviderSessionCursor::new(plan, value, seen_candidate_ids, seen_provider_refs)
            })
            .transpose()
            .map_err(|failure| {
                ProviderSessionOperationFailure::from_runtime(
                    ProviderSessionOperationFailureStage::CatalogueProjection,
                    failure,
                )
            })?;

        Ok(Self {
            source_plan: plan.clone(),
            candidates,
            next_cursor,
            cleanup,
        })
    }

    /// Iterates candidates in provider-supplied page order.
    pub fn candidates(&self) -> impl ExactSizeIterator<Item = &ProviderSessionCandidate> {
        self.candidates.iter()
    }

    #[must_use]
    /// Returns the next bounded cursor, when more provider data is available.
    pub const fn next_cursor(&self) -> Option<&ProviderSessionCursor> {
        self.next_cursor.as_ref()
    }

    #[must_use]
    /// Returns operation cleanup truth.
    pub const fn cleanup(&self) -> &CleanupOutcome {
        &self.cleanup
    }

    pub(crate) const fn source_plan(&self) -> &ProviderSessionCataloguePlan {
        &self.source_plan
    }
}

impl fmt::Debug for ProviderSessionCatalogueOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProviderSessionCatalogueOutcome")
            .field("candidates", &self.candidates)
            .field("next_cursor", &self.next_cursor)
            .field("cleanup", &self.cleanup)
            .finish()
    }
}

impl PartialEq for ProviderSessionCatalogueOutcome {
    fn eq(&self, other: &Self) -> bool {
        self.candidates == other.candidates
            && self.next_cursor == other.next_cursor
            && self.cleanup == other.cleanup
    }
}

impl Eq for ProviderSessionCatalogueOutcome {}

/// Exact read-only evidence observed while revalidating an import candidate.
#[derive(Clone, Eq, PartialEq)]
pub struct ProviderSessionImportRevalidation {
    candidate_id: crate::ProviderSessionCandidateId,
    provider_session_ref: SessionRef,
    working_resource: WorkingResourceRef,
    activity: ProviderSessionActivityState,
    availability: ProviderSessionImportAvailability,
}

impl ProviderSessionImportRevalidation {
    /// Creates exact revalidation evidence for the selected candidate.
    #[must_use]
    pub const fn new(
        candidate_id: crate::ProviderSessionCandidateId,
        provider_session_ref: SessionRef,
        working_resource: WorkingResourceRef,
        activity: ProviderSessionActivityState,
        availability: ProviderSessionImportAvailability,
    ) -> Self {
        Self {
            candidate_id,
            provider_session_ref,
            working_resource,
            activity,
            availability,
        }
    }

    #[must_use]
    /// Returns the revalidated operation-local candidate identity.
    pub const fn candidate_id(&self) -> &crate::ProviderSessionCandidateId {
        &self.candidate_id
    }

    #[must_use]
    /// Returns the revalidated provider-session activity state.
    pub const fn activity(&self) -> ProviderSessionActivityState {
        self.activity
    }

    #[must_use]
    /// Returns the revalidated import availability.
    pub const fn availability(&self) -> ProviderSessionImportAvailability {
        self.availability
    }
}

impl fmt::Debug for ProviderSessionImportRevalidation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProviderSessionImportRevalidation")
            .field("candidate_id", &self.candidate_id)
            .field("provider_session_ref", &self.provider_session_ref)
            .field("working_resource", &self.working_resource)
            .field("activity", &self.activity)
            .field("availability", &self.availability)
            .finish()
    }
}

/// Successful explicit import result containing separately usable resume authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSessionImportOutcome {
    binding: SessionResumeBinding,
    revalidation: ProviderSessionImportRevalidation,
    cleanup: CleanupOutcome,
}

impl ProviderSessionImportOutcome {
    /// Validates revalidation evidence and issues one exact resume binding.
    pub fn new(
        plan: &ProviderSessionImportPlan,
        request: &ProviderSessionImportRequest,
        revalidation: ProviderSessionImportRevalidation,
        cleanup: CleanupOutcome,
    ) -> Result<Self, ProviderSessionOperationFailure> {
        validate_provider_session_import_request(plan, request).map_err(|failure| {
            ProviderSessionOperationFailure::from_runtime(
                ProviderSessionOperationFailureStage::BeforeDispatch,
                failure,
            )
        })?;
        require_clean(&cleanup)?;
        let candidate = plan.agreement().candidate();
        if revalidation.candidate_id != *candidate.candidate_id()
            || revalidation.provider_session_ref != *candidate.provider_session_ref()
            || revalidation.working_resource != *plan.agreement().working_resource()
            || revalidation.availability != ProviderSessionImportAvailability::Available
        {
            return Err(ProviderSessionOperationFailure::new(
                ProviderSessionOperationFailureStage::ImportRevalidation,
                SafeDiagnostic::new(
                    "swallowtail.provider_session_import.revalidation_mismatch",
                    "Provider-session import revalidation does not match its immutable plan",
                ),
            ));
        }

        let preflight = plan.preflight();
        let binding = SessionResumeBinding::new(
            revalidation.provider_session_ref.clone(),
            preflight.instance_id().clone(),
            preflight.execution_host_id().clone(),
            preflight.model_route_id().cloned().ok_or_else(|| {
                binding_failure("Provider-session import lost its model-route binding")
            })?,
            preflight
                .model_id()
                .cloned()
                .ok_or_else(|| binding_failure("Provider-session import lost its model binding"))?,
            revalidation.working_resource.clone(),
            plan.agreement().session().access_policy().clone(),
        )
        .with_origin(ProviderSessionBindingOrigin::ExplicitlyImported);
        if !binding.matches_attachment(
            preflight,
            plan.agreement().working_resource(),
            plan.agreement().session().access_policy(),
        ) {
            return Err(binding_failure(
                "Provider-session import binding differs from its immutable plan",
            ));
        }
        Ok(Self {
            binding,
            revalidation,
            cleanup,
        })
    }

    #[must_use]
    /// Returns the exact imported session-resume binding.
    pub const fn binding(&self) -> &SessionResumeBinding {
        &self.binding
    }

    #[must_use]
    /// Returns the read-only evidence used to issue the binding.
    pub const fn revalidation(&self) -> &ProviderSessionImportRevalidation {
        &self.revalidation
    }

    #[must_use]
    /// Returns operation cleanup truth.
    pub const fn cleanup(&self) -> &CleanupOutcome {
        &self.cleanup
    }
}

fn require_clean(cleanup: &CleanupOutcome) -> Result<(), ProviderSessionOperationFailure> {
    match cleanup {
        CleanupOutcome::Clean | CleanupOutcome::NotApplicable => Ok(()),
        CleanupOutcome::Degraded(_) | CleanupOutcome::Failed(_) => {
            Err(ProviderSessionOperationFailure::new(
                ProviderSessionOperationFailureStage::Cleanup,
                SafeDiagnostic::new(
                    "swallowtail.provider_session_operation.cleanup_failed",
                    "Provider-session operation cleanup did not complete cleanly",
                ),
            ))
        }
    }
}

fn projection_failure(
    code: &'static str,
    message: &'static str,
) -> ProviderSessionOperationFailure {
    ProviderSessionOperationFailure::new(
        ProviderSessionOperationFailureStage::CatalogueProjection,
        SafeDiagnostic::new(code, message),
    )
}

fn binding_failure(message: &'static str) -> ProviderSessionOperationFailure {
    ProviderSessionOperationFailure::new(
        ProviderSessionOperationFailureStage::ImportBindingIssue,
        SafeDiagnostic::new(
            "swallowtail.provider_session_import.binding_invalid",
            message,
        ),
    )
}
