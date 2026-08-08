use super::{failure, requires_capability, requires_read_only_working_resource, requires_service};
use crate::plan_family::{PlanRule, check_plan_rules};
use crate::{
    CancellationControl, Deadline, ImmediateCancellation, ProviderSessionCatalogueId,
    ProviderSessionCursor, RequestId, RuntimeFailure, WorkingResourceRef,
};
use std::sync::Arc;
use swallowtail_core::{
    CancellationScope, Capability, DriverRole, ExecutionLayer, HostServiceKind, OperationShape,
    PreflightPlan, ProviderSessionCatalogueBounds, ProviderSessionDiscoveryScope,
};

/// Exact bounded discovery scope for a provider-session catalogue.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSessionCatalogueScope {
    kind: ProviderSessionDiscoveryScope,
    working_resource: WorkingResourceRef,
}

impl ProviderSessionCatalogueScope {
    #[must_use]
    /// Creates a scope restricted to one host-approved working resource.
    pub const fn working_resource(working_resource: WorkingResourceRef) -> Self {
        Self {
            kind: ProviderSessionDiscoveryScope::WorkingResource,
            working_resource,
        }
    }

    #[must_use]
    /// Returns the provider-session discovery scope kind.
    pub const fn kind(&self) -> ProviderSessionDiscoveryScope {
        self.kind
    }

    #[must_use]
    /// Returns the exact host-approved working resource.
    pub const fn working_resource_ref(&self) -> &WorkingResourceRef {
        &self.working_resource
    }
}

/// Immutable catalogue identity, scope, bounds, and deadline agreement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSessionCatalogueAgreement {
    catalogue_id: ProviderSessionCatalogueId,
    scope: ProviderSessionCatalogueScope,
    bounds: ProviderSessionCatalogueBounds,
    deadline: Option<Deadline>,
}

impl ProviderSessionCatalogueAgreement {
    /// Creates an agreement for one bounded catalogue traversal.
    #[must_use]
    pub const fn new(
        catalogue_id: ProviderSessionCatalogueId,
        scope: ProviderSessionCatalogueScope,
        bounds: ProviderSessionCatalogueBounds,
        deadline: Option<Deadline>,
    ) -> Self {
        Self {
            catalogue_id,
            scope,
            bounds,
            deadline,
        }
    }

    #[must_use]
    /// Returns the catalogue operation identity.
    pub const fn catalogue_id(&self) -> &ProviderSessionCatalogueId {
        &self.catalogue_id
    }

    #[must_use]
    /// Returns the exact discovery scope.
    pub const fn scope(&self) -> &ProviderSessionCatalogueScope {
        &self.scope
    }

    #[must_use]
    /// Returns the portable page, traversal, cursor, and content bounds.
    pub const fn bounds(&self) -> ProviderSessionCatalogueBounds {
        self.bounds
    }

    #[must_use]
    /// Returns the optional operation deadline.
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline
    }
}

use crate::plan_family::plan_family;

plan_family! {
    plan: {
        plan_type: ProviderSessionCataloguePlan,
        prepared_type: PreparedProviderSessionCatalogueEvidence,
        agreement: ProviderSessionCatalogueAgreement,
        plan_doc: "Side-effect-free plan for one bounded provider-session catalogue operation.",
        prepared_doc: "Prepared route and access evidence for a bounded catalogue operation.",
        agreement_doc: "Returns the immutable catalogue agreement.",
    }
}

/// Typed request for one bounded provider-session catalogue page.
#[derive(Clone, Debug)]
pub struct ProviderSessionCatalogueRequest {
    request_id: RequestId,
    agreement: ProviderSessionCatalogueAgreement,
    cursor: Option<ProviderSessionCursor>,
    cancellation: Arc<ImmediateCancellation>,
}

