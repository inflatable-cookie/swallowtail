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
            Some(41) => provider_failure(
                "swallowtail.gemini.headless.native_authentication",
                "Gemini CLI rejected non-interactive authentication",
            ),
            Some(42) => provider_failure(
                "swallowtail.gemini.headless.native_input",
                "Gemini CLI rejected the headless input",
            ),
            Some(44) => provider_failure(
                "swallowtail.gemini.headless.native_sandbox",
                "Gemini CLI reported a sandbox failure",
            ),
            Some(52) => provider_failure(
                "swallowtail.gemini.headless.native_configuration",
                "Gemini CLI rejected its effective configuration",
            ),
            Some(53) => provider_failure(
                "swallowtail.gemini.headless.native_turn_limit",
                "Gemini CLI reached its session-turn limit",
            ),
            Some(54) => provider_failure(
                "swallowtail.gemini.headless.native_tool",
                "Gemini CLI reported a fatal tool execution failure",
            ),
            Some(55) => provider_failure(
                "swallowtail.gemini.headless.native_trust",
                "Gemini CLI rejected workspace trust",
            ),
            Some(130) => provider_failure(
                "swallowtail.gemini.headless.process_interrupted",
                "Gemini CLI was interrupted outside Swallowtail cancellation",
            ),
            _ if self.provider_failure.is_some() => TerminalStatus::ProviderFailed(
                self.provider_failure.expect("checked provider failure"),
            ),
            _ if !exit.success() => provider_failure(
                "swallowtail.gemini.headless.process_failed",
                "Gemini CLI exited unsuccessfully",
            ),
            _ if !self.initialized || !self.terminal_seen => {
                TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                    "swallowtail.gemini.headless.incomplete_stream",
                    "Gemini CLI ended without complete init and result evidence",
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
