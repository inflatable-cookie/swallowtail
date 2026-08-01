use super::{
    ProviderSessionCandidate, ProviderSessionCataloguePlan, failure, requires_capability,
    requires_service, same_catalogue_and_import_binding, same_working_resource,
};
use crate::{
    CancellationControl, Deadline, ImmediateCancellation, ProviderSessionCandidateId, RequestId,
    RuntimeFailure, SessionPlanAgreement, WorkingResourceRef, validate_session_plan_agreement,
};
use std::sync::Arc;
use swallowtail_core::{
    CancellationScope, Capability, DriverRole, ExecutionLayer, HostServiceKind, OperationShape,
    PreflightPlan, ProviderSessionImportAvailability, SessionRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSessionImportAgreement {
    candidate: ProviderSessionCandidate,
    working_resource: WorkingResourceRef,
    session: SessionPlanAgreement,
    deadline: Option<Deadline>,
}

impl ProviderSessionImportAgreement {
    #[must_use]
    pub const fn new(
        candidate: ProviderSessionCandidate,
        working_resource: WorkingResourceRef,
        session: SessionPlanAgreement,
        deadline: Option<Deadline>,
    ) -> Self {
        Self {
            candidate,
            working_resource,
            session,
            deadline,
        }
    }

    #[must_use]
    pub const fn candidate_id(&self) -> &ProviderSessionCandidateId {
        self.candidate.candidate_id()
    }

    #[must_use]
    pub const fn working_resource(&self) -> &WorkingResourceRef {
        &self.working_resource
    }

    #[must_use]
    pub const fn session(&self) -> &SessionPlanAgreement {
        &self.session
    }

    #[must_use]
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline
    }

    pub(super) const fn candidate(&self) -> &ProviderSessionCandidate {
        &self.candidate
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSessionImportPlan {
    preflight: PreflightPlan,
    source_catalogue: ProviderSessionCataloguePlan,
    agreement: ProviderSessionImportAgreement,
}

impl ProviderSessionImportPlan {
    pub fn new(
        preflight: PreflightPlan,
        source_catalogue: ProviderSessionCataloguePlan,
        agreement: ProviderSessionImportAgreement,
    ) -> Result<Self, RuntimeFailure> {
        validate_plan(&preflight, &source_catalogue, &agreement)?;
        Ok(Self {
            preflight,
            source_catalogue,
            agreement,
        })
    }

    #[must_use]
    pub const fn preflight(&self) -> &PreflightPlan {
        &self.preflight
    }

    #[must_use]
    pub const fn source_catalogue(&self) -> &ProviderSessionCataloguePlan {
        &self.source_catalogue
    }

    #[must_use]
    pub const fn agreement(&self) -> &ProviderSessionImportAgreement {
        &self.agreement
    }
}

#[derive(Clone, Debug)]
pub struct ProviderSessionImportRequest {
    request_id: RequestId,
    agreement: ProviderSessionImportAgreement,
    cancellation: Arc<ImmediateCancellation>,
}

impl ProviderSessionImportRequest {
    pub fn new(
        request_id: RequestId,
        plan: &ProviderSessionImportPlan,
        cancellation: Arc<ImmediateCancellation>,
    ) -> Result<Self, RuntimeFailure> {
        if cancellation.scope() != CancellationScope::ProviderSessionImport {
            return Err(failure(
                "swallowtail.provider_session_import.cancellation_scope_mismatch",
                "Provider-session import request has the wrong cancellation scope",
            ));
        }
        Ok(Self {
            request_id,
            agreement: plan.agreement().clone(),
            cancellation,
        })
    }

    pub fn from_plan(
        request_id: RequestId,
        plan: &ProviderSessionImportPlan,
    ) -> Result<Self, RuntimeFailure> {
        Self::new(
            request_id,
            plan,
            Arc::new(ImmediateCancellation::new(
                CancellationScope::ProviderSessionImport,
            )),
        )
    }

    #[must_use]
    pub const fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    #[must_use]
    pub const fn agreement(&self) -> &ProviderSessionImportAgreement {
        &self.agreement
    }

    #[must_use]
    pub const fn provider_session_ref(&self) -> &SessionRef {
        self.agreement.candidate().provider_session_ref()
    }

    #[must_use]
    pub const fn cancellation(&self) -> &Arc<ImmediateCancellation> {
        &self.cancellation
    }
}

fn validate_plan(
    preflight: &PreflightPlan,
    source_catalogue: &ProviderSessionCataloguePlan,
    agreement: &ProviderSessionImportAgreement,
) -> Result<(), RuntimeFailure> {
    if preflight.requirements().execution_layer() != ExecutionLayer::HarnessInteraction
        || preflight.requirements().driver_role() != DriverRole::ProviderSessionImport
        || preflight.requirements().operation_shape() != OperationShape::ProviderSessionImport
    {
        return Err(plan_mismatch());
    }
    for capability in [
        Capability::ProviderSessionImport,
        Capability::LoadSession,
        Capability::Resume,
    ] {
        if !requires_capability(preflight, capability) {
            return Err(failure(
                "swallowtail.provider_session_import.capability_mismatch",
                "Provider-session import plan lacks its complete continuation capabilities",
            ));
        }
    }
    if !requires_service(preflight, HostServiceKind::Task)
        || !requires_service(preflight, HostServiceKind::WorkingResource)
    {
        return Err(failure(
            "swallowtail.provider_session_import.service_required",
            "Provider-session import requires scoped task and working-resource services",
        ));
    }
    if agreement.deadline().is_some() && !requires_service(preflight, HostServiceKind::Time) {
        return Err(failure(
            "swallowtail.provider_session_import.time_service_required",
            "Deadline-bound provider-session import requires time service",
        ));
    }
    if preflight.model_route_id().is_none() || preflight.model_id().is_none() {
        return Err(failure(
            "swallowtail.provider_session_import.model_route_required",
            "Provider-session import requires an exact future model route",
        ));
    }
    if preflight.interface_versions().next().is_none() {
        return Err(failure(
            "swallowtail.provider_session_import.interface_version_required",
            "Provider-session import requires exact interface-version evidence",
        ));
    }
    validate_session_plan_agreement(preflight, agreement.session())?;
    if !agreement
        .candidate()
        .matches_catalogue_plan(source_catalogue)
    {
        return Err(failure(
            "swallowtail.provider_session_import.candidate_plan_mismatch",
            "Provider-session candidate does not match its source catalogue plan",
        ));
    }
    if agreement.candidate().import_availability() != ProviderSessionImportAvailability::Available {
        return Err(failure(
            "swallowtail.provider_session_import.candidate_unavailable",
            "Provider-session candidate is unavailable for explicit import",
        ));
    }
    if !same_catalogue_and_import_binding(source_catalogue.preflight(), preflight) {
        return Err(failure(
            "swallowtail.provider_session_import.attachment_mismatch",
            "Provider-session import attachment differs from its source catalogue",
        ));
    }
    if !same_working_resource(
        agreement.candidate().working_resource(),
        agreement.working_resource(),
    ) {
        return Err(failure(
            "swallowtail.provider_session_import.resource_mismatch",
            "Provider-session import working resource differs from its candidate",
        ));
    }
    Ok(())
}

fn plan_mismatch() -> RuntimeFailure {
    failure(
        "swallowtail.provider_session_import.plan_mismatch",
        "Provider-session import does not match its immutable plan",
    )
}
