use super::{
    ProviderSessionCandidate, ProviderSessionCataloguePlan, failure, requires_capability,
    requires_service, same_catalogue_and_import_binding, same_working_resource,
};
use crate::plan_family::{PlanRule, check_plan_rules};
use crate::{
    CancellationControl, Deadline, ImmediateCancellation, ProviderSessionCandidateId, RequestId,
    RuntimeFailure, SessionPlanAgreement, WorkingResourceRef, validate_session_plan_agreement,
};
use std::sync::Arc;
use swallowtail_core::{
    CancellationScope, Capability, DriverRole, ExecutionLayer, HostServiceKind, OperationShape,
    PreflightPlan, ProviderSessionImportAvailability, SessionRef,
};

/// Explicit candidate selection and future attachment agreement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSessionImportAgreement {
    candidate: ProviderSessionCandidate,
    working_resource: WorkingResourceRef,
    session: SessionPlanAgreement,
    deadline: Option<Deadline>,
}

impl ProviderSessionImportAgreement {
    /// Creates import authorization for one candidate and exact attachment.
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
    /// Returns the selected operation-local candidate identity.
    pub const fn candidate_id(&self) -> &ProviderSessionCandidateId {
        self.candidate.candidate_id()
    }

    #[must_use]
    /// Returns the exact host-approved working resource.
    pub const fn working_resource(&self) -> &WorkingResourceRef {
        &self.working_resource
    }

    #[must_use]
    /// Returns the future session access and provider-state agreement.
    pub const fn session(&self) -> &SessionPlanAgreement {
        &self.session
    }

    #[must_use]
    /// Returns the optional import deadline.
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline
    }

    #[must_use]
    /// Returns the selected catalogue candidate.
    pub const fn candidate(&self) -> &ProviderSessionCandidate {
        &self.candidate
    }
}

/// Side-effect-free plan for revalidating and importing one candidate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSessionImportPlan {
    preflight: PreflightPlan,
    source_catalogue: ProviderSessionCataloguePlan,
    agreement: ProviderSessionImportAgreement,
}

impl ProviderSessionImportPlan {
    /// Validates and creates an explicit provider-session import plan.
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
    /// Returns the exact import route preflight plan.
    pub const fn preflight(&self) -> &PreflightPlan {
        &self.preflight
    }

    #[must_use]
    /// Returns the exact source catalogue plan.
    pub const fn source_catalogue(&self) -> &ProviderSessionCataloguePlan {
        &self.source_catalogue
    }

    #[must_use]
    /// Returns the immutable import and attachment agreement.
    pub const fn agreement(&self) -> &ProviderSessionImportAgreement {
        &self.agreement
    }
}

/// Typed request to revalidate and import one selected provider session.
#[derive(Clone, Debug)]
pub struct ProviderSessionImportRequest {
    request_id: RequestId,
    agreement: ProviderSessionImportAgreement,
    cancellation: Arc<ImmediateCancellation>,
}

impl ProviderSessionImportRequest {
    /// Creates a request after validating its cancellation scope.
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

    /// Creates a request with a new import-scoped cancellation control.
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
    /// Returns the consumer-unique request identity.
    pub const fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    #[must_use]
    /// Returns the immutable import agreement.
    pub const fn agreement(&self) -> &ProviderSessionImportAgreement {
        &self.agreement
    }

    #[must_use]
    /// Returns the opaque provider-session reference for the owning adapter.
    pub const fn provider_session_ref(&self) -> &SessionRef {
        self.agreement.candidate().provider_session_ref()
    }

    #[must_use]
    /// Returns the import-scoped cancellation control.
    pub const fn cancellation(&self) -> &Arc<ImmediateCancellation> {
        &self.cancellation
    }
}

/// Ordered per-role validation rules for a provider-session import plan.
///
/// The shared rules cover the harness-interaction evidence, continuation
/// capabilities, scoped services, exact model route, and interface-version
/// evidence. The import-specific candidate, attachment, and working-resource
/// rules follow below because they need the source catalogue plan.
const IMPORT_PLAN_RULES: [PlanRule<ProviderSessionImportAgreement>; 8] = [
    PlanRule::new(
        "swallowtail.provider_session_import.plan_mismatch",
        "Provider-session import does not match its immutable plan",
        |preflight, _| {
            preflight.requirements().execution_layer() == ExecutionLayer::HarnessInteraction
        },
    ),
    PlanRule::new(
        "swallowtail.provider_session_import.plan_mismatch",
        "Provider-session import does not match its immutable plan",
        |preflight, _| {
            preflight.requirements().driver_role() == DriverRole::ProviderSessionImport
        },
    ),
    PlanRule::new(
        "swallowtail.provider_session_import.plan_mismatch",
        "Provider-session import does not match its immutable plan",
        |preflight, _| {
            preflight.requirements().operation_shape() == OperationShape::ProviderSessionImport
        },
    ),
    PlanRule::new(
        "swallowtail.provider_session_import.capability_mismatch",
        "Provider-session import plan lacks its complete continuation capabilities",
        |preflight, _| {
            [
                Capability::ProviderSessionImport,
                Capability::LoadSession,
                Capability::Resume,
            ]
            .iter()
            .all(|capability| requires_capability(preflight, *capability))
        },
    ),
    PlanRule::new(
        "swallowtail.provider_session_import.service_required",
        "Provider-session import requires scoped task and working-resource services",
        |preflight, _| {
            requires_service(preflight, HostServiceKind::Task)
                && requires_service(preflight, HostServiceKind::WorkingResource)
        },
    ),
    PlanRule::new(
        "swallowtail.provider_session_import.time_service_required",
        "Deadline-bound provider-session import requires time service",
        |preflight, agreement| {
            agreement.deadline().is_none() || requires_service(preflight, HostServiceKind::Time)
        },
    ),
    PlanRule::new(
        "swallowtail.provider_session_import.model_route_required",
        "Provider-session import requires an exact future model route",
        |preflight, _| preflight.model_route_id().is_some() && preflight.model_id().is_some(),
    ),
    PlanRule::new(
        "swallowtail.provider_session_import.interface_version_required",
        "Provider-session import requires exact interface-version evidence",
        |preflight, _| preflight.interface_versions().next().is_some(),
    ),
];

fn validate_plan(
    preflight: &PreflightPlan,
    source_catalogue: &ProviderSessionCataloguePlan,
    agreement: &ProviderSessionImportAgreement,
) -> Result<(), RuntimeFailure> {
    check_plan_rules(preflight, agreement, &IMPORT_PLAN_RULES)?;
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
