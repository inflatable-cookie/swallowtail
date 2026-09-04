use swallowtail_core::OperationShape;

use super::MAX_CONSUMER_ROUTE_PROVIDER_OPERATION_ROWS;
use super::admission::{admit_sources, admit_view};
use super::applicability::ConsumerRouteApplicability;
use super::contribution::admit_extension_budget;
use super::failure::{ConsumerRouteProjectionFailure, ConsumerRouteProjectionFailureKind, failure};
use super::identity::{ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind};
use super::row::ConsumerRouteProjectionRow;
use super::semantics::{
    ConsumerRouteActorPosture, ConsumerRouteEvidenceStrength, ConsumerRouteLifecycle,
    ConsumerRouteMutationAuthority, ConsumerRouteSourceClass, ConsumerRouteStateSupport,
};
use super::value::{
    ConsumerRouteOmissionSemantics, ConsumerRouteValueDomain, ConsumerRouteValueKind,
};
use super::view::ConsumerRouteView;
use crate::{
    PreparedOperationEvidence, ProviderSessionCatalogueOutcome, ProviderSessionHistoryPage,
};

/// Borrowed successful outcome that may prove provider-operation observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConsumerRouteProviderOperationOutcome<'a> {
    /// One validated provider-session catalogue outcome.
    ProviderSessionCatalogue(&'a ProviderSessionCatalogueOutcome),
    /// One validated provider-session history page.
    ProviderSessionHistory(&'a ProviderSessionHistoryPage),
}

impl ConsumerRouteProviderOperationOutcome<'_> {
    const fn operation_shape(self) -> OperationShape {
        match self {
            Self::ProviderSessionCatalogue(_) => OperationShape::ProviderSessionCatalogue,
            Self::ProviderSessionHistory(_) => OperationShape::ProviderSessionHistory,
        }
    }

    fn matches_plan(self, evidence: &PreparedOperationEvidence) -> bool {
        match self {
            Self::ProviderSessionCatalogue(outcome) => {
                outcome.source_plan().preflight() == evidence.plan()
            }
            Self::ProviderSessionHistory(outcome) => {
                outcome.source_plan().preflight() == evidence.plan()
            }
        }
    }
}

/// Immutable observation rows from one completed provider operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsumerRouteProviderOperationObservation {
    applicability: ConsumerRouteApplicability,
    source: ConsumerRouteProjectionSourceIdentity,
    rows: Vec<ConsumerRouteProjectionRow>,
}

impl ConsumerRouteProviderOperationObservation {
    /// Admits rows proven by one matching completed provider operation.
    pub fn new(
        evidence: &PreparedOperationEvidence,
        outcome: ConsumerRouteProviderOperationOutcome<'_>,
        source: ConsumerRouteProjectionSourceIdentity,
        rows: impl IntoIterator<Item = ConsumerRouteProjectionRow>,
    ) -> Result<Self, ConsumerRouteProjectionFailure> {
        if source.kind() != ConsumerRouteProjectionSourceKind::ProviderOperationObservation {
            return Err(evidence_rejected());
        }
        let evidence_shape = evidence.plan().requirements().operation_shape();
        if !matches!(
            evidence_shape,
            OperationShape::ProviderSessionCatalogue | OperationShape::ProviderSessionHistory
        ) {
            return Err(failure(
                ConsumerRouteProjectionFailureKind::ProviderOperationObservationInvalid,
                "swallowtail.consumer_route_projection.provider_operation_shape_rejected",
                "Provider-operation observation requires an admitted completed operation shape",
            ));
        }
        if evidence_shape != outcome.operation_shape() || !outcome.matches_plan(evidence) {
            return Err(evidence_rejected());
        }

        let rows = rows.into_iter().collect::<Vec<_>>();
        if rows.len() > MAX_CONSUMER_ROUTE_PROVIDER_OPERATION_ROWS {
            return Err(failure(
                ConsumerRouteProjectionFailureKind::LimitExceeded,
                "swallowtail.consumer_route_projection.provider_operation_limit_exceeded",
                "Projected provider-operation state exceeds the fixed row maximum",
            ));
        }
        if rows.iter().any(|row| row.source() != &source) {
            return Err(failure(
                ConsumerRouteProjectionFailureKind::IdentityInvalid,
                "swallowtail.consumer_route_projection.row_source_unknown",
                "A projected row names a source the contribution did not supply",
            ));
        }
        if rows.iter().any(|row| !valid_observation_row(row)) {
            return Err(failure(
                ConsumerRouteProjectionFailureKind::ProviderOperationObservationInvalid,
                "swallowtail.consumer_route_projection.provider_operation_row_rejected",
                "Provider-operation observation row claims incompatible lifecycle or authority",
            ));
        }

        let applicability = ConsumerRouteApplicability::from_prepared_operation(evidence);
        let source_identities = admit_sources(std::slice::from_ref(&source))?;
        admit_view(
            ConsumerRouteView::ProviderOperation,
            &rows,
            &applicability,
            &source_identities,
        )?;
        admit_extension_budget(
            rows.iter(),
            "Provider-operation observation exceeds the fixed namespaced-extension maximum",
        )?;
        Ok(Self {
            applicability,
            source,
            rows,
        })
    }

    #[must_use]
    /// Returns the exact applicability derived from the prepared evidence.
    pub const fn applicability(&self) -> &ConsumerRouteApplicability {
        &self.applicability
    }

    #[must_use]
    /// Returns the independently replaceable completed-outcome source.
    pub const fn source(&self) -> &ConsumerRouteProjectionSourceIdentity {
        &self.source
    }

    /// Iterates the exact admitted provider-operation rows.
    pub fn rows(&self) -> impl ExactSizeIterator<Item = &ConsumerRouteProjectionRow> {
        self.rows.iter()
    }
}

fn valid_observation_row(row: &ConsumerRouteProjectionRow) -> bool {
    row.lifecycle() == ConsumerRouteLifecycle::PostOperationObservationOnly
        && row.source_class() == ConsumerRouteSourceClass::ProviderOperationOutcome
        && row.evidence_strength() == ConsumerRouteEvidenceStrength::CompletedProviderOperation
        && row.actor_posture() == ConsumerRouteActorPosture::ObservationOnly
        && row.state_support() == ConsumerRouteStateSupport::descriptor_only().with_observed()
        && matches!(
            row.mutation_authority(),
            ConsumerRouteMutationAuthority::Absent
        )
        && row.control_value().is_some_and(|value| {
            value.kind() == ConsumerRouteValueKind::BoundedQuery
                && matches!(value.domain(), ConsumerRouteValueDomain::Descriptor)
                && value.omission() == ConsumerRouteOmissionSemantics::NotSelectable
        })
}

fn evidence_rejected() -> ConsumerRouteProjectionFailure {
    failure(
        ConsumerRouteProjectionFailureKind::ProviderOperationObservationInvalid,
        "swallowtail.consumer_route_projection.provider_operation_evidence_rejected",
        "Provider-operation observation does not match its completed outcome evidence",
    )
}
