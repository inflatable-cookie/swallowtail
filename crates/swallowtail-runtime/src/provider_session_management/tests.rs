#[cfg(test)]
mod tests {
    use super::{InvalidProviderSessionManagementBindingKind, ProviderSessionManagementBinding};
    use crate::{AccessEvidenceSourceId, PreparedAccessEvidence, WorkingResourceRef};
    use swallowtail_core::{
        AccessProfileId, AccessStatus, AdapterId, AdapterIdentity, AdapterVersion, Capability,
        CapabilityProfile, CapabilityRequirement, ConfiguredInstance, ConfiguredInstanceId,
        CredentialState, DriverDescriptor, EndpointAuthorization, ExecutionHostId,
        InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef,
        IntegrationFamilyId, InterfaceBehaviorRevision, InterfaceCompatibilityClaim,
        InterfaceCompatibilityClaimId, InterfaceNewerVersionPosture, InterfaceSupportStatus,
        InterfaceVersion, InterfaceVersionAxis, InterfaceVersionBinding, InterfaceVersionScheme,
        InterfaceVersionSegment, ProtocolFacadeId, ProviderSessionBindingOrigin, RuntimeReadiness,
        SessionRef, SupportAuthority, TransportFamilyId,
    };

    fn fixture_driver() -> DriverDescriptor {
        let axis = InterfaceVersionAxis::new("fixture.rpc").expect("axis is valid");
        let version = InterfaceVersion::new("1.2.0").expect("version is valid");
        let claim = InterfaceCompatibilityClaim::new(
            InterfaceCompatibilityClaimId::new("fixture.rpc.support").expect("claim id is valid"),
            axis,
            InterfaceVersionScheme::Semantic,
            InterfaceNewerVersionPosture::QualifiedOnly,
            [InterfaceVersionSegment::exact(
                version,
                InterfaceBehaviorRevision::new("fixture-v1").expect("behavior revision is valid"),
                InterfaceSupportStatus::Maintained,
            )],
            [],
        )
        .expect("claim is valid");
        DriverDescriptor::new(
            AdapterIdentity::new(
                AdapterId::new("fixture.driver").expect("driver id is valid"),
                AdapterVersion::new("1.0.0").expect("driver version is valid"),
            ),
            IntegrationFamilyId::new("fixture").expect("family is valid"),
            TransportFamilyId::new("fixture-rpc").expect("transport is valid"),
        )
        .with_interface_compatibility(claim)
    }

    fn fixture_instance(driver_id: &str, access_profile_id: &str) -> ConfiguredInstance {
        ConfiguredInstance::new(
            ConfiguredInstanceId::new("fixture.instance").expect("instance id is valid"),
            InstanceRevision::new("revision-1").expect("revision is valid"),
            AdapterId::new(driver_id).expect("driver id is valid"),
            ExecutionHostId::new("fixture.host").expect("host id is valid"),
            InstanceTargetRef::new("private/service/target").expect("target is valid"),
            InstanceOwnership::ExternalAttached,
            AccessProfileId::new(access_profile_id).expect("access id is valid"),
            SupportAuthority::IntegrationMaintainerSupported,
            ProtocolFacadeId::new("fixture.facade").expect("facade is valid"),
            InstancePolicyId::new("fixture.policy").expect("policy is valid"),
            CapabilityProfile::new([
                CapabilityRequirement::new(Capability::ProviderSessionArchive, []),
                CapabilityRequirement::new(Capability::ProviderNativeSessionClose, []),
            ]),
        )
        .with_interface_versions([InterfaceVersionBinding::new(
            InterfaceVersionAxis::new("fixture.rpc").expect("axis is valid"),
            InterfaceVersion::new("1.2.0").expect("version is valid"),
        )])
    }

    fn fixture_access(profile_id: &str) -> PreparedAccessEvidence {
        PreparedAccessEvidence::observed(
            AccessStatus::new(
                AccessProfileId::new(profile_id).expect("access id is valid"),
                CredentialState::Ready,
                swallowtail_core::EntitlementState::Available,
                EndpointAuthorization::Allowed,
                RuntimeReadiness::Ready,
                SupportAuthority::IntegrationMaintainerSupported,
            ),
            AccessEvidenceSourceId::new("private/access/source").expect("source is valid"),
        )
    }

    fn binding() -> ProviderSessionManagementBinding {
        ProviderSessionManagementBinding::from_bound_session(
            SessionRef::new("provider/private/session").expect("session ref is valid"),
            &fixture_driver(),
            &fixture_instance("fixture.driver", "fixture.access"),
            fixture_access("fixture.access"),
            Some(WorkingResourceRef::new("private/workspace").expect("working resource is valid")),
            ProviderSessionBindingOrigin::Created,
        )
        .expect("binding is valid")
    }

    #[test]
    fn bound_session_carries_exact_route_and_independent_capabilities() {
        let binding = binding();

        assert_eq!(binding, binding.clone());
        assert_eq!(binding.transport_family().as_str(), "fixture-rpc");
        assert_eq!(binding.instance_revision().as_str(), "revision-1");
        assert_eq!(binding.interface_compatibility().len(), 1);
        assert!(binding.supports(Capability::ProviderSessionArchive));
        assert!(binding.supports(Capability::ProviderNativeSessionClose));
        assert!(!binding.supports(Capability::ProviderSessionRestore));
        assert!(!binding.supports(Capability::ProviderSessionDelete));
        assert!(binding.working_resource().is_some());
    }

