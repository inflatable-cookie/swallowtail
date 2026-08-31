use super::super::*;
use swallowtail_adapter_codex::{CodexPreparedExec, CodexPreparedIntegration};
use swallowtail_core::{AccessProfile, AccessStatus};
use swallowtail_runtime::PreparationFailure;

use super::ledger::FIXTURE_VERSION;

/// Prepares one exec run under caller-supplied exact access evidence.
///
/// The shared root helper always asserts the ready access posture, so the
/// cross-access counterexamples bind their own exact profile and observation
/// here instead of widening that helper.
pub(super) fn exec_run_with_access(
    request_id: &str,
    profile: AccessProfile,
    status: AccessStatus,
) -> Result<CodexPreparedExec, PreparationFailure> {
    integration_with_access(profile, status)?.prepare_structured_exec(CodexExecProfileInput::new(
        RequestId::new(request_id).unwrap(),
        OperationContent::new("private prompt").unwrap(),
        model(),
        working_resource(),
        ExternalNetworkPolicy::Denied,
        ExternalSearchPolicy::Disabled,
    ))
}

fn integration_with_access(
    profile: AccessProfile,
    status: AccessStatus,
) -> Result<CodexPreparedIntegration, PreparationFailure> {
    let host = ExecutionHostId::new("host.local").unwrap();
    let input = CodexPreparationInput::new(
        CodexPreparedDriver::StructuredExec,
        ConfiguredInstanceId::new("codex.prepared").unwrap(),
        InstanceRevision::new("1").unwrap(),
        host.clone(),
        InstalledExecutableTarget::new(
            ExecutableRef::new("codex-executable").unwrap(),
            InterfaceVersionAxis::new(CODEX_CLI_AXIS).unwrap(),
        ),
        EnvironmentRef::new("saved-login").unwrap(),
        profile,
        PreparedAccessEvidence::caller_asserted(status),
    );
    let probe = CodexPreparationProbe::new(
        RequestId::new("probe-access").unwrap(),
        swallowtail_runtime::ScopeId::new("probe-access").unwrap(),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    );
    let (process, _) = FakeProcessService::completed(&format!("codex-cli {FIXTURE_VERSION}\n"));
    let services = host_services_for(host, process).with_time(std::sync::Arc::new(PendingTime));
    block_on(prepare_codex(input, probe, services))
}
