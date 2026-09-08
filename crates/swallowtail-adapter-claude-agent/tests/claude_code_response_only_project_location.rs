//! Card 108: the response-only route accepts one optional `Read` working
//! resource as the native child's project location. The scratch-directory
//! fixture freezes the child-side view: the child runs in the leased
//! directory with tools suppressed, and the CLI's cwd-relative ambient
//! behaviour (project settings, `CLAUDE.md` discovery, git context) anchors
//! at that directory. Nothing here claims the directory is a boundary; the
//! route stays `AmbientHost`.

use crate::claude_code_support::{
    FakeProcessService, PendingTimeService, host_services, response_fixture,
    response_preparation_input,
};
use futures_executor::block_on;
use futures_util::StreamExt;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use swallowtail_adapter_claude_agent::{
    ClaudeCodeResponseModelSelection, ClaudeCodeResponsePreparationProbe,
    ClaudeCodeResponsePreparedIntegration, ClaudeCodeResponsePreparedRun,
    ClaudeCodeResponseProfileInput, prepare_claude_code_response_only,
};
use swallowtail_core::{
    Capability, CapabilityConstraint, ExecutionHostId, HarnessIsolation, HostServiceKind, ModelId,
    ModelRouteId, ModelRouteRevision, ResourceAccess, ResourceRepresentation,
};
use swallowtail_host_local::{LocalHostServices, LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, DiscoveryCancellation, MonotonicInstant, OperationContent,
    PreparationStage, RequestId, RuntimeEventKind, ScopeId, TerminalStatus, WorkingResourceRef,
};

const FAR_FUTURE: MonotonicInstant = MonotonicInstant::from_ticks(u64::MAX / 2);
const RESOURCE_NAME: &str = "fixture.project-directory";
static SCRATCH_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[test]
fn resource_free_route_keeps_authority_free_preparation() {
    let host = ExecutionHostId::new("host.response-location.absent").expect("host is valid");
    let (process, _state) = FakeProcessService::completed("2.1.228 (Claude Code)\n");
    let (services, _task) = host_services(host.clone(), process, Arc::new(PendingTimeService));
    let prepared = block_on(prepare_claude_code_response_only(
        response_preparation_input(host),
        probe("absent"),
        services,
    ))
    .expect("resource-free preparation is unchanged");
    let run = profile(&prepared, "absent", None);
    assert!(run.request().working_resource().is_none());
    assert!(
        !run.plan()
            .requirements()
            .capabilities()
            .any(|requirement| requirement.capability() == Capability::WorkingResource)
    );
    assert!(
        !run.plan()
            .requirements()
            .host_services()
            .any(|service| service == HostServiceKind::WorkingResource)
    );
}

#[test]
fn bound_resource_prepares_with_read_filesystem_authority() {
    let fixture = project_fixture("plan");
    let host = ExecutionHostId::new("host.response-location.plan").expect("host is valid");
    let prepared = prepared_with_project(host, &fixture);
    let run = profile(&prepared, "bound-plan", Some(fixture.resource.clone()));
    let requirement = run
        .plan()
        .requirements()
        .capabilities()
        .find(|requirement| requirement.capability() == Capability::WorkingResource)
        .expect("the bound resource advertises working-resource authority");
    assert!(requirement.constraints().any(
        |constraint| constraint == &CapabilityConstraint::ResourceAccess(ResourceAccess::Read)
    ));
    assert!(requirement.constraints().any(|constraint| constraint
        == &CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem)));
    assert!(
        run.plan()
            .requirements()
            .host_services()
            .any(|service| service == HostServiceKind::WorkingResource)
    );
    assert_eq!(
        run.plan().requirements().harness_isolation(),
        Some(HarnessIsolation::AmbientHost)
    );
    assert_eq!(run.request().working_resource(), Some(&fixture.resource));
}

#[test]
fn bound_resource_reaches_the_child_process_request() {
    let fixture = project_fixture("request");
    let host = ExecutionHostId::new("host.response-location.request").expect("host is valid");
    let prepared = prepared_with_project(host.clone(), &fixture);
    let run = profile(&prepared, "bound-request", Some(fixture.resource.clone()));
    let (process, state) =
        FakeProcessService::completed(&response_fixture("response-complete.jsonl"));
    let services = fixture.local.services().clone().with_process(process);
    let mut handle = block_on(run.start_run(services)).expect("resource-bound run starts");
    let outcome = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(state.request().working_resource, Some(RESOURCE_NAME.into()));
}

