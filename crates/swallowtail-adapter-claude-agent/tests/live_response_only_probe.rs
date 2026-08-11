use futures_executor::block_on;
use futures_util::StreamExt;
use std::ffi::OsString;
use std::path::PathBuf;
use std::time::Duration;
use swallowtail_adapter_claude_agent::{
    CLAUDE_CODE_RESPONSE_ONLY_AXIS, CLAUDE_CODE_RESPONSE_ONLY_VERSION,
    ClaudeCodeResponseModelSelection, ClaudeCodeResponsePreparationInput,
    ClaudeCodeResponsePreparationProbe, ClaudeCodeResponseProfileInput,
    prepare_claude_code_response_only,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialMechanism,
    CredentialState, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, InstanceRevision, InterfaceVersionAxis, ModelId,
    ModelRouteId, ModelRouteRevision, ReasoningMode, RuntimeReadiness, SupportAuthority,
};
use swallowtail_host_local::{
    LocalExecutableLaunch, LocalHostServices, LocalProcessHost, LocalProcessLimits,
};
use swallowtail_runtime::{
    CleanupOutcome, DiscoveryCancellation, EnvironmentRef, ExecutableRef, OperationContent,
    PreparedAccessEvidence, RequestId, RuntimeEventKind, ScopeId, TerminalStatus,
};

#[test]
#[ignore = "requires SWALLOWTAIL_LIVE_CLAUDE_CODE_RESPONSE_ONLY=1, Claude Code 2.1.227, and local Max/OAuth state"]
fn configured_claude_code_returns_one_tool_free_text_response() {
    assert_eq!(
        std::env::var("SWALLOWTAIL_LIVE_CLAUDE_CODE_RESPONSE_ONLY").as_deref(),
        Ok("1"),
        "authenticated response-only probe requires its explicit gate"
    );
    assert!(
        std::env::var_os("ANTHROPIC_API_KEY").is_none(),
        "response-only live proof must use local subscription auth"
    );
    let source_before = git_status();
    let (local, target, environment, execution_host_id) = live_host();
    let access_id = AccessProfileId::new("live.claude-code.local-subscription").expect("access id");
    let access = AccessProfile::new(
        access_id.clone(),
        CredentialMechanism::LocalUnauthenticated,
        EntitlementMetering::SubscriptionAllowance,
        EndpointAudience::new("anthropic-claude-code").expect("audience"),
        SupportAuthority::ProviderSupported,
    );
    let prepared = block_on(prepare_claude_code_response_only(
        ClaudeCodeResponsePreparationInput::new(
            ConfiguredInstanceId::new("live.claude-code.response-only").expect("instance id"),
            InstanceRevision::new(CLAUDE_CODE_RESPONSE_ONLY_VERSION).expect("revision"),
            execution_host_id,
            target,
            environment,
            access,
            PreparedAccessEvidence::caller_asserted(AccessStatus::new(
                access_id,
                CredentialState::NotRequired,
                EntitlementState::Available,
                EndpointAuthorization::Allowed,
                RuntimeReadiness::Ready,
                SupportAuthority::ProviderSupported,
            )),
        ),
        ClaudeCodeResponsePreparationProbe::new(
            RequestId::new("live-claude-code-response-prepare").expect("request id"),
            ScopeId::new("live-claude-code-response-prepare").expect("scope id"),
            local.deadline_after(Duration::from_secs(5)),
            DiscoveryCancellation::new(),
        ),
        local.services().clone(),
    ))
    .expect("installed Claude Code response-only route prepares");
    let run = prepared
        .prepare_run(ClaudeCodeResponseProfileInput::new(
            RequestId::new("live-claude-code-response").expect("request id"),
            ClaudeCodeResponseModelSelection::new(
                ModelRouteId::new("live.claude-code.response-only").expect("route id"),
                ModelRouteRevision::new(CLAUDE_CODE_RESPONSE_ONLY_VERSION).expect("route revision"),
                ModelId::new("claude-sonnet-5").expect("model id"),
            ),
            OperationContent::new("Reply exactly CLAUDE_RESPONSE_ONLY_LIVE_OK.").expect("prompt"),
            local.deadline_after(Duration::from_secs(90)),
        ))
        .expect("response-only run prepares");
    assert!(run.request().working_resource().is_none());
    assert!(run.request().structured_output().is_none());

    let mut handle = block_on(run.start_run(local.services().clone()))
        .expect("authenticated response-only run starts");
    assert!(handle.take_callbacks().is_none());
    assert!(handle.take_management_binding().is_none());
    assert!(handle.detachment().is_none());
    let mut events = handle.take_events().expect("event stream");
    let terminal = handle.take_terminal_outcome().expect("terminal outcome");
    let outcome = block_on(async {
        while let Some(event) = events.next().await {
            event.expect("live response-only event remains valid");
        }
        terminal.await
    });
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome
            .output()
            .map(OperationContent::as_str)
            .map(str::trim),
        Some("CLAUDE_RESPONSE_ONLY_LIVE_OK")
    );
    assert_eq!(outcome.cleanup(), &CleanupOutcome::Clean);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);

    let medium = prepared
        .prepare_run(
            ClaudeCodeResponseProfileInput::new(
                RequestId::new("live-claude-code-response-medium").expect("request id"),
                ClaudeCodeResponseModelSelection::new(
                    ModelRouteId::new("live.claude-code.response-only.medium")
                        .expect("route id"),
                    ModelRouteRevision::new(CLAUDE_CODE_RESPONSE_ONLY_VERSION)
                        .expect("route revision"),
                    ModelId::new("claude-sonnet-5").expect("model id"),
                ),
                OperationContent::new(
                    "Solve carefully: find the lexicographically smallest permutation of A B C D E F such that C immediately follows A, E precedes B, F precedes B, B precedes D, and F is neither first nor last. Return only the permutation.",
                )
                .expect("prompt"),
                local.deadline_after(Duration::from_secs(90)),
            )
            .with_reasoning_mode(ReasoningMode::new("medium").expect("reasoning mode")),
        )
        .expect("medium response-only run prepares");
    let mut medium_handle = block_on(medium.start_run(local.services().clone()))
        .expect("authenticated medium response-only run starts");
    let medium_events = block_on(
        medium_handle
            .take_events()
            .expect("medium event stream")
            .collect::<Vec<_>>(),
    )
    .into_iter()
    .collect::<Result<Vec<_>, _>>()
    .expect("medium response events remain valid");
    let medium_outcome = block_on(
        medium_handle
            .take_terminal_outcome()
            .expect("medium terminal outcome"),
    );
    assert_eq!(medium_outcome.status(), &TerminalStatus::Completed);
    assert!(medium_outcome.output().is_some());
    assert!(medium_events.iter().any(|event| {
        event.kind() == &RuntimeEventKind::ProgressSnapshot && event.content().is_none()
    }));
    assert_eq!(
        medium_events
            .iter()
            .filter(|event| matches!(event.kind(), RuntimeEventKind::Activity(_)))
            .count(),
        1
    );
    assert_eq!(
        medium_events
            .iter()
            .filter(|event| event.kind() == &RuntimeEventKind::OutputAvailable)
            .count(),
        1
    );
    assert_eq!(block_on(medium_handle.close()), CleanupOutcome::Clean);

    let cancellable = prepared
        .prepare_run(ClaudeCodeResponseProfileInput::new(
            RequestId::new("live-claude-code-response-cancel").expect("request id"),
            ClaudeCodeResponseModelSelection::new(
                ModelRouteId::new("live.claude-code.response-only.cancel").expect("route id"),
                ModelRouteRevision::new(CLAUDE_CODE_RESPONSE_ONLY_VERSION).expect("route revision"),
                ModelId::new("claude-sonnet-5").expect("model id"),
            ),
            OperationContent::new("Write a long plain-text essay about cancellation.")
                .expect("prompt"),
            local.deadline_after(Duration::from_secs(90)),
        ))
        .expect("cancellable response-only run prepares");
    let mut cancelled = block_on(cancellable.start_run(local.services().clone()))
        .expect("cancellable response-only run starts");
    assert_eq!(
        block_on(cancelled.cancellation().request()).expect("cancellation request"),
        swallowtail_runtime::CancellationAcknowledgement::Requested
    );
    let mut cancelled_events = cancelled.take_events().expect("cancelled event stream");
    let cancelled_terminal = cancelled
        .take_terminal_outcome()
        .expect("cancelled terminal outcome");
    let cancelled_outcome = block_on(async {
        while let Some(event) = cancelled_events.next().await {
            event.expect("cancelled response event remains valid");
        }
        cancelled_terminal.await
    });
    assert_eq!(cancelled_outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(cancelled_outcome.cleanup(), &CleanupOutcome::Clean);
    assert_eq!(block_on(cancelled.close()), CleanupOutcome::Clean);
    assert_eq!(
        git_status(),
        source_before,
        "provider created a source artifact"
    );
}

