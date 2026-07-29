use swallowtail_core::ObservableActivityProfile;
use swallowtail_runtime::RuntimeEvent;

mod interaction;
mod lifecycle;
mod support;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservableActivityFixtureCase {
    CompleteLifecycle,
    UpdateAndCompletion,
    CompletionOnly,
    Unavailable,
    CallbackCorrelation,
    DirectToolCorrelation,
    IntermediateAssistant,
    FinalAssistant,
    ReasoningSummary,
    UnknownSemantic,
}

/// Provider-neutral prepared profile and projected operation event trace.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservableActivityTraceFixture {
    profile: ObservableActivityProfile,
    events: Vec<RuntimeEvent>,
}

impl ObservableActivityTraceFixture {
    #[must_use]
    pub fn for_case(case: ObservableActivityFixtureCase) -> Self {
        match case {
            ObservableActivityFixtureCase::CompleteLifecycle => lifecycle::complete(),
            ObservableActivityFixtureCase::UpdateAndCompletion => {
                lifecycle::update_and_completion()
            }
            ObservableActivityFixtureCase::CompletionOnly => lifecycle::completion_only(),
            ObservableActivityFixtureCase::Unavailable => lifecycle::unavailable(),
            ObservableActivityFixtureCase::CallbackCorrelation => interaction::callback(),
            ObservableActivityFixtureCase::DirectToolCorrelation => interaction::direct_tool(),
            ObservableActivityFixtureCase::IntermediateAssistant => {
                interaction::intermediate_assistant()
            }
            ObservableActivityFixtureCase::FinalAssistant => interaction::final_assistant(),
            ObservableActivityFixtureCase::ReasoningSummary => interaction::reasoning_summary(),
            ObservableActivityFixtureCase::UnknownSemantic => interaction::unknown_semantic(),
        }
    }

    #[must_use]
    pub const fn profile(&self) -> &ObservableActivityProfile {
        &self.profile
    }

    #[must_use]
    pub fn events(&self) -> &[RuntimeEvent] {
        &self.events
    }
}
