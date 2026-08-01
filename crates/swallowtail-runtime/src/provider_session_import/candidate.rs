use super::{ProviderSessionCataloguePlan, failure, same_working_resource};
use crate::{
    ProviderSessionCandidateId, ProviderSessionCatalogueId, RuntimeFailure, WorkingResourceRef,
};
use std::fmt;
use swallowtail_core::{
    PreflightPlan, ProviderSessionActivityState, ProviderSessionDisplayContent,
    ProviderSessionImportAvailability, SessionRef,
};

#[derive(Clone, Eq, PartialEq)]
pub struct ProviderSessionCursor {
    catalogue_id: ProviderSessionCatalogueId,
    source_preflight: PreflightPlan,
    value: String,
}

impl ProviderSessionCursor {
    pub fn new(
        plan: &ProviderSessionCataloguePlan,
        value: impl Into<String>,
    ) -> Result<Self, RuntimeFailure> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(failure(
                "swallowtail.provider_session_catalogue.cursor_required",
                "Provider-session catalogue cursor must not be empty",
            ));
        }
        if value.len() > plan.agreement().bounds().maximum_cursor_bytes().get() as usize {
            return Err(failure(
                "swallowtail.provider_session_catalogue.cursor_limit_exceeded",
                "Provider-session catalogue cursor exceeds its planned bound",
            ));
        }
        Ok(Self {
            catalogue_id: plan.agreement().catalogue_id().clone(),
            source_preflight: plan.preflight().clone(),
            value,
        })
    }

    #[must_use]
    pub fn as_provider_value(&self) -> &str {
        &self.value
    }

    pub(super) fn matches_plan(&self, plan: &ProviderSessionCataloguePlan) -> bool {
        self.catalogue_id == *plan.agreement().catalogue_id()
            && self.source_preflight == *plan.preflight()
    }
}

impl fmt::Debug for ProviderSessionCursor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProviderSessionCursor")
            .field("catalogue_id", &self.catalogue_id)
            .field(
                "value",
                &format_args!("<opaque:{} bytes>", self.value.len()),
            )
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct ProviderSessionCandidate {
    catalogue_id: ProviderSessionCatalogueId,
    source_preflight: PreflightPlan,
    candidate_id: ProviderSessionCandidateId,
    provider_session_ref: SessionRef,
    working_resource: WorkingResourceRef,
    display: ProviderSessionDisplayContent,
    updated_at_unix_milliseconds: Option<u64>,
    activity: ProviderSessionActivityState,
    import_availability: ProviderSessionImportAvailability,
}

impl ProviderSessionCandidate {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        plan: &ProviderSessionCataloguePlan,
        candidate_id: ProviderSessionCandidateId,
        provider_session_ref: SessionRef,
        display: ProviderSessionDisplayContent,
        updated_at_unix_milliseconds: Option<u64>,
        activity: ProviderSessionActivityState,
        import_availability: ProviderSessionImportAvailability,
    ) -> Result<Self, RuntimeFailure> {
        if provider_session_ref.as_provider_value().len()
            > plan
                .agreement()
                .bounds()
                .maximum_provider_reference_bytes()
                .get() as usize
        {
            return Err(failure(
                "swallowtail.provider_session_catalogue.reference_limit_exceeded",
                "Provider-session reference exceeds its planned bound",
            ));
        }
        if display.byte_len() > plan.agreement().bounds().maximum_content_bytes().get() as usize {
            return Err(failure(
                "swallowtail.provider_session_catalogue.content_limit_exceeded",
                "Provider-session display content exceeds its planned bound",
            ));
        }
        Ok(Self {
            catalogue_id: plan.agreement().catalogue_id().clone(),
            source_preflight: plan.preflight().clone(),
            candidate_id,
            provider_session_ref,
            working_resource: plan.agreement().scope().working_resource_ref().clone(),
            display,
            updated_at_unix_milliseconds,
            activity,
            import_availability,
        })
    }

    #[must_use]
    pub const fn candidate_id(&self) -> &ProviderSessionCandidateId {
        &self.candidate_id
    }

    #[must_use]
    pub const fn display(&self) -> &ProviderSessionDisplayContent {
        &self.display
    }

    #[must_use]
    pub const fn updated_at_unix_milliseconds(&self) -> Option<u64> {
        self.updated_at_unix_milliseconds
    }

    #[must_use]
    pub const fn activity(&self) -> ProviderSessionActivityState {
        self.activity
    }

    #[must_use]
    pub const fn import_availability(&self) -> ProviderSessionImportAvailability {
        self.import_availability
    }

    pub(super) const fn provider_session_ref(&self) -> &SessionRef {
        &self.provider_session_ref
    }

    pub(super) const fn working_resource(&self) -> &WorkingResourceRef {
        &self.working_resource
    }

    pub(super) fn matches_catalogue_plan(&self, plan: &ProviderSessionCataloguePlan) -> bool {
        self.catalogue_id == *plan.agreement().catalogue_id()
            && self.source_preflight == *plan.preflight()
            && same_working_resource(
                &self.working_resource,
                plan.agreement().scope().working_resource_ref(),
            )
    }
}

impl fmt::Debug for ProviderSessionCandidate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProviderSessionCandidate")
            .field("catalogue_id", &self.catalogue_id)
            .field("candidate_id", &self.candidate_id)
            .field("provider_session_ref", &self.provider_session_ref)
            .field("working_resource", &self.working_resource)
            .field("display", &self.display)
            .field(
                "updated_at_unix_milliseconds",
                &self.updated_at_unix_milliseconds,
            )
            .field("activity", &self.activity)
            .field("import_availability", &self.import_availability)
            .finish()
    }
}
