use swallowtail_core::{Capability, CapabilityManifest};
use swallowtail_testkit::{
    ContractKernelFixture, assert_capability_rejection, assert_contract_kernel,
    assert_diagnostic_redaction, assert_extension_isolation, assert_extension_policies,
    assert_reference_opacity,
};

#[test]
fn canonical_fixture_satisfies_contract_kernel() {
    assert_contract_kernel(&ContractKernelFixture::canonical());
}

#[test]
fn fixture_cases_are_independently_reusable() {
    let fixture = ContractKernelFixture::canonical();

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

#[test]
#[should_panic(expected = "Contract 003 capability rejection")]
fn assertion_failure_identifies_the_violated_contract_rule() {
    let manifest = CapabilityManifest::new([Capability::StructuredRun, Capability::Resume]);

    assert_capability_rejection(&manifest, Capability::StructuredRun, Capability::Resume);
}
