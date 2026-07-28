use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    CleanupOutcome, OperationContent, ProcessExit, TerminalOutcome, TerminalStatus,
};

pub(crate) struct ParsedTerminal {
    pub(super) final_output: Option<OperationContent>,
    pub(super) provider_failure: Option<SafeDiagnostic>,
    pub(super) initialized: bool,
    pub(super) terminal_seen: bool,
}

impl ParsedTerminal {
    pub(crate) fn outcome(self, exit: ProcessExit) -> TerminalOutcome {
        let status = match exit.code() {
            Some(130) => provider_failure(
                "swallowtail.claude_code.headless.process_interrupted",
                "Claude Code was interrupted outside Swallowtail cancellation",
            ),
            _ if self.provider_failure.is_some() => TerminalStatus::ProviderFailed(
                self.provider_failure.expect("checked provider failure"),
            ),
            _ if !exit.success() => provider_failure(
                "swallowtail.claude_code.headless.process_failed",
                "Claude Code exited unsuccessfully",
            ),
            _ if !self.initialized || !self.terminal_seen => {
                TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                    "swallowtail.claude_code.headless.incomplete_stream",
                    "Claude Code ended without complete init and result evidence",
                ))
            }
            _ => TerminalStatus::Completed,
        };
        let outcome = TerminalOutcome::new(status, CleanupOutcome::Clean);
        match self.final_output {
            Some(output) => outcome.with_output(output),
            None => outcome,
        }
    }
}

fn provider_failure(code: &'static str, message: &'static str) -> TerminalStatus {
    TerminalStatus::ProviderFailed(SafeDiagnostic::new(code, message))
}