fn live_host() -> (
    LocalHostServices,
    swallowtail_runtime::InstalledExecutableTarget,
    EnvironmentRef,
    ExecutionHostId,
) {
    let selected = installed_path("claude").expect("Claude Code is installed on PATH");
    let binary = std::fs::canonicalize(selected).expect("Claude Code resolves exactly");
    let environment =
        EnvironmentRef::new("live.claude-code.local-subscription").expect("environment");
    let execution_host_id = ExecutionHostId::new("live.claude-code.local-host").expect("host id");
    let executable = ExecutableRef::new("live.claude-code.installed").expect("executable ref");
    let (builder, target) = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_installed_executable_launch(
            executable,
            InterfaceVersionAxis::new(CLAUDE_CODE_RESPONSE_ONLY_AXIS).expect("release axis"),
            LocalExecutableLaunch::new(binary),
        );
    let home = std::env::var_os("HOME").expect("local Claude auth requires HOME");
    let user = std::env::var_os("USER").expect("local Claude auth requires USER");
    let logname = std::env::var_os("LOGNAME").expect("local Claude auth requires LOGNAME");
    let local = builder
        .approve_environment(
            environment.clone(),
            [
                (OsString::from("HOME"), home),
                (OsString::from("USER"), user),
                (OsString::from("LOGNAME"), logname),
            ],
        )
        .build_services(execution_host_id.clone());
    (local, target, environment, execution_host_id)
}

fn installed_path(command: &str) -> Option<PathBuf> {
    std::env::var_os("PATH")
        .into_iter()
        .flat_map(|path| std::env::split_paths(&path).collect::<Vec<_>>())
        .map(|directory| directory.join(command))
        .find(|candidate| candidate.is_file())
}

fn git_status() -> Vec<u8> {
    std::process::Command::new("git")
        .args(["status", "--porcelain=v1", "--untracked-files=all"])
        .output()
        .expect("git status runs")
        .stdout
}
