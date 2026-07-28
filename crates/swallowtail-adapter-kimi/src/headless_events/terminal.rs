use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    CleanupOutcome, OperationContent, ProcessExit, TerminalOutcome, TerminalStatus,
};

pub(crate) struct ParsedTerminal {
    final_output: Option<OperationContent>,
    terminal_seen: bool,
}

impl ParsedTerminal {
    pub(super) const fn new(final_output: Option<OperationContent>, terminal_seen: bool) -> Self {
        Self {
            final_output,
            terminal_seen,
        }
    }

    pub(crate) fn outcome(self, exit: ProcessExit) -> TerminalOutcome {
        let status = match exit.code() {
            Some(130 | 143) => TerminalStatus::ProviderFailed(SafeDiagnostic::new(
                "swallowtail.kimi.headless.process_interrupted",
                "Kimi Code was interrupted outside Swallowtail cancellation",
            )),
            _ if !exit.success() => TerminalStatus::ProviderFailed(SafeDiagnostic::new(
                "swallowtail.kimi.headless.process_failed",
                "Kimi Code exited unsuccessfully",
            )),
            _ if !self.terminal_seen => TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                "swallowtail.kimi.headless.incomplete_stream",
                "Kimi Code ended without a terminal resume hint",
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
