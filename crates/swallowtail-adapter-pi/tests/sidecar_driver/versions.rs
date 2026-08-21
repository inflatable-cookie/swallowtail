use super::{driver, make_host_id};
use crate::support::{
    SidecarFixtureHost, SidecarFixtureSelection, SidecarScenario, sidecar_open_request,
    sidecar_selection, sidecar_selection_with_instance_versions, sidecar_versions,
};
use futures_executor::block_on;
use swallowtail_adapter_pi::{
    PI_SDK_SIDECAR_NODE_AXIS, PI_SDK_SIDECAR_PACKAGE_AXIS, PI_SDK_SIDECAR_SIDECAR_AXIS,
    PI_SDK_SIDECAR_WIRE_AXIS,
};
use swallowtail_core::{InterfaceVersion, InterfaceVersionAxis, InterfaceVersionBinding};
use swallowtail_runtime::{InteractiveSessionDriver, RequestId};

#[test]
fn missing_ambiguous_or_incompatible_version_bindings_fail_before_process_work() {
    // Missing the wire axis entirely.
    let missing: Vec<_> = sidecar_versions()
        .into_iter()
        .filter(|binding| binding.axis().as_str() != PI_SDK_SIDECAR_WIRE_AXIS)
        .collect();
    let error = version_case(missing);
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.sdk-sidecar.version_missing"
    );

    // Two bindings on the package axis.
    let mut ambiguous = sidecar_versions().to_vec();
    ambiguous.push(binding(PI_SDK_SIDECAR_PACKAGE_AXIS, "0.84.2"));
    ambiguous.push(binding(PI_SDK_SIDECAR_PACKAGE_AXIS, "0.84.1"));
    let error = version_case(ambiguous);
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.sdk-sidecar.version_ambiguous"
    );

    // One off-point value per axis.
    for (axis, value) in [
        (PI_SDK_SIDECAR_PACKAGE_AXIS, "0.84.3"),
        (PI_SDK_SIDECAR_PACKAGE_AXIS, "0.84.1"),
        (PI_SDK_SIDECAR_PACKAGE_AXIS, "0.84.2-rc.1"),
        (PI_SDK_SIDECAR_NODE_AXIS, "22.23.3"),
        (PI_SDK_SIDECAR_NODE_AXIS, "22.23.2-rc.1"),
        (PI_SDK_SIDECAR_WIRE_AXIS, "swallowtail-pi-sdk-jsonl-v2"),
        (
            PI_SDK_SIDECAR_SIDECAR_AXIS,
            "swallowtail-pi-sdk-sidecar@0.0.0",
        ),
    ] {
        let versions: Vec<_> = sidecar_versions()
            .into_iter()
            .map(|existing| {
                if existing.axis().as_str() == axis {
                    binding(axis, value)
                } else {
                    existing
                }
            })
            .collect();
        let error = version_case(versions);
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.pi.sdk-sidecar.version_incompatible",
            "{axis} {value} must be rejected"
        );
    }
}

#[test]
fn unbound_or_mismatched_resume_bindings_fail_before_process_work() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.resume-mismatch");
    let fixture = SidecarFixtureHost::new(SidecarScenario::Complete);
    let selected = sidecar_selection(host_id.clone());
    // Same route dimensions but a different configured instance: the binding
    // no longer matches the plan.
    let binding = swallowtail_runtime::SessionResumeBinding::new(
        swallowtail_core::SessionRef::new("pi-sidecar-session-fixture").expect("valid session ref"),
        swallowtail_core::ConfiguredInstanceId::new("pi.fixture.other-instance")
            .expect("valid instance"),
        selected.plan.execution_host_id().clone(),
        selected.plan.model_route_id().expect("model route").clone(),
        selected.plan.model_id().expect("model").clone(),
        selected.resource.clone(),
        swallowtail_core::SessionAccessPolicy::ambient_harness(
            swallowtail_core::ResourceAccess::Read,
        ),
    );
    let error = block_on(driver(selected.credential.clone()).resume_session(
        selected.plan,
        swallowtail_runtime::ResumeSessionRequest::new(
            RequestId::new("sidecar-resume-mismatch").expect("valid request"),
            binding,
            selected.resource.clone(),
            None,
            swallowtail_runtime::SessionPlanAgreement::explicit(
                swallowtail_core::SessionAccessPolicy::ambient_harness(
                    swallowtail_core::ResourceAccess::Read,
                ),
                Some(swallowtail_core::SessionProviderStatePolicy::DurableProviderSessionPreserved),
                Some(swallowtail_core::HarnessConfigurationPosture::ProviderSuppressed),
            ),
        ),
        fixture.services(host_id),
    ))
    .err()
    .expect("mismatched binding fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.sdk-sidecar.resume_binding_mismatch"
    );
    assert_eq!(fixture.credential_acquisitions(), 0);
    assert!(!fixture.process_started());
}

fn version_case(versions: Vec<InterfaceVersionBinding>) -> swallowtail_runtime::RuntimeFailure {
    let host_id_value = format!("pi.fixture.sdk-sidecar.version-{}", versions.len());
    let host_id = make_host_id(&host_id_value);
    let fixture = SidecarFixtureHost::new(SidecarScenario::Complete);
    let selected: SidecarFixtureSelection =
        sidecar_selection_with_instance_versions(host_id.clone(), versions);
    let error = block_on(driver(selected.credential.clone()).open_session(
        selected.plan,
        sidecar_open_request("sidecar-version-fail", selected.resource),
        fixture.services(host_id),
    ))
    .err()
    .expect("version binding rejection fails");
    assert_eq!(fixture.credential_acquisitions(), 0);
    assert!(!fixture.process_started());
    error
}

fn binding(axis: &str, value: &str) -> InterfaceVersionBinding {
    InterfaceVersionBinding::new(
        InterfaceVersionAxis::new(axis).expect("valid axis"),
        InterfaceVersion::new(value).expect("valid version"),
    )
}
