use swallowtail_core::{
    FailureClassification, FailureKind, FailureOrigin, FailureRecovery, SafeDiagnostic,
};
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
                FailureKind::AuthenticationRejected,
                FailureRecovery::ReauthenticationRequired,
            ),
            Some(42) => provider_failure(
                "swallowtail.gemini.headless.native_input",
                "Gemini CLI rejected the headless input",
                FailureKind::InvalidRequest,
                FailureRecovery::InputChangeRequired,
            ),
            Some(44) => provider_failure(
                "swallowtail.gemini.headless.native_sandbox",
                "Gemini CLI reported a sandbox failure",
                FailureKind::Unknown,
                FailureRecovery::ConfigurationChangeRequired,
            ),
            Some(52) => provider_failure(
                "swallowtail.gemini.headless.native_configuration",
                "Gemini CLI rejected its effective configuration",
                FailureKind::InvalidRequest,
                FailureRecovery::ConfigurationChangeRequired,
            ),
            Some(53) => provider_failure(
                "swallowtail.gemini.headless.native_turn_limit",
                "Gemini CLI reached its session-turn limit",
                FailureKind::InputLimitExceeded,
                FailureRecovery::InputChangeRequired,
            ),
            Some(54) => provider_failure(
                "swallowtail.gemini.headless.native_tool",
                "Gemini CLI reported a fatal tool execution failure",
                FailureKind::Unknown,
                FailureRecovery::Unknown,
            ),
            Some(55) => provider_failure(
                "swallowtail.gemini.headless.native_trust",
                "Gemini CLI rejected workspace trust",
                FailureKind::AuthorizationDenied,
                FailureRecovery::ConfigurationChangeRequired,
            ),
            Some(130) => provider_failure(
                "swallowtail.gemini.headless.process_interrupted",
                "Gemini CLI was interrupted outside Swallowtail cancellation",
                FailureKind::TransportInterrupted,
                FailureRecovery::RetryMaySucceed,
            ),
            _ if self.provider_failure.is_some() => TerminalStatus::ProviderFailed(
                self.provider_failure.expect("checked provider failure"),
            ),
            _ if !exit.success() => provider_failure(
                "swallowtail.gemini.headless.process_failed",
                "Gemini CLI exited unsuccessfully",
                FailureKind::Unknown,
                FailureRecovery::Unknown,
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

fn provider_failure(
    code: &'static str,
    message: &'static str,
    kind: FailureKind,
    recovery: FailureRecovery,
) -> TerminalStatus {
    TerminalStatus::ProviderFailed(
        SafeDiagnostic::new(code, message).with_failure_classification(FailureClassification::new(
            FailureOrigin::Harness,
            kind,
            recovery,
        )),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_native_exit_carries_harness_failure_evidence() {
        let outcome = ParsedTerminal {
            final_output: None,
            provider_failure: None,
            initialized: true,
            terminal_seen: true,
        }
        .outcome(ProcessExit::new(false, Some(41)));
        let failure = outcome.failure().expect("exit is a failure");

        assert_eq!(
            failure.diagnostic().failure_classification().origin(),
            FailureOrigin::Harness
        );
        assert_eq!(
            failure.diagnostic().failure_classification().kind(),
            FailureKind::AuthenticationRejected
        );
    }
}