#[test]
fn preparation_fails_closed_without_host_working_resource_authority() {
    let host = ExecutionHostId::new("host.response-location.unbacked").expect("host is valid");
    let (process, _state) = FakeProcessService::completed("2.1.228 (Claude Code)\n");
    let (services, _task) = host_services(host.clone(), process, Arc::new(PendingTimeService));
    let prepared = block_on(prepare_claude_code_response_only(
        response_preparation_input(host),
        probe("unbacked"),
        services,
    ))
    .expect("resource-free preparation is unchanged");
    let error = prepared
        .prepare_run(profile_input(
            "unbacked",
            Some(WorkingResourceRef::new(RESOURCE_NAME).expect("resource is valid")),
        ))
        .expect_err("a bound resource without host authority must fail closed");
    assert_eq!(error.stage(), PreparationStage::Preflight);
}

#[test]
fn run_rejects_when_host_services_lose_the_working_resource_service() {
    let fixture = project_fixture("run-guard");
    let host = ExecutionHostId::new("host.response-location.run-guard").expect("host is valid");
    let prepared = prepared_with_project(host.clone(), &fixture);
    let run = profile(&prepared, "bound-guard", Some(fixture.resource.clone()));
    let (process, state) = FakeProcessService::held_open();
    let (services, _task) = host_services(host, process, Arc::new(PendingTimeService));
    let error = match block_on(run.start_run(services)) {
        Ok(_handle) => panic!("run-time validation must reject the lost working-resource service"),
        Err(error) => error,
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude_code.response_only.host_service_missing"
    );
    assert!(!state.started());
}

