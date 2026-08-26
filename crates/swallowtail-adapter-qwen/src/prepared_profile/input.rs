use crate::budgets::{QwenHeadlessBudgets, QwenSessionTurnBudget, QwenToolCallBudget};
use swallowtail_core::{
    HarnessMode, ModelId, ModelRouteId, ModelRouteRevision, ProviderId, ReasoningMode,
};
use swallowtail_runtime::{Deadline, OperationContent, RequestId, WorkingResourceRef};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Exact provider and model route for a Qwen operation.
pub struct QwenModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    provider_id: ProviderId,
    model_id: ModelId,
}

impl QwenModelSelection {
    /// Creates an exact Qwen model selection.
    #[must_use]
    pub const fn new(
        route_id: ModelRouteId,
        route_revision: ModelRouteRevision,
        provider_id: ProviderId,
        model_id: ModelId,
    ) -> Self {
        Self {
            route_id,
            route_revision,
            provider_id,
            model_id,
        }
    }

    pub(super) fn into_parts(self) -> (ModelRouteId, ModelRouteRevision, ProviderId, ModelId) {
        (
            self.route_id,
            self.route_revision,
            self.provider_id,
            self.model_id,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer inputs for one prepared Qwen structured run.
pub struct QwenRunProfileInput {
    request_id: RequestId,
    model: QwenModelSelection,
    content: OperationContent,
    working_resource: WorkingResourceRef,
    deadline: Deadline,
    reasoning_mode: Option<ReasoningMode>,
    harness_mode: Option<HarnessMode>,
    budgets: QwenHeadlessBudgets,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer inputs for one prepared Qwen interactive session.
pub struct QwenSessionProfileInput {
    request_id: RequestId,
    model: QwenModelSelection,
    working_resource: WorkingResourceRef,
    deadline: Option<Deadline>,
    reasoning_mode: Option<ReasoningMode>,
    harness_mode: Option<HarnessMode>,
    budgets: QwenHeadlessBudgets,
}

impl QwenSessionProfileInput {
    /// Creates a Qwen session profile without a deadline.
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: QwenModelSelection,
        working_resource: WorkingResourceRef,
    ) -> Self {
        Self {
            request_id,
            model,
            working_resource,
            deadline: None,
            reasoning_mode: None,
            harness_mode: None,
            budgets: QwenHeadlessBudgets::omitted(),
        }
    }

    /// Adds a deadline to the turn-scoped session.
    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }

    /// Selects one exact Qwen reasoning mode.
    #[must_use]
    pub fn with_reasoning_mode(mut self, reasoning_mode: ReasoningMode) -> Self {
        self.reasoning_mode = Some(reasoning_mode);
        self
    }

    /// Selects portable Plan for every child of this session.
    ///
    /// Omission keeps `--approval-mode default`. Only `HarnessMode::Plan` is
    /// admitted, and only on exact `0.21.15`, `0.22.0`, and `0.22.1`.
    #[must_use]
    pub const fn with_harness_mode(mut self, harness_mode: HarnessMode) -> Self {
        self.harness_mode = Some(harness_mode);
        self
    }

    /// Selects an admitted per-child session-turn budget.
    #[must_use]
    pub const fn with_session_turn_budget(mut self, budget: QwenSessionTurnBudget) -> Self {
        self.budgets = self.budgets.with_session_turns(budget);
        self
    }

    /// Selects an admitted per-child tool-call budget.
    #[must_use]
    pub const fn with_tool_call_budget(mut self, budget: QwenToolCallBudget) -> Self {
        self.budgets = self.budgets.with_tool_calls(budget);
        self
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        RequestId,
        QwenModelSelection,
        WorkingResourceRef,
        Option<Deadline>,
        Option<ReasoningMode>,
        Option<HarnessMode>,
        QwenHeadlessBudgets,
    ) {
        (
            self.request_id,
            self.model,
            self.working_resource,
            self.deadline,
            self.reasoning_mode,
            self.harness_mode,
            self.budgets,
        )
    }
}

impl QwenRunProfileInput {
    /// Creates a bounded Qwen structured-run profile.
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: QwenModelSelection,
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
            harness_mode: None,
            budgets: QwenHeadlessBudgets::omitted(),
        }
    }

    /// Selects one exact Qwen reasoning mode.
    #[must_use]
    pub fn with_reasoning_mode(mut self, reasoning_mode: ReasoningMode) -> Self {
        self.reasoning_mode = Some(reasoning_mode);
        self
    }

    /// Selects portable Plan for the structured-run child.
    ///
    /// Omission keeps `--approval-mode default`. Only `HarnessMode::Plan` is
    /// admitted, and only on exact `0.21.15`, `0.22.0`, and `0.22.1`.
    #[must_use]
    pub const fn with_harness_mode(mut self, harness_mode: HarnessMode) -> Self {
        self.harness_mode = Some(harness_mode);
        self
    }

    /// Selects an admitted per-child session-turn budget.
    #[must_use]
    pub const fn with_session_turn_budget(mut self, budget: QwenSessionTurnBudget) -> Self {
        self.budgets = self.budgets.with_session_turns(budget);
        self
    }

    /// Selects an admitted per-child tool-call budget.
    #[must_use]
    pub const fn with_tool_call_budget(mut self, budget: QwenToolCallBudget) -> Self {
        self.budgets = self.budgets.with_tool_calls(budget);
        self
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        RequestId,
        QwenModelSelection,
        OperationContent,
        WorkingResourceRef,
        Deadline,
        Option<ReasoningMode>,
        Option<HarnessMode>,
        QwenHeadlessBudgets,
    ) {
        (
            self.request_id,
            self.model,
            self.content,
            self.working_resource,
            self.deadline,
            self.reasoning_mode,
            self.harness_mode,
            self.budgets,
        )
    }
}
