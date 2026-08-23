//! Caller-decreasing Qwen headless turn and tool budgets.

use crate::selection::QwenPlanSelection;
use crate::validation::failure;
use swallowtail_core::{Diagnostic, SafeDiagnostic};
use swallowtail_runtime::{PreparationFailure, PreparationStage, RuntimeFailure};

pub(crate) const QUALIFIED_VERSION: &str = "0.21.15";
pub(crate) const DEFAULT_SESSION_TURNS: u8 = 24;
pub(crate) const DEFAULT_TOOL_CALLS: u8 = 16;

/// Caller-decreasing Qwen session-turn budget for one child process.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QwenSessionTurnBudget(u8);

impl QwenSessionTurnBudget {
    /// Smallest admitted turn budget.
    pub const MIN: u8 = 1;
    /// Largest admitted turn budget; omission uses this same current argv.
    pub const MAX: u8 = DEFAULT_SESSION_TURNS;

    /// Accepts only `1..=24`.
    #[must_use]
    pub const fn try_new(value: u8) -> Option<Self> {
        if value >= Self::MIN && value <= Self::MAX {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Returns the admitted integer.
    #[must_use]
    pub const fn get(self) -> u8 {
        self.0
    }
}

/// Caller-decreasing Qwen tool-call budget for one child process.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QwenToolCallBudget(u8);

impl QwenToolCallBudget {
    /// Zero-tool budget; first tool tick aborts before dispatch.
    pub const MIN: u8 = 0;
    /// Largest admitted tool budget; omission uses this same current argv.
    pub const MAX: u8 = DEFAULT_TOOL_CALLS;

    /// Accepts only `0..=16`.
    #[must_use]
    pub const fn try_new(value: u8) -> Option<Self> {
        if value <= Self::MAX {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Returns the admitted integer.
    #[must_use]
    pub const fn get(self) -> u8 {
        self.0
    }
}

/// Optional per-child Qwen turn and tool budgets.
///
/// Omitted fields keep the current command bytes `24` and `16`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QwenHeadlessBudgets {
    session_turns: Option<QwenSessionTurnBudget>,
    tool_calls: Option<QwenToolCallBudget>,
}

impl QwenHeadlessBudgets {
    /// No caller-selected budgets; argv stays `24` / `16`.
    #[must_use]
    pub const fn omitted() -> Self {
        Self {
            session_turns: None,
            tool_calls: None,
        }
    }

    /// Selects an admitted session-turn budget.
    #[must_use]
    pub const fn with_session_turns(mut self, session_turns: QwenSessionTurnBudget) -> Self {
        self.session_turns = Some(session_turns);
        self
    }

    /// Selects an admitted tool-call budget.
    #[must_use]
    pub const fn with_tool_calls(mut self, tool_calls: QwenToolCallBudget) -> Self {
        self.tool_calls = Some(tool_calls);
        self
    }

    /// Returns the selected session-turn budget.
    #[must_use]
    pub const fn session_turns(&self) -> Option<QwenSessionTurnBudget> {
        self.session_turns
    }

    /// Returns the selected tool-call budget.
    #[must_use]
    pub const fn tool_calls(&self) -> Option<QwenToolCallBudget> {
        self.tool_calls
    }

    pub(crate) const fn is_selected(self) -> bool {
        self.session_turns.is_some() || self.tool_calls.is_some()
    }

    pub(crate) fn session_turns_arg(self) -> String {
        self.session_turns
            .map_or(DEFAULT_SESSION_TURNS, QwenSessionTurnBudget::get)
            .to_string()
    }

    pub(crate) fn tool_calls_arg(self) -> String {
        self.tool_calls
            .map_or(DEFAULT_TOOL_CALLS, QwenToolCallBudget::get)
            .to_string()
    }
}

pub(crate) fn validate_preparation(
    version: &swallowtail_core::InterfaceVersion,
    budgets: QwenHeadlessBudgets,
) -> Result<(), PreparationFailure> {
    if !budgets.is_selected() || version.as_str() == QUALIFIED_VERSION {
        Ok(())
    } else {
        Err(PreparationFailure::new(
            PreparationStage::Preflight,
            Diagnostic::new(SafeDiagnostic::new(
                "swallowtail.qwen.preparation.budget_unsupported",
                "Qwen turn and tool budgets require exact package 0.21.15",
            )),
        ))
    }
}

pub(crate) fn validate_runtime(
    selection: &QwenPlanSelection,
    budgets: QwenHeadlessBudgets,
) -> Result<(), RuntimeFailure> {
    if !budgets.is_selected() || selection.version().as_str() == QUALIFIED_VERSION {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.qwen.headless.budget_version_mismatch",
            "Qwen turn and tool budgets require exact package 0.21.15",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{QwenHeadlessBudgets, QwenSessionTurnBudget, QwenToolCallBudget};

    #[test]
    fn constructors_admit_only_the_research_198_domains() {
        assert!(QwenSessionTurnBudget::try_new(0).is_none());
        assert!(QwenSessionTurnBudget::try_new(1).is_some());
        assert!(QwenSessionTurnBudget::try_new(24).is_some());
        assert!(QwenSessionTurnBudget::try_new(25).is_none());
        assert!(QwenToolCallBudget::try_new(0).is_some());
        assert!(QwenToolCallBudget::try_new(16).is_some());
        assert!(QwenToolCallBudget::try_new(17).is_none());
        assert_eq!(QwenHeadlessBudgets::omitted().session_turns_arg(), "24");
        assert_eq!(QwenHeadlessBudgets::omitted().tool_calls_arg(), "16");
        let only_turns = QwenHeadlessBudgets::omitted()
            .with_session_turns(QwenSessionTurnBudget::try_new(3).expect("admitted turns"));
        assert_eq!(only_turns.session_turns_arg(), "3");
        assert_eq!(only_turns.tool_calls_arg(), "16");
        let only_tools = QwenHeadlessBudgets::omitted()
            .with_tool_calls(QwenToolCallBudget::try_new(2).expect("admitted tools"));
        assert_eq!(only_tools.session_turns_arg(), "24");
        assert_eq!(only_tools.tool_calls_arg(), "2");
    }
}