    #[test]
    fn management_context_can_be_rejected_before_provider_session_work() {
        let driver = fixture_driver();
        let access = fixture_access("fixture.access");

        ProviderSessionManagementBinding::validate_bound_session_context(
            &driver,
            &fixture_instance("fixture.driver", "fixture.access"),
            &access,
        )
        .expect("matching context is valid");
        let failure = ProviderSessionManagementBinding::validate_bound_session_context(
            &driver,
            &fixture_instance("another.driver", "fixture.access"),
            &access,
        )
        .expect_err("driver mismatch fails before provider work");

        assert_eq!(
            failure.kind(),
            InvalidProviderSessionManagementBindingKind::DriverMismatch
        );
    }

    #[test]
    fn binding_rejects_driver_and_access_mismatch_without_provider_detail() {
        let session = SessionRef::new("provider/private/session").expect("session ref is valid");
        let driver_failure = ProviderSessionManagementBinding::from_bound_session(
            session.clone(),
            &fixture_driver(),
            &fixture_instance("another.driver", "fixture.access"),
            fixture_access("fixture.access"),
            None,
            ProviderSessionBindingOrigin::ExplicitlyImported,
        )
        .expect_err("driver mismatch must fail");
        let access_failure = ProviderSessionManagementBinding::from_bound_session(
            session,
            &fixture_driver(),
            &fixture_instance("fixture.driver", "fixture.access"),
            fixture_access("another.access"),
            None,
            ProviderSessionBindingOrigin::ExplicitlyImported,
        )
        .expect_err("access mismatch must fail");

        assert_eq!(
            driver_failure.kind(),
            InvalidProviderSessionManagementBindingKind::DriverMismatch
        );
        assert_eq!(
            access_failure.kind(),
            InvalidProviderSessionManagementBindingKind::AccessProfileMismatch
        );
        assert_eq!(
            driver_failure.diagnostic().code(),
            "swallowtail.provider_session_management_binding_invalid"
        );
        assert!(!format!("{driver_failure:?}").contains("provider/private/session"));
    }

    #[test]
    fn binding_rejects_missing_version_and_management_capability() {
        let driver = fixture_driver();
        let instance_without_version = ConfiguredInstance::new(
            ConfiguredInstanceId::new("fixture.instance").expect("instance id is valid"),
            InstanceRevision::new("revision-1").expect("revision is valid"),
            AdapterId::new("fixture.driver").expect("driver id is valid"),
            ExecutionHostId::new("fixture.host").expect("host id is valid"),
            InstanceTargetRef::new("private/service/target").expect("target is valid"),
            InstanceOwnership::ExternalAttached,
            AccessProfileId::new("fixture.access").expect("access id is valid"),
            SupportAuthority::IntegrationMaintainerSupported,
            ProtocolFacadeId::new("fixture.facade").expect("facade is valid"),
            InstancePolicyId::new("fixture.policy").expect("policy is valid"),
            CapabilityProfile::new([CapabilityRequirement::new(
                Capability::ProviderSessionArchive,
                [],
            )]),
        );
        let missing_version = ProviderSessionManagementBinding::from_bound_session(
            SessionRef::new("provider/session").expect("session ref is valid"),
            &driver,
            &instance_without_version,
            fixture_access("fixture.access"),
            None,
            ProviderSessionBindingOrigin::Loaded,
        )
        .expect_err("missing version must fail");

        let instance_without_management = ConfiguredInstance::new(
            ConfiguredInstanceId::new("fixture.instance").expect("instance id is valid"),
            InstanceRevision::new("revision-1").expect("revision is valid"),
            AdapterId::new("fixture.driver").expect("driver id is valid"),
            ExecutionHostId::new("fixture.host").expect("host id is valid"),
            InstanceTargetRef::new("private/service/target").expect("target is valid"),
            InstanceOwnership::ExternalAttached,
            AccessProfileId::new("fixture.access").expect("access id is valid"),
            SupportAuthority::IntegrationMaintainerSupported,
            ProtocolFacadeId::new("fixture.facade").expect("facade is valid"),
            InstancePolicyId::new("fixture.policy").expect("policy is valid"),
            CapabilityProfile::new([CapabilityRequirement::new(
                Capability::InteractiveSession,
                [],
            )]),
        )
        .with_interface_versions([InterfaceVersionBinding::new(
            InterfaceVersionAxis::new("fixture.rpc").expect("axis is valid"),
            InterfaceVersion::new("1.2.0").expect("version is valid"),
        )]);
        let missing_capability = ProviderSessionManagementBinding::from_bound_session(
            SessionRef::new("provider/session").expect("session ref is valid"),
            &driver,
            &instance_without_management,
            fixture_access("fixture.access"),
            None,
            ProviderSessionBindingOrigin::Loaded,
        )
        .expect_err("missing management capability must fail");

        assert_eq!(
            missing_version.kind(),
            InvalidProviderSessionManagementBindingKind::MissingInterfaceVersion
        );
        assert_eq!(
            missing_capability.kind(),
            InvalidProviderSessionManagementBindingKind::MissingManagementCapability
        );
    }

    #[test]
    fn debug_output_redacts_provider_target_resource_and_access_source() {
        let debug = format!("{:?}", binding());

        for private in [
            "provider/private/session",
            "private/service/target",
            "private/workspace",
            "private/access/source",
        ] {
            assert!(!debug.contains(private));
        }
    }
}