impl ProviderSessionCatalogueRequest {
    /// Creates a request after validating cancellation and cursor scope.
    pub fn new(
        request_id: RequestId,
        plan: &ProviderSessionCataloguePlan,
        cursor: Option<ProviderSessionCursor>,
        cancellation: Arc<ImmediateCancellation>,
    ) -> Result<Self, RuntimeFailure> {
        if cancellation.scope() != CancellationScope::ProviderSessionCatalogue {
            return Err(failure(
                "swallowtail.provider_session_catalogue.cancellation_scope_mismatch",
                "Provider-session catalogue request has the wrong cancellation scope",
            ));
        }
        if cursor
            .as_ref()
            .is_some_and(|cursor| !cursor.matches_plan(plan))
        {
            return Err(failure(
                "swallowtail.provider_session_catalogue.cursor_plan_mismatch",
                "Provider-session catalogue cursor does not match its immutable plan",
            ));
        }
        Ok(Self {
            request_id,
            agreement: plan.agreement().clone(),
            cursor,
            cancellation,
        })
    }

    /// Creates a request with a new catalogue-scoped cancellation control.
    pub fn from_plan(
        request_id: RequestId,
        plan: &ProviderSessionCataloguePlan,
        cursor: Option<ProviderSessionCursor>,
    ) -> Result<Self, RuntimeFailure> {
        Self::new(
            request_id,
            plan,
            cursor,
            Arc::new(ImmediateCancellation::new(
                CancellationScope::ProviderSessionCatalogue,
            )),
        )
    }

    #[must_use]
    /// Returns the consumer-unique request identity.
    pub const fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    #[must_use]
    /// Returns the immutable catalogue agreement.
    pub const fn agreement(&self) -> &ProviderSessionCatalogueAgreement {
        &self.agreement
    }

    #[must_use]
    /// Returns the exact continuation cursor, when requesting a later page.
    pub const fn cursor(&self) -> Option<&ProviderSessionCursor> {
        self.cursor.as_ref()
    }

    #[must_use]
    /// Returns the catalogue-scoped cancellation control.
    pub const fn cancellation(&self) -> &Arc<ImmediateCancellation> {
        &self.cancellation
    }
}

/// Ordered per-role validation rules for a provider-session catalogue plan.
///
/// Catalogue requires harness-interaction evidence, the exact catalogue
/// capability with a read-only working-resource requirement, and scoped task
/// and working-resource services.
const CATALOGUE_PLAN_RULES: [PlanRule<ProviderSessionCatalogueAgreement>; 7] = [
    PlanRule::new(
        "swallowtail.provider_session_catalogue.plan_mismatch",
        "Provider-session catalogue does not match its immutable plan",
        |preflight, _| {
            preflight.requirements().execution_layer() == ExecutionLayer::HarnessInteraction
        },
    ),
    PlanRule::new(
        "swallowtail.provider_session_catalogue.plan_mismatch",
        "Provider-session catalogue does not match its immutable plan",
        |preflight, _| {
            preflight.requirements().driver_role() == DriverRole::ProviderSessionCatalogue
        },
    ),
    PlanRule::new(
        "swallowtail.provider_session_catalogue.plan_mismatch",
        "Provider-session catalogue does not match its immutable plan",
        |preflight, _| {
            preflight.requirements().operation_shape() == OperationShape::ProviderSessionCatalogue
        },
    ),
    PlanRule::new(
        "swallowtail.provider_session_catalogue.capability_mismatch",
        "Provider-session catalogue plan lacks its exact capabilities",
        |preflight, _| {
            requires_capability(preflight, Capability::ProviderSessionCatalogue)
                && requires_read_only_working_resource(preflight)
        },
    ),
    PlanRule::new(
        "swallowtail.provider_session_catalogue.service_required",
        "Provider-session catalogue requires scoped task and working-resource services",
        |preflight, _| {
            requires_service(preflight, HostServiceKind::Task)
                && requires_service(preflight, HostServiceKind::WorkingResource)
        },
    ),
    PlanRule::new(
        "swallowtail.provider_session_catalogue.time_service_required",
        "Deadline-bound provider-session catalogue requires time service",
        |preflight, agreement| {
            agreement.deadline().is_none()
                || requires_service(preflight, HostServiceKind::Time)
        },
    ),
    PlanRule::new(
        "swallowtail.provider_session_catalogue.interface_version_required",
        "Provider-session catalogue requires exact interface-version evidence",
        |preflight, _| preflight.interface_versions().next().is_some(),
    ),
];

fn validate_plan(
    preflight: &PreflightPlan,
    agreement: &ProviderSessionCatalogueAgreement,
) -> Result<(), RuntimeFailure> {
    check_plan_rules(preflight, agreement, &CATALOGUE_PLAN_RULES)
}
