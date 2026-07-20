use crate::ContractKernelFixture;
use swallowtail_core::{
    Capability, CapabilityManifest, Diagnostic, EventEnvelope, ExtensionPolicy, RunRef, SessionRef,
};

const CAPABILITY_RULE: &str = "Contract 003 capability rejection";
const REFERENCE_RULE: &str = "Contract 003 provider-reference opacity";
const DIAGNOSTIC_RULE: &str = "Contract 003 diagnostic redaction";
const EXTENSION_RULE: &str = "Contract 003 provider-extension isolation";
const EXTENSION_POLICY_RULE: &str = "Contract 003 unknown-extension policy";

/// Runs every pure Contract 003 assertion against one fixture set.
pub fn assert_contract_kernel(fixture: &ContractKernelFixture) {
    assert_capability_rejection(
        fixture.capabilities(),
        fixture.supported_capability(),
        fixture.unsupported_capability(),
    );
    assert_reference_opacity(fixture.session_ref(), fixture.run_ref());
    assert_diagnostic_redaction(fixture.diagnostic());
    assert_extension_isolation(fixture.event_with_extension());
    assert_extension_policies(fixture.event_with_extension());
}

pub fn assert_capability_rejection(
    manifest: &CapabilityManifest,
    supported: Capability,
    unsupported: Capability,
) {
    assert!(
        manifest.require(supported).is_ok(),
        "{CAPABILITY_RULE}: declared capability {supported:?} was rejected"
    );

    let rejection = manifest
        .require(unsupported)
        .expect_err("Contract 003 capability rejection: undeclared capability was accepted");
    assert_eq!(
        rejection.capability(),
        unsupported,
        "{CAPABILITY_RULE}: rejection identified the wrong capability"
    );
    assert_eq!(
        rejection.diagnostic().code(),
        "swallowtail.unsupported_capability",
        "{CAPABILITY_RULE}: rejection used the wrong diagnostic code"
    );
}

pub fn assert_reference_opacity(session_ref: &SessionRef, run_ref: &RunRef) {
    let session_debug = format!("{session_ref:?}");
    let run_debug = format!("{run_ref:?}");

    assert!(
        !session_debug.contains(session_ref.as_provider_value()),
        "{REFERENCE_RULE}: SessionRef debug output exposed its provider value"
    );
    assert!(
        !run_debug.contains(run_ref.as_provider_value()),
        "{REFERENCE_RULE}: RunRef debug output exposed its provider value"
    );
    assert!(
        session_debug.contains("<opaque>") && run_debug.contains("<opaque>"),
        "{REFERENCE_RULE}: default formatting did not mark references opaque"
    );
}

pub fn assert_diagnostic_redaction(diagnostic: &Diagnostic) {
    let internal_detail = diagnostic
        .internal_detail()
        .unwrap_or_else(|| panic!("{DIAGNOSTIC_RULE}: fixture must contain internal detail"));

    assert!(
        !diagnostic.to_string().contains(internal_detail),
        "{DIAGNOSTIC_RULE}: Display exposed internal detail"
    );
    assert!(
        !format!("{diagnostic:?}").contains(internal_detail),
        "{DIAGNOSTIC_RULE}: Debug exposed internal detail"
    );
    assert!(
        diagnostic.to_string().contains(diagnostic.safe().message()),
        "{DIAGNOSTIC_RULE}: Display omitted the safe message"
    );
}

pub fn assert_extension_isolation(event: &EventEnvelope) {
    let extension = event
        .extension()
        .unwrap_or_else(|| panic!("{EXTENSION_RULE}: fixture must contain an extension"));
    let rendered = format!("{event:?}");
    let byte_rendering = format!("{:?}", extension.payload());

    assert!(
        !rendered.contains(&byte_rendering),
        "{EXTENSION_RULE}: Debug exposed extension bytes"
    );
    if let Ok(text) = std::str::from_utf8(extension.payload()) {
        assert!(
            !rendered.contains(text),
            "{EXTENSION_RULE}: Debug exposed extension text"
        );
    }
    assert!(
        rendered.contains(extension.namespace().as_str()),
        "{EXTENSION_RULE}: Debug lost the provider namespace"
    );
}

pub fn assert_extension_policies(event: &EventEnvelope) {
    let extension = event
        .extension()
        .unwrap_or_else(|| panic!("{EXTENSION_POLICY_RULE}: fixture must contain an extension"));

    let preserved = event
        .clone()
        .apply_extension_policy(ExtensionPolicy::Preserve)
        .unwrap_or_else(|error| {
            panic!("{EXTENSION_POLICY_RULE}: preserve policy rejected extension: {error}")
        });
    assert_eq!(
        preserved, *event,
        "{EXTENSION_POLICY_RULE}: preserve policy changed the event"
    );

    let rejection = event
        .clone()
        .apply_extension_policy(ExtensionPolicy::Reject)
        .expect_err("Contract 003 unknown-extension policy: reject policy accepted extension");
    assert_eq!(
        rejection.namespace(),
        extension.namespace(),
        "{EXTENSION_POLICY_RULE}: rejection identified the wrong namespace"
    );
    assert_eq!(
        rejection.diagnostic().code(),
        "swallowtail.provider_extension_rejected",
        "{EXTENSION_POLICY_RULE}: rejection used the wrong diagnostic code"
    );
}
