use super::{ProfilePreflightFixture, SyntheticProfile, valid};
use swallowtail_core::{
    AttachedModelObservation, AttachedModelObservationScope, AttachedModelTag,
    AttachedRuntimeRequirements, AttachedRuntimeResidency, CatalogTimestamp,
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis, InterfaceVersionBinding,
    InterfaceVersionScheme, InterfaceVersionSegment, ModelManifestDigest,
};

const DIGEST: &str = "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

impl ProfilePreflightFixture {
    pub(crate) fn attached_runtime() -> Self {
        let mut fixture = Self::new(SyntheticProfile::AttachedSelfHosted);
        fixture.set_attached_claim("fixture.attached-runtime.claim-1");
        let version = runtime_version("0.30.0");
        let tag = model_tag();
        let digest = manifest_digest();
        fixture.instance = fixture
            .instance
            .clone()
            .with_interface_versions([version.clone()]);
        fixture.requirements = fixture
            .requirements
            .clone()
            .with_interface_versions([version.clone()])
            .with_attached_runtime(AttachedRuntimeRequirements::new(
                version.clone(),
                fixture.route.model_id().clone(),
                tag.clone(),
                digest.clone(),
                AttachedRuntimeResidency::RuntimeManaged,
            ));
        fixture.attached_model_observation = Some(
            AttachedModelObservation::new(
                AttachedModelObservationScope::SelectedModelDetail,
                fixture.instance.id().clone(),
                fixture.instance.execution_host_id().clone(),
                version,
                CatalogTimestamp::new(1_700_000_000, 0).expect("timestamp is valid"),
                tag,
            )
            .with_manifest_digest(digest),
        );
        fixture
    }

    pub(crate) fn revise_attached_claim(&mut self) {
        self.set_attached_claim("fixture.attached-runtime.claim-2");
    }

    pub(crate) fn replace_attached_observation(&mut self, observation: AttachedModelObservation) {
        self.attached_model_observation = Some(observation);
    }

    pub(crate) const fn attached_observation(&self) -> Option<&AttachedModelObservation> {
        self.attached_model_observation.as_ref()
    }

    fn set_attached_claim(&mut self, id: &str) {
        self.driver = self
            .driver
            .clone()
            .with_interface_compatibility(runtime_claim(id));
    }
}

pub(crate) fn runtime_claim(id: &str) -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        valid(InterfaceCompatibilityClaimId::new, id),
        valid(InterfaceVersionAxis::new, "fixture.attached.runtime"),
        InterfaceVersionScheme::Semantic,
        [InterfaceVersionSegment::new(
            valid(InterfaceVersion::new, "0.14.0"),
            valid(InterfaceVersion::new, "0.32.1"),
            valid(InterfaceBehaviorRevision::new, "fixture.attached.native-v1"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("attached-runtime claim is valid")
}

pub(crate) fn runtime_version(version: &str) -> InterfaceVersionBinding {
    InterfaceVersionBinding::new(
        valid(InterfaceVersionAxis::new, "fixture.attached.runtime"),
        valid(InterfaceVersion::new, version),
    )
}

pub(crate) fn model_tag() -> AttachedModelTag {
    valid(AttachedModelTag::new, "fixture-model:8b")
}

pub(crate) fn manifest_digest() -> ModelManifestDigest {
    valid(ModelManifestDigest::new, DIGEST)
}
