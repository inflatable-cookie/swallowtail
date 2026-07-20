use crate::{ExtensionNamespace, SafeDiagnostic};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ResourceAccess {
    Read,
    ReadWrite,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ResourceRepresentation {
    Stream,
    BoundedBytes,
    TemporaryFile,
    Filesystem,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FilesystemBoundary {
    WorkingResource,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProviderApprovalPolicy {
    Never,
}

/// Permission for provider-side access beyond the selected provider route.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ExternalNetworkPolicy {
    Denied,
    HostApproved,
}

/// Whether the operation may ask the provider or harness to search externally.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ExternalSearchPolicy {
    Disabled,
    Enabled,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProviderRequestHandling {
    Reject,
    ObserveAndStop,
}

/// Declares the provider extensions which may be observed without granting authority.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProviderRequestPolicy {
    observe_and_stop: BTreeSet<ExtensionNamespace>,
}

impl ProviderRequestPolicy {
    #[must_use]
    pub fn reject_all() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn observe_and_stop(namespaces: impl IntoIterator<Item = ExtensionNamespace>) -> Self {
        Self {
            observe_and_stop: namespaces.into_iter().collect(),
        }
    }

    #[must_use]
    pub fn handling_for(&self, namespace: &ExtensionNamespace) -> ProviderRequestHandling {
        if self.observe_and_stop.contains(namespace) {
            ProviderRequestHandling::ObserveAndStop
        } else {
            ProviderRequestHandling::Reject
        }
    }

    pub fn observed_extensions(&self) -> impl ExactSizeIterator<Item = &ExtensionNamespace> {
        self.observe_and_stop.iter()
    }
}

/// Expanded access policy for one interactive session.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionAccessPolicy {
    resource_access: ResourceAccess,
    filesystem_boundary: FilesystemBoundary,
    approval_policy: ProviderApprovalPolicy,
    external_network: ExternalNetworkPolicy,
    external_search: ExternalSearchPolicy,
    provider_requests: ProviderRequestPolicy,
}

impl SessionAccessPolicy {
    pub fn new(
        resource_access: ResourceAccess,
        filesystem_boundary: FilesystemBoundary,
        approval_policy: ProviderApprovalPolicy,
        external_network: ExternalNetworkPolicy,
        external_search: ExternalSearchPolicy,
        provider_requests: ProviderRequestPolicy,
    ) -> Result<Self, IncompatibleSessionAccessPolicy> {
        if external_search == ExternalSearchPolicy::Enabled
            && external_network == ExternalNetworkPolicy::Denied
        {
            return Err(IncompatibleSessionAccessPolicy::search_requires_network());
        }
        Ok(Self {
            resource_access,
            filesystem_boundary,
            approval_policy,
            external_network,
            external_search,
            provider_requests,
        })
    }

    #[must_use]
    pub fn read_only() -> Self {
        Self::new(
            ResourceAccess::Read,
            FilesystemBoundary::WorkingResource,
            ProviderApprovalPolicy::Never,
            ExternalNetworkPolicy::Denied,
            ExternalSearchPolicy::Disabled,
            ProviderRequestPolicy::reject_all(),
        )
        .expect("read-only session access policy is internally valid")
    }

    #[must_use]
    pub fn bounded_workspace(
        observed_provider_requests: impl IntoIterator<Item = ExtensionNamespace>,
    ) -> Self {
        Self::new(
            ResourceAccess::ReadWrite,
            FilesystemBoundary::WorkingResource,
            ProviderApprovalPolicy::Never,
            ExternalNetworkPolicy::Denied,
            ExternalSearchPolicy::Disabled,
            ProviderRequestPolicy::observe_and_stop(observed_provider_requests),
        )
        .expect("bounded workspace session access policy is internally valid")
    }

    #[must_use]
    pub const fn resource_access(&self) -> ResourceAccess {
        self.resource_access
    }

    #[must_use]
    pub const fn filesystem_boundary(&self) -> FilesystemBoundary {
        self.filesystem_boundary
    }

    #[must_use]
    pub const fn approval_policy(&self) -> ProviderApprovalPolicy {
        self.approval_policy
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
    pub const fn provider_requests(&self) -> &ProviderRequestPolicy {
        &self.provider_requests
    }
}

impl Default for SessionAccessPolicy {
    fn default() -> Self {
        Self::read_only()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IncompatibleSessionAccessPolicy {
    diagnostic: SafeDiagnostic,
}

impl IncompatibleSessionAccessPolicy {
    fn search_requires_network() -> Self {
        Self {
            diagnostic: SafeDiagnostic::new(
                "swallowtail.session_access_policy_rejected",
                "External search requires host-approved external network access",
            ),
        }
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for IncompatibleSessionAccessPolicy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for IncompatibleSessionAccessPolicy {}

#[cfg(test)]
mod tests {
    use super::{
        ExternalNetworkPolicy, ExternalSearchPolicy, FilesystemBoundary, ProviderApprovalPolicy,
        ProviderRequestHandling, ProviderRequestPolicy, ResourceAccess, SessionAccessPolicy,
    };
    use crate::ExtensionNamespace;

    #[test]
    fn read_only_is_the_expanded_default() {
        let policy = SessionAccessPolicy::default();

        assert_eq!(policy.resource_access(), ResourceAccess::Read);
        assert_eq!(
            policy.filesystem_boundary(),
            FilesystemBoundary::WorkingResource
        );
        assert_eq!(policy.approval_policy(), ProviderApprovalPolicy::Never);
        assert_eq!(policy.external_network(), ExternalNetworkPolicy::Denied);
        assert_eq!(policy.external_search(), ExternalSearchPolicy::Disabled);
        assert_eq!(policy.provider_requests().observed_extensions().len(), 0);
    }

    #[test]
    fn observed_provider_requests_are_explicit_and_default_to_reject() {
        let approval = ExtensionNamespace::new("example/approval").expect("namespace is valid");
        let unknown = ExtensionNamespace::new("example/unknown").expect("namespace is valid");
        let policy = ProviderRequestPolicy::observe_and_stop([approval.clone()]);

        assert_eq!(
            policy.handling_for(&approval),
            ProviderRequestHandling::ObserveAndStop
        );
        assert_eq!(
            policy.handling_for(&unknown),
            ProviderRequestHandling::Reject
        );
    }

    #[test]
    fn external_search_cannot_imply_network_authority() {
        let error = SessionAccessPolicy::new(
            ResourceAccess::Read,
            FilesystemBoundary::WorkingResource,
            ProviderApprovalPolicy::Never,
            ExternalNetworkPolicy::Denied,
            ExternalSearchPolicy::Enabled,
            ProviderRequestPolicy::reject_all(),
        )
        .expect_err("search without network authority must fail");

        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.session_access_policy_rejected"
        );
    }
}