#[test]
fn scratch_directory_fixture_freezes_the_child_project_location() {
    let fixture = project_fixture("scratch");
    let host = ExecutionHostId::new("host.response-location.scratch").expect("host is valid");
    let prepared = prepared_with_project(host, &fixture);
    let run = profile(&prepared, "bound-scratch", Some(fixture.resource.clone()));
    let mut handle = block_on(run.start_run(fixture.local.services().clone()))
        .expect("the resource-bound run starts against the local host");
    let events = block_on(
        handle
            .take_events()
            .expect("event stream is available")
            .collect::<Vec<_>>(),
    )
    .into_iter()
    .collect::<Result<Vec<_>, _>>()
    .expect("events are valid");
    let outcome = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome.output().map(OperationContent::as_str),
        Some(r#"{"decision":"accept","score":7}"#)
    );
    assert_eq!(
        events
            .iter()
            .filter(|event| event.kind() == &RuntimeEventKind::OutputAvailable)
            .count(),
        1
    );
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);

    // The native child ran inside the leased directory.
    let child_cwd = read_trimmed(&fixture.scratch.join("cwd.txt"));
    let leased = fs::canonicalize(&fixture.scratch).expect("scratch directory resolves");
    let leased = leased.to_string_lossy().trim().to_owned();
    assert_eq!(child_cwd, leased);

    // Tool suppression holds from the child's own argv view.
    let child_arguments = fs::read_to_string(fixture.scratch.join("argv.txt"))
        .expect("child argv is recorded")
        .lines()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    assert!(
        child_arguments
            .windows(2)
            .any(|pair| pair == ["--tools", ""])
    );
    assert!(child_arguments.iter().any(|value| value == "--safe-mode"));
    assert!(
        child_arguments
            .windows(2)
            .any(|pair| pair == ["--mcp-config", r#"{"mcpServers":{}}"#])
    );
    assert!(
        child_arguments
            .iter()
            .any(|value| value == "--no-session-persistence")
    );

    // Ambient cwd-relative CLI behaviour anchors at the leased directory:
    // project settings and upward CLAUDE.md discovery resolve there, and git
    // context derives from it (none, because the scratch directory is not a
    // repository). These are recorded ambient behaviours, not a boundary.
    let findings = fs::read_to_string(fixture.scratch.join("findings.txt"))
        .expect("cwd-relative findings are recorded");
    assert_eq!(
        finding(&findings, "settings:"),
        format!("{leased}/.claude/settings.json")
    );
    assert_eq!(
        finding(&findings, "claude-md:"),
        format!("{leased}/CLAUDE.md")
    );
    assert_eq!(finding(&findings, "git:"), "none");

    fs::remove_dir_all(&fixture.scratch).expect("scratch directory is removable");
}

struct ProjectFixture {
    local: LocalHostServices,
    scratch: PathBuf,
    resource: WorkingResourceRef,
}

/// Builds one local host whose approved working resource is a fresh scratch
/// directory and whose approved executable is a scripted stand-in that
/// records its own cwd, argv, and cwd-relative findings before emitting the
/// frozen response-only stream.
fn project_fixture(label: &str) -> ProjectFixture {
    let sequence = SCRATCH_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let scratch = std::env::temp_dir().join(format!(
        "swallowtail-claude-code-response-location-{}-{label}-{sequence}",
        std::process::id()
    ));
    fs::create_dir_all(scratch.join(".claude")).expect("scratch .claude is created");
    fs::write(scratch.join(".claude/settings.json"), "{}\n").expect("project settings seeded");
    fs::write(scratch.join("CLAUDE.md"), "# scratch project\n").expect("project memory seeded");
    fs::write(
        scratch.join("stream.jsonl"),
        response_fixture("response-complete.jsonl"),
    )
    .expect("response stream seeded");
    let script = scratch.join("claude-fixture.sh");
    fs::write(&script, script_body(&scratch)).expect("fixture script is written");
    fs::set_permissions(&script, fs::Permissions::from_mode(0o755))
        .expect("fixture script is executable");
    let resource = WorkingResourceRef::new(RESOURCE_NAME).expect("resource is valid");
    let local = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable(
            swallowtail_runtime::ExecutableRef::new("claude.fixture.executable")
                .expect("executable is valid"),
            &script,
        )
        .approve_environment(
            swallowtail_runtime::EnvironmentRef::new(
                "claude.fixture.local-subscription-environment",
            )
            .expect("environment is valid"),
            [("PATH".into(), "/usr/bin:/bin".into())],
        )
        .approve_working_resource(resource.clone(), &scratch)
        .build_services(
            ExecutionHostId::new(format!("host.response-location.{label}")).expect("host is valid"),
        );
    ProjectFixture {
        local,
        scratch,
        resource,
    }
}

fn script_body(scratch: &Path) -> String {
    let scratch = scratch.to_str().expect("scratch path is utf-8");
    format!(
        r#"#!/bin/sh
if [ "$1" = "--version" ]; then
    printf '2.1.228 (Claude Code)\n'
    exit 0
fi
printf '%s\n' "$@" > '{scratch}/argv.txt'
pwd -P > '{scratch}/cwd.txt'
{{
    if [ -f "$PWD/.claude/settings.json" ]; then
        printf 'settings:%s\n' "$PWD/.claude/settings.json"
    else
        printf 'settings:none\n'
    fi
    anchor=none
    directory="$PWD"
    while [ "$directory" != "/" ]; do
        if [ -f "$directory/CLAUDE.md" ]; then
            anchor="$directory/CLAUDE.md"
            break
        fi
        directory=$(dirname "$directory")
    done
    printf 'claude-md:%s\n' "$anchor"
    if git_top=$(git -C "$PWD" rev-parse --show-toplevel 2>/dev/null); then
        printf 'git:%s\n' "$git_top"
    else
        printf 'git:none\n'
    fi
}} > '{scratch}/findings.txt'
cat '{scratch}/stream.jsonl'
"#
    )
}

fn prepared_with_project(
    host: ExecutionHostId,
    fixture: &ProjectFixture,
) -> ClaudeCodeResponsePreparedIntegration {
    block_on(prepare_claude_code_response_only(
        response_preparation_input(host),
        probe("project"),
        fixture.local.services().clone(),
    ))
    .expect("response-only prepares against the local project host")
}

fn probe(label: &str) -> ClaudeCodeResponsePreparationProbe {
    ClaudeCodeResponsePreparationProbe::new(
        RequestId::new(format!("claude-code-response-location-{label}")).expect("request is valid"),
        ScopeId::new(format!("claude-code-response-location-{label}")).expect("scope is valid"),
        Deadline::at(FAR_FUTURE),
        DiscoveryCancellation::new(),
    )
}

fn profile_input(
    label: &str,
    working_resource: Option<WorkingResourceRef>,
) -> ClaudeCodeResponseProfileInput {
    let input = ClaudeCodeResponseProfileInput::new(
        RequestId::new(format!("claude-code-response-{label}")).expect("request is valid"),
        ClaudeCodeResponseModelSelection::new(
            ModelRouteId::new(format!("claude-code-response.{label}")).expect("route is valid"),
            ModelRouteRevision::new("1").expect("route revision is valid"),
            ModelId::new("claude-sonnet-5").expect("model is valid"),
        ),
        OperationContent::new("return JSON-shaped text").expect("content is valid"),
        Deadline::at(FAR_FUTURE),
    );
    match working_resource {
        Some(resource) => input.with_working_resource(resource),
        None => input,
    }
}

fn profile(
    prepared: &ClaudeCodeResponsePreparedIntegration,
    label: &str,
    working_resource: Option<WorkingResourceRef>,
) -> ClaudeCodeResponsePreparedRun {
    prepared
        .prepare_run(profile_input(label, working_resource))
        .expect("response-only run prepares")
}

fn read_trimmed(path: &Path) -> String {
    fs::read_to_string(path)
        .expect("fixture output is readable")
        .trim()
        .to_owned()
}

fn finding(findings: &str, prefix: &str) -> String {
    findings
        .lines()
        .find_map(|line| line.strip_prefix(prefix).map(str::to_owned))
        .unwrap_or_else(|| panic!("finding {prefix} is present"))
}
