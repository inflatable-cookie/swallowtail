use crate::profile_fixture::attached_runtime::{runtime_claim, runtime_version};
use crate::{
    ClosedSemanticWindowCase, ConformanceAssertion, ConformanceReport, ProfilePreflightFixture,
    assert_closed_semantic_compatibility_window,
};
use swallowtail_core::{
    AttachedModelObservation, AttachedModelObservationScope, AttachedRuntimeResidency,
    InterfaceVersion, PreflightDimension,
};
use swallowtail_runtime::{OperationPolicy, validate_attached_runtime_residency_policy};

pub(crate) fn run() -> ConformanceReport {
    let mut report = crate::profile_attached::run();
    assert_version_window();
    assert_exact_binding_and_residency();
    assert_mismatched_observation_rejected();
    assert_stale_claim_rejected();
    report.record(ConformanceAssertion::AttachedRuntimeBinding);
    report.record(ConformanceAssertion::RuntimeManagedResidency);
    report.record(ConformanceAssertion::ClosedCompatibilityWindow);
    report
}

fn assert_version_window() {
    let claim = runtime_claim("fixture.attached-runtime.claim-window");
    let case = ClosedSemanticWindowCase::new(version("0.14.0"), version("0.32.1"))
        .with_accepted([version("0.18.0"), version("0.30.0")])
        .with_rejected([
            version("0.13.5"),
            version("0.18.0-rc.1"),
            version("0.32.2"),
            version("0.32.3-rc.0"),
        ]);
    assert_closed_semantic_compatibility_window(&claim, &case);
}

fn assert_exact_binding_and_residency() {
    let fixture = ProfilePreflightFixture::attached_runtime();
    let plan = fixture
        .preflight()
        .expect("attached-runtime preflight passes");
    let observed = plan
        .attached_model_observation()
        .expect("selected-model detail is bound");
    assert_eq!(
        observed.scope(),
        AttachedModelObservationScope::SelectedModelDetail
    );
    assert_eq!(observed.runtime_version(), &runtime_version("0.30.0"));
    assert_ne!(
        observed.model_tag().as_str(),
        plan.model_id().unwrap().as_str()
    );
    assert!(observed.manifest_digest().is_some());

    let accepted = OperationPolicy::offline()
        .with_attached_runtime_residency(AttachedRuntimeResidency::RuntimeManaged);
    validate_attached_runtime_residency_policy(&plan, &accepted)
        .expect("request accepts preflight-bound residency");
    assert!(
        validate_attached_runtime_residency_policy(&plan, &OperationPolicy::offline()).is_err()
    );
}

fn assert_mismatched_observation_rejected() {
    let mut fixture = ProfilePreflightFixture::attached_runtime();
    let observed = fixture
        .attached_observation()
        .cloned()
        .expect("fixture has selected-model detail");
    fixture.replace_attached_observation(
        AttachedModelObservation::new(
            AttachedModelObservationScope::RunningInventory,
            observed.instance_id().clone(),
            observed.execution_host_id().clone(),
            observed.runtime_version().clone(),
            observed.observed_at(),
            observed.model_tag().clone(),
        )
        .with_manifest_digest(observed.manifest_digest().unwrap().clone()),
    );
    let failure = fixture
        .preflight()
        .expect_err("running inventory cannot replace selected-model detail");
    assert_eq!(failure.dimension(), PreflightDimension::AttachedRuntime);
}

fn assert_stale_claim_rejected() {
    let mut fixture = ProfilePreflightFixture::attached_runtime();
    let plan = fixture.preflight().expect("initial preflight passes");
    fixture.revise_attached_claim();
    assert!(plan.validate_current(&fixture.context()).is_err());
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::ConformanceAssertion;

    #[test]
    fn attached_runtime_pack_is_additive_to_the_existing_profile() {
        let report = run();
        assert!(report.covers(ConformanceAssertion::AttachedServiceNeverStopped));
        assert!(report.covers(ConformanceAssertion::AttachedRuntimeBinding));
        assert!(report.covers(ConformanceAssertion::RuntimeManagedResidency));
        assert!(report.covers(ConformanceAssertion::ClosedCompatibilityWindow));
    }
}
