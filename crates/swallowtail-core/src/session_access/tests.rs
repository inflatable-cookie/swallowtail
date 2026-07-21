use super::{
    ExternalNetworkPolicy, ExternalSearchPolicy, FilesystemBoundary, HarnessIsolation,
    ProviderApprovalPolicy, ProviderRequestHandling, ProviderRequestPolicy, ResourceAccess,
    SessionAccessPolicy,
};
use crate::ExtensionNamespace;

#[test]
fn ambient_read_is_the_expanded_default() {
    let policy = SessionAccessPolicy::default();
    assert_eq!(policy.resource_access(), Some(ResourceAccess::Read));
    assert_eq!(policy.filesystem_boundary(), None);
    assert_eq!(
        policy.harness_isolation(),
        Some(HarnessIsolation::AmbientHost)
    );
    assert_eq!(policy.approval_policy(), ProviderApprovalPolicy::Never);
    assert_eq!(
        policy.external_network(),
        ExternalNetworkPolicy::AmbientHost
    );
    assert_eq!(policy.external_search(), ExternalSearchPolicy::Disabled);
    assert_eq!(policy.provider_requests().observed_extensions().len(), 0);
}

#[test]
fn resource_free_policy_represents_absence_directly() {
    let policy = SessionAccessPolicy::resource_free();
    assert_eq!(policy.resource_access(), None);
    assert_eq!(policy.filesystem_boundary(), None);
    assert_eq!(policy.harness_isolation(), None);
    assert_eq!(policy.approval_policy(), ProviderApprovalPolicy::Never);
    assert_eq!(policy.external_network(), ExternalNetworkPolicy::Denied);
    assert_eq!(policy.external_search(), ExternalSearchPolicy::Disabled);
}

#[test]
fn ambient_harness_declares_no_filesystem_boundary() {
    let policy = SessionAccessPolicy::ambient_harness(ResourceAccess::ReadWrite);
    assert_eq!(policy.resource_access(), Some(ResourceAccess::ReadWrite));
    assert_eq!(policy.filesystem_boundary(), None);
    assert_eq!(
        policy.harness_isolation(),
        Some(HarnessIsolation::AmbientHost)
    );
    assert_eq!(
        policy.external_network(),
        ExternalNetworkPolicy::AmbientHost
    );
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
        HarnessIsolation::ProviderEnforced,
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

#[test]
fn ambient_harness_cannot_claim_a_bounded_filesystem() {
    let error = SessionAccessPolicy::new(
        ResourceAccess::Read,
        FilesystemBoundary::WorkingResource,
        HarnessIsolation::AmbientHost,
        ProviderApprovalPolicy::Never,
        ExternalNetworkPolicy::AmbientHost,
        ExternalSearchPolicy::Disabled,
        ProviderRequestPolicy::reject_all(),
    )
    .expect_err("ambient execution must not claim containment");
    assert_eq!(
        error.diagnostic().message(),
        "Ambient harness execution cannot claim a bounded filesystem"
    );
}
