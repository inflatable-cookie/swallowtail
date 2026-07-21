use std::error::Error;
use std::fmt;
use swallowtail_core::{
    ExternalNetworkPolicy, ExternalSearchPolicy, ReasoningMode, SafeDiagnostic,
};

/// Explicit policy selected for one operation. Catalog defaults do not populate it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationPolicy {
    external_network: ExternalNetworkPolicy,
    external_search: ExternalSearchPolicy,
    reasoning_mode: Option<ReasoningMode>,
}

impl OperationPolicy {
    pub fn new(
        external_network: ExternalNetworkPolicy,
        external_search: ExternalSearchPolicy,
    ) -> Result<Self, IncompatibleOperationPolicy> {
        if external_network == ExternalNetworkPolicy::AmbientHost {
            return Err(IncompatibleOperationPolicy::ambient_network_requires_harness());
        }
        if external_search == ExternalSearchPolicy::Enabled
            && external_network == ExternalNetworkPolicy::Denied
        {
            return Err(IncompatibleOperationPolicy::search_requires_network());
        }
        Ok(Self {
            external_network,
            external_search,
            reasoning_mode: None,
        })
    }

    #[must_use]
    pub fn offline() -> Self {
        Self::new(
            ExternalNetworkPolicy::Denied,
            ExternalSearchPolicy::Disabled,
        )
        .expect("offline operation policy is internally valid")
    }

    #[must_use]
    pub fn with_reasoning_mode(mut self, reasoning_mode: ReasoningMode) -> Self {
        self.reasoning_mode = Some(reasoning_mode);
        self
    }

    #[must_use]
    pub const fn external_network(&self) -> ExternalNetworkPolicy {
        self.external_network
    }

    #[must_use]
    pub const fn external_search(&self) -> ExternalSearchPolicy {
        self.external_search
    }

    #[must_use]
    pub const fn reasoning_mode(&self) -> Option<&ReasoningMode> {
        self.reasoning_mode.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IncompatibleOperationPolicy {
    diagnostic: SafeDiagnostic,
}

impl IncompatibleOperationPolicy {
    fn search_requires_network() -> Self {
        Self {
            diagnostic: SafeDiagnostic::new(
                "swallowtail.operation_policy_rejected",
                "External search requires host-approved external network access",
            ),
        }
    }

    fn ambient_network_requires_harness() -> Self {
        Self {
            diagnostic: SafeDiagnostic::new(
                "swallowtail.operation_policy_rejected",
                "Ambient host network authority is valid only for a harness session",
            ),
        }
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for IncompatibleOperationPolicy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for IncompatibleOperationPolicy {}

#[cfg(test)]
mod tests {
    use super::{ExternalNetworkPolicy, ExternalSearchPolicy, OperationPolicy};

    #[test]
    fn external_search_never_implies_network_authority() {
        let error =
            OperationPolicy::new(ExternalNetworkPolicy::Denied, ExternalSearchPolicy::Enabled)
                .expect_err("search without network authority must fail");

        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.operation_policy_rejected"
        );
    }

    #[test]
    fn ambient_network_authority_is_harness_only() {
        let error = OperationPolicy::new(
            ExternalNetworkPolicy::AmbientHost,
            ExternalSearchPolicy::Disabled,
        )
        .expect_err("direct operation policy must reject ambient host authority");

        assert_eq!(
            error.diagnostic().message(),
            "Ambient host network authority is valid only for a harness session"
        );
    }
}
