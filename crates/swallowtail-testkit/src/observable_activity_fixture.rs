use swallowtail_core::ObservableActivityProfile;
use swallowtail_runtime::RuntimeEvent;

mod interaction;
mod lifecycle;
mod support;

/// Observable-activity fidelity or semantic scenario represented by a trace.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservableActivityFixtureCase {
    /// Started, updated, and completed lifecycle.
    CompleteLifecycle,
    /// Update and completion without an explicit start.
    UpdateAndCompletion,
    /// Completion-only lifecycle.
    CompletionOnly,
    /// Route without observable activity.
    Unavailable,
    /// Activity correlated to a provider callback.
    CallbackCorrelation,
    /// Activity correlated to a consumer-owned direct tool.
    DirectToolCorrelation,
    /// Non-terminal assistant text.
    IntermediateAssistant,
    /// Terminal assistant answer text.
    FinalAssistant,
    /// Provider-safe reasoning summary.
    ReasoningSummary,
    /// Namespaced unmodelled semantic activity.
    UnknownSemantic,
}

/// Provider-neutral prepared profile and projected operation event trace.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservableActivityTraceFixture {
    profile: ObservableActivityProfile,
    events: Vec<RuntimeEvent>,
}

impl ObservableActivityTraceFixture {
    /// Builds the prepared profile and event trace for one scenario.
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

    /// Returns the route's prepared observable-activity profile.
    #[must_use]
    pub const fn profile(&self) -> &ObservableActivityProfile {
        &self.profile
    }

    /// Returns the projected operation events in observation order.
    #[must_use]
    pub fn events(&self) -> &[RuntimeEvent] {
        &self.events
    }
}
