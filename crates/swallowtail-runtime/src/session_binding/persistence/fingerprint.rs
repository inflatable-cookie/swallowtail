use super::{SessionResumeBindingPersistenceFailure, attachment_mismatch};
use crate::WorkingResourceRef;
use sha2::{Digest, Sha256};
use swallowtail_core::{
    CredentialMechanism, ExternalNetworkPolicy, ExternalSearchPolicy, FilesystemBoundary,
    HarnessConfigurationPosture, HarnessIsolation, PreflightPlan, ProviderApprovalPolicy,
    ProviderRequestPolicy, ResourceAccess, SessionAccessPolicy, SessionProviderStatePolicy,
};

pub(super) fn attachment_fingerprint(
    plan: &PreflightPlan,
    working_resource: &WorkingResourceRef,
    access_policy: &SessionAccessPolicy,
) -> Result<[u8; 32], SessionResumeBindingPersistenceFailure> {
    let model = match (
        plan.model_route_id(),
        plan.model_route_revision(),
        plan.model_id(),
    ) {
        (Some(route), Some(revision), Some(model)) => Some((route, revision, model)),
        (None, None, None) => None,
        _ => return Err(attachment_mismatch()),
    };
    let mut digest = Sha256::new();
    digest.update(if model.is_some() {
        b"swallowtail.session-resume-binding.attachment.v1".as_slice()
    } else {
        b"swallowtail.session-resume-binding.attachment.model-less.v1".as_slice()
    });
    hash_text(&mut digest, plan.driver_identity().id().as_str());
    hash_text(&mut digest, plan.driver_identity().version().as_str());
    hash_text(&mut digest, plan.integration_family().as_str());
    hash_text(&mut digest, plan.transport_family().as_str());
    hash_text(&mut digest, plan.instance_id().as_str());
    hash_text(&mut digest, plan.instance_revision().as_str());
    hash_text(&mut digest, plan.instance_target_ref().as_host_value());
    hash_text(&mut digest, plan.execution_host_id().as_str());
    hash_text(&mut digest, plan.protocol_facade_id().as_str());
    hash_text(&mut digest, plan.instance_policy_id().as_str());
    hash_text(&mut digest, plan.access_profile_id().as_str());
    hash_credential_mechanism(&mut digest, plan.credential_mechanism());
    hash_text(&mut digest, plan.endpoint_audience().as_str());
    if let Some((route, route_revision, model)) = model {
        hash_text(&mut digest, route.as_str());
        hash_text(&mut digest, route_revision.as_str());
        hash_text(&mut digest, model.as_str());
    }
    hash_optional_text(&mut digest, plan.provider_id().map(|value| value.as_str()));
    let mut versions = plan
        .interface_versions()
        .map(|version| (version.axis().as_str(), version.version().as_str()))
        .collect::<Vec<_>>();
    versions.sort_unstable();
    hash_usize(&mut digest, versions.len());
    for (axis, version) in versions {
        hash_text(&mut digest, axis);
        hash_text(&mut digest, version);
    }
    hash_text(&mut digest, working_resource.as_host_value());
    hash_access_policy(&mut digest, access_policy);
    hash_optional_session_state(
        &mut digest,
        plan.requirements().session_provider_state_policy(),
    );
    hash_optional_harness_configuration(&mut digest, plan.harness_configuration_posture());
    Ok(digest.finalize().into())
}

fn hash_access_policy(digest: &mut Sha256, policy: &SessionAccessPolicy) {
    digest.update([option_resource_access(policy.resource_access())]);
    digest.update([option_filesystem_boundary(policy.filesystem_boundary())]);
    digest.update([option_harness_isolation(policy.harness_isolation())]);
    digest.update([match policy.approval_policy() {
        ProviderApprovalPolicy::Never => 0,
        ProviderApprovalPolicy::ConsumerMediated => 1,
    }]);
    digest.update([match policy.external_network() {
        ExternalNetworkPolicy::Denied => 0,
        ExternalNetworkPolicy::HostApproved => 1,
        ExternalNetworkPolicy::AmbientHost => 2,
    }]);
    digest.update([match policy.external_search() {
        ExternalSearchPolicy::Disabled => 0,
        ExternalSearchPolicy::Enabled => 1,
    }]);
    hash_provider_request_policy(digest, policy.provider_requests());
}

