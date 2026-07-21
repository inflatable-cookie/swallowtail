use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    CleanupOutcome, OperationContent, ProcessExit, TerminalOutcome, TerminalStatus,
};

pub(crate) struct ParsedTerminal {
    final_output: Option<OperationContent>,
    provider_failure: Option<SafeDiagnostic>,
    terminal_seen: bool,
}

impl ParsedTerminal {
    pub(super) const fn new(
        final_output: Option<OperationContent>,
        provider_failure: Option<SafeDiagnostic>,
        terminal_seen: bool,
    ) -> Self {
        Self {
            final_output,
            provider_failure,
            terminal_seen,
        }
    }

    pub(crate) fn outcome(self, exit: ProcessExit) -> TerminalOutcome {
        let status = match exit.code() {
            Some(53) => TerminalStatus::ProviderFailed(SafeDiagnostic::new(
                "swallowtail.qwen.headless.native_turn_limit",
                "Qwen Code reached its configured session-turn limit",
            )),
            Some(55) => TerminalStatus::ProviderFailed(SafeDiagnostic::new(
                "swallowtail.qwen.headless.native_budget",
                "Qwen Code reached a configured native run budget",
            )),
            Some(130) => TerminalStatus::ProviderFailed(SafeDiagnostic::new(
                "swallowtail.qwen.headless.process_interrupted",
                "Qwen Code was interrupted outside Swallowtail cancellation",
            )),
            _ if self.provider_failure.is_some() => TerminalStatus::ProviderFailed(
                self.provider_failure.expect("checked provider failure"),
            ),
            _ if !exit.success() => TerminalStatus::ProviderFailed(SafeDiagnostic::new(
                "swallowtail.qwen.headless.process_failed",
                "Qwen Code exited unsuccessfully",
            )),
            _ if !self.terminal_seen => TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                "swallowtail.qwen.headless.incomplete_stream",
                "Qwen Code ended without a terminal result",
            )),
            _ => TerminalStatus::Completed,
        };
        let outcome = TerminalOutcome::new(status, CleanupOutcome::Clean);
        match self.final_output {
            Some(output) => outcome.with_output(output),
            None => outcome,
        }
    }
}
