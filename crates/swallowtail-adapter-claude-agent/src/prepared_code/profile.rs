#[path = "profile/plan.rs"]
mod plan;
#[path = "profile/prepared.rs"]
mod prepared;

pub use prepared::{ClaudeCodePreparedEvidence, ClaudeCodePreparedRun};

use super::ClaudeCodePreparedIntegration;
use super::instance::{REASONING_MODES, run_capabilities};
use crate::ClaudeCodeMaximumTurns;
use crate::claude_code_activity::profile::{activity_profile, with_activity};
use plan::{build_plan, instance_with_capabilities, operation_capabilities, requirements};
use prepared::new_prepared_run;
use swallowtail_core::{
    HarnessConfigurationPosture, HarnessIsolation, ModelId, ModelRoute, ModelRouteId,
    ModelRouteRevision, ReasoningMode,
};
use swallowtail_runtime::{
    Deadline, OperationContent, OperationPolicy, PreparationFailure, ProviderRetentionPolicy,
    RequestId, StructuredRunRequest, WorkingResourceRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Exact model route for a native Claude Code headless run.
pub struct ClaudeCodeModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    model_id: ModelId,
}

impl ClaudeCodeModelSelection {
    /// Creates an exact native Claude Code model selection.
    #[must_use]
    pub const fn new(
        route_id: ModelRouteId,
        route_revision: ModelRouteRevision,
        model_id: ModelId,
    ) -> Self {
        Self {
            route_id,
            route_revision,
            model_id,
        }
    }

    fn into_parts(self) -> (ModelRouteId, ModelRouteRevision, ModelId) {
        (self.route_id, self.route_revision, self.model_id)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer inputs for one native `claude -p` structured run.
pub struct ClaudeCodeRunProfileInput {
    request_id: RequestId,
    model: ClaudeCodeModelSelection,
    content: OperationContent,
    working_resource: WorkingResourceRef,
    deadline: Deadline,
    reasoning_mode: Option<ReasoningMode>,
    maximum_turns: Option<ClaudeCodeMaximumTurns>,
    watchers: bool,
}

impl ClaudeCodeRunProfileInput {
    /// Creates a bounded native headless run profile.
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: ClaudeCodeModelSelection,
        content: OperationContent,
        working_resource: WorkingResourceRef,
        deadline: Deadline,
    ) -> Self {
        Self {
            request_id,
            model,
            content,
            working_resource,
            deadline,
            reasoning_mode: None,
            maximum_turns: None,
            watchers: false,
        }
    }

    /// Selects the requested reasoning mode.
    #[must_use]
    pub fn with_reasoning_mode(mut self, reasoning_mode: ReasoningMode) -> Self {
        self.reasoning_mode = Some(reasoning_mode);
        self
    }

    /// Selects one admitted Claude Code maximum agentic-turn bound.
    ///
    /// Omission preserves the exact current command with no `--max-turns`
    /// argument and passes the approved environment through unchanged. That is
    /// not a claim of unlimited execution: with the flag absent, an ambient
    /// `CLAUDE_CODE_MAX_TURNS` remains authoritative on the host, and an
    /// invalid ambient value still aborts Claude Code at startup.
    ///
    /// A selected value is dispatched as exactly one canonical
    /// `--max-turns <n>` and unconditionally overrides that ambient value.
    /// Swallowtail neither reads nor rewrites the approved environment to
    /// achieve it. The selection is rejected before process work unless the
    /// prepared Claude Code version is qualified rather than
    /// provisionally permitted.
    #[must_use]
    pub const fn with_maximum_turns(mut self, maximum_turns: ClaudeCodeMaximumTurns) -> Self {
        self.maximum_turns = Some(maximum_turns);
        self
    }

    /// Returns the selected maximum-turn bound when one was supplied.
    #[must_use]
    pub const fn maximum_turns(&self) -> Option<ClaudeCodeMaximumTurns> {
        self.maximum_turns
    }

    /// Opts this prepared run into the exact Claude Code `2.1.251` watcher candidate.
    ///
    /// Omission preserves the current empty strict MCP command and does not
    /// open a bridge or lease private files. Opt-in is rejected before those
    /// effects unless the prepared version is exactly `2.1.251`. This does not
    /// advertise watcher support.
    #[must_use]
    pub const fn with_watchers(mut self) -> Self {
        self.watchers = true;
        self
    }

    /// Reports whether this profile requested the watcher candidate.
    #[must_use]
    pub const fn watchers(&self) -> bool {
        self.watchers
    }

    fn into_parts(
        self,
    ) -> (
        RequestId,
        ClaudeCodeModelSelection,
        OperationContent,
        WorkingResourceRef,
        Deadline,
        Option<ReasoningMode>,
        Option<ClaudeCodeMaximumTurns>,
        bool,
    ) {
        (
            self.request_id,
            self.model,
            self.content,
            self.working_resource,
            self.deadline,
            self.reasoning_mode,
            self.maximum_turns,
            self.watchers,
        )
    }
}

impl ClaudeCodePreparedIntegration {
    /// Prepares a native `claude -p` structured run.
    pub fn prepare_run(
        &self,
        input: ClaudeCodeRunProfileInput,
    ) -> Result<ClaudeCodePreparedRun, PreparationFailure> {
        let (
            request_id,
            model,
            content,
            working_resource,
            deadline,
            reasoning,
            maximum_turns,
            watchers,
        ) = input.into_parts();
        if reasoning
            .as_ref()
            .is_some_and(|mode| !REASONING_MODES.contains(&mode.as_str()))
        {
            return Err(plan::failure(
                "swallowtail.claude_code.headless.preparation.reasoning_mode_unsupported",
                "Claude Code prepared run reasoning mode is unsupported",
            ));
        }
        if maximum_turns.is_some() && !plan::qualifies_maximum_turns(self) {
            return Err(plan::failure(
                "swallowtail.claude_code.headless.preparation.maximum_turns_unqualified",
                "Claude Code prepared run maximum turns requires a qualified Claude Code version",
            ));
        }
        if watchers && !plan::qualifies_watchers(self) {
            return Err(plan::failure(
                "swallowtail.claude_code.headless.preparation.watchers_unqualified",
                "Claude Code prepared run watchers require exact Claude Code 2.1.251",
            ));
        }
        let activity = activity_profile(self, watchers)?;
        let capabilities = with_activity(run_capabilities(), &activity);
        let instance = instance_with_capabilities(self, capabilities.clone());
        let operation_capabilities = operation_capabilities(&capabilities, reasoning.as_ref());
        let requirements = requirements(self, operation_capabilities, watchers);
        let (route_id, route_revision, model_id) = model.into_parts();
        let route = ModelRoute::new(
            route_id,
            route_revision,
            self.instance().id().clone(),
            model_id,
            capabilities,
        );
        let plan = build_plan(self, &instance, &route, &requirements)?;
        let mut policy = OperationPolicy::offline()
            .with_provider_retention(ProviderRetentionPolicy::Prohibited)
            .with_harness_mode(swallowtail_core::HarnessMode::Plan)
            .with_harness_isolation(HarnessIsolation::AmbientHost)
            .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
        if let Some(reasoning) = reasoning {
            policy = policy.with_reasoning_mode(reasoning);
        }
        let request = StructuredRunRequest::new(request_id, content, policy)
            .with_working_resource(working_resource)
            .with_deadline(deadline);
        new_prepared_run(self, plan, activity, maximum_turns, watchers, request)
    }
}