fn hash_provider_request_policy(digest: &mut Sha256, policy: &ProviderRequestPolicy) {
    hash_usize(digest, policy.observed_extensions().len());
    for namespace in policy.observed_extensions() {
        hash_text(digest, namespace.as_str());
    }
    hash_usize(digest, policy.exchanged_extensions().len());
    for namespace in policy.exchanged_extensions() {
        hash_text(digest, namespace.as_str());
    }
}

fn hash_credential_mechanism(digest: &mut Sha256, mechanism: &CredentialMechanism) {
    let (tag, extension) = match mechanism {
        CredentialMechanism::InteractiveOauth => (0, None),
        CredentialMechanism::DeviceOauth => (1, None),
        CredentialMechanism::AutomationToken => (2, None),
        CredentialMechanism::ApiKey => (3, None),
        CredentialMechanism::WorkloadIdentity => (4, None),
        CredentialMechanism::CloudProviderIdentity => (5, None),
        CredentialMechanism::GatewayHelper => (6, None),
        CredentialMechanism::Unauthenticated => (7, None),
        CredentialMechanism::LocalUnauthenticated => (8, None),
        CredentialMechanism::ProviderSpecific(namespace) => (9, Some(namespace.as_str())),
    };
    digest.update([tag]);
    hash_optional_text(digest, extension);
}

fn hash_optional_session_state(digest: &mut Sha256, policy: Option<SessionProviderStatePolicy>) {
    digest.update([match policy {
        None => 0,
        Some(SessionProviderStatePolicy::Prohibited) => 1,
        Some(SessionProviderStatePolicy::DurableProviderSessionPreserved) => 2,
        Some(SessionProviderStatePolicy::DurableConversationDeleteOnClose) => 3,
    }]);
}

fn hash_optional_harness_configuration(
    digest: &mut Sha256,
    posture: Option<HarnessConfigurationPosture>,
) {
    digest.update([match posture {
        None => 0,
        Some(HarnessConfigurationPosture::Ambient) => 1,
        Some(HarnessConfigurationPosture::ProviderSuppressed) => 2,
        Some(HarnessConfigurationPosture::HostScoped) => 3,
    }]);
}

fn option_resource_access(value: Option<ResourceAccess>) -> u8 {
    match value {
        None => 0,
        Some(ResourceAccess::Read) => 1,
        Some(ResourceAccess::ReadWrite) => 2,
    }
}

fn option_filesystem_boundary(value: Option<FilesystemBoundary>) -> u8 {
    match value {
        None => 0,
        Some(FilesystemBoundary::WorkingResource) => 1,
    }
}

fn option_harness_isolation(value: Option<HarnessIsolation>) -> u8 {
    match value {
        None => 0,
        Some(HarnessIsolation::AmbientHost) => 1,
        Some(HarnessIsolation::ProviderEnforced) => 2,
        Some(HarnessIsolation::HostEnforced) => 3,
    }
}

fn hash_text(digest: &mut Sha256, value: &str) {
    hash_usize(digest, value.len());
    digest.update(value.as_bytes());
}

fn hash_optional_text(digest: &mut Sha256, value: Option<&str>) {
    match value {
        Some(value) => {
            digest.update([1]);
            hash_text(digest, value);
        }
        None => digest.update([0]),
    }
}

fn hash_usize(digest: &mut Sha256, value: usize) {
    digest.update(u64::try_from(value).unwrap_or(u64::MAX).to_be_bytes());
}
