use crate::{ObservableActivityFixtureCase, ObservableActivityTraceFixture};
use swallowtail_core::{
    Capability, DriverRole, ObservableActivityAvailability, ObservableActivityProfile,
    OperationShape,
};
use swallowtail_runtime::{PreparedOperationEvidence, RuntimeEvent};

mod contract;
mod evidence;
mod trace;

/// Asserts that one adapter-projected trace stays within its immutable route profile.
pub fn assert_observable_activity_trace(
    profile: &ObservableActivityProfile,
    events: &[RuntimeEvent],
) {
    if let Err(message) = trace::validate(profile, events) {
        panic!("observable activity trace is not conformant: {message}");
    }
}

/// Asserts that a prepared non-agent operation cannot advertise ordinary activity.
pub fn assert_observable_activity_not_applicable(evidence: &PreparedOperationEvidence) {
    let profile = evidence.observable_activity();
    assert_eq!(
        profile.availability(),
        ObservableActivityAvailability::NotApplicable
    );
    assert_eq!(profile.interface_basis().count(), 0);
    assert_eq!(profile.kinds().count(), 0);
    assert!(profile.capability_requirement().is_none());
    assert!(
        !evidence
            .plan()
            .requirements()
            .capabilities()
            .any(|requirement| requirement.capability() == Capability::ObservableActivity)
    );
    assert!(!matches!(
        (
            evidence.binding().operation_shape(),
            evidence.binding().driver_role()
        ),
        (OperationShape::StructuredRun, DriverRole::StructuredRun)
            | (
                OperationShape::InteractiveSession,
                DriverRole::InteractiveSession
            )
    ));
}

/// Runs the provider-neutral observable-activity conformance pack.
pub fn assert_observable_activity_contract() {
    for case in [
        ObservableActivityFixtureCase::CompleteLifecycle,
        ObservableActivityFixtureCase::UpdateAndCompletion,
        ObservableActivityFixtureCase::CompletionOnly,
        ObservableActivityFixtureCase::Unavailable,
        ObservableActivityFixtureCase::CallbackCorrelation,
        ObservableActivityFixtureCase::DirectToolCorrelation,
        ObservableActivityFixtureCase::IntermediateAssistant,
        ObservableActivityFixtureCase::FinalAssistant,
        ObservableActivityFixtureCase::ReasoningSummary,
        ObservableActivityFixtureCase::UnknownSemantic,
    ] {
        let fixture = ObservableActivityTraceFixture::for_case(case);
        assert_observable_activity_trace(fixture.profile(), fixture.events());
    }

    contract::assert_details();
}
