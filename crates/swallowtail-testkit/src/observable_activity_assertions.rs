use crate::{ObservableActivityFixtureCase, ObservableActivityTraceFixture};
use swallowtail_core::ObservableActivityProfile;
use swallowtail_runtime::RuntimeEvent;

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
