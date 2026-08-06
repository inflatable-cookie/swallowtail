#![deny(missing_docs)]

use crate::AccessEvidenceSourceId;
use swallowtail_core::AccessStatus;

/// Provenance of access status supplied to prepared integration construction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AccessEvidenceProvenance {
    /// Status observed by the named safe host, provider, or consumer source.
    Observed(AccessEvidenceSourceId),
    /// Status explicitly asserted by the caller without observation promotion.
    CallerAsserted,
}

/// Exact access status paired with honest, non-authority-widening provenance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedAccessEvidence {
    status: AccessStatus,
    provenance: AccessEvidenceProvenance,
}

impl PreparedAccessEvidence {
    #[must_use]
    /// Records access status observed by one identified safe source.
    pub const fn observed(status: AccessStatus, source: AccessEvidenceSourceId) -> Self {
        Self {
            status,
            provenance: AccessEvidenceProvenance::Observed(source),
        }
    }

    #[must_use]
    /// Records access status explicitly asserted by the caller.
    pub const fn caller_asserted(status: AccessStatus) -> Self {
        Self {
            status,
            provenance: AccessEvidenceProvenance::CallerAsserted,
        }
    }

    #[must_use]
    /// Returns the supplied multidimensional access status unchanged.
    pub const fn status(&self) -> &AccessStatus {
        &self.status
    }

    #[must_use]
    /// Returns whether and where the access status was observed.
    pub const fn provenance(&self) -> &AccessEvidenceProvenance {
        &self.provenance
    }
}

#[cfg(test)]
mod tests {
    use super::{AccessEvidenceProvenance, PreparedAccessEvidence};
    use crate::AccessEvidenceSourceId;
    use swallowtail_core::{
        AccessProfileId, AccessStatus, CredentialState, EndpointAuthorization, EntitlementState,
        RuntimeReadiness, SupportAuthority,
    };

    fn status() -> AccessStatus {
        AccessStatus::new(
            AccessProfileId::new("fixture-access").expect("access id is valid"),
            CredentialState::Unknown,
            EntitlementState::Unknown,
            EndpointAuthorization::Unknown,
            RuntimeReadiness::Unknown,
            SupportAuthority::ExperimentalObserved,
        )
    }

    #[test]
    fn provenance_does_not_change_the_supplied_status() {
        let observed = PreparedAccessEvidence::observed(
            status(),
            AccessEvidenceSourceId::new("fixture-host-observation").expect("source id is valid"),
        );
        let asserted = PreparedAccessEvidence::caller_asserted(status());

        assert_eq!(observed.status(), asserted.status());
        assert!(matches!(
            observed.provenance(),
            AccessEvidenceProvenance::Observed(_)
        ));
        assert_eq!(
            asserted.provenance(),
            &AccessEvidenceProvenance::CallerAsserted
        );
        assert!(!format!("{observed:?}").contains("fixture-host-observation"));
    }
}
