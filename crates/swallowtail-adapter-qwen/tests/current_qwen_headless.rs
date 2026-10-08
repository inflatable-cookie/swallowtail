mod support;

use futures_executor::block_on;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use support::{
    FakeProcessService, FakeWorkingResourceService, PendingTimeService,
    host_services_with_resource, plan_at_package_version, request_for,
};
use swallowtail_adapter_qwen::QwenHeadlessDriver;
use swallowtail_core::{ExecutionHostId, ResourceAccess, ResourceRepresentation};
use swallowtail_runtime::{
    CleanupOutcome, EnvironmentRef, StructuredRunDriver, TerminalStatus, WorkingResourceRef,
};

const SSH_CONNECTION_HASH: &str =
    "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
static NEXT_FIXTURE: AtomicUsize = AtomicUsize::new(0);

struct FixtureDirectory(PathBuf);

struct Invocation {
    started: bool,
    error_code: Option<String>,
    arguments: Vec<String>,
    cleanup: CleanupOutcome,
    releases: usize,
    resolved: Option<support::ResolvedWorkingResource>,
}

impl FixtureDirectory {
    fn new() -> Self {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time is after the epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "swallowtail-qwen-currentness-{}-{}-{}",
            std::process::id(),
            nonce,
            NEXT_FIXTURE.fetch_add(1, Ordering::SeqCst)
        ));
        fs::create_dir(&path).expect("fresh Qwen fixture directory is created");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for FixtureDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn reserved_workspace(root: &Path) -> PathBuf {
    let workspace = root
        .join("qwen-home")
        .join("ssh-workspaces")
        .join(SSH_CONNECTION_HASH)
        .join("workspace");
    fs::create_dir_all(&workspace).expect("reserved Qwen fixture path is created");
    workspace
}

fn invoke_with_resource(path: &Path) -> Invocation {
    let transcript = include_str!("fixtures/qwen-code-v0.19.11/success.jsonl").replace(
        "\"qwen_code_version\":\"0.19.11\"",
        "\"qwen_code_version\":\"0.25.0\"",
    );
    let (process, process_state) = FakeProcessService::completed(&transcript);
    let resource_service = FakeWorkingResourceService::new(path.to_string_lossy().into_owned());
    let state = resource_service.state();
    let host = ExecutionHostId::new("host.local").expect("valid host id");
    let (services, _) = host_services_with_resource(
        host,
        process,
        Arc::new(PendingTimeService),
        resource_service,
    );
    let driver = QwenHeadlessDriver::new(
        EnvironmentRef::new("qwen-saved-environment").expect("valid environment"),
    );
    let start = block_on(driver.start_run(
        plan_at_package_version("0.25.0"),
        request_for(
            "current-qwen-headless",
            WorkingResourceRef::new("workspace.current").expect("valid resource"),
        ),
        services,
    ));
    match start {
        Err(error) => Invocation {
            started: process_state.started(),
            error_code: Some(error.diagnostic().code().to_owned()),
            arguments: vec![],
            cleanup: CleanupOutcome::NotApplicable,
            releases: state.releases(),
            resolved: state.resolved(),
        },
        Ok(mut run) => {
            let terminal = block_on(
                run.take_terminal_outcome()
                    .expect("Qwen terminal outcome is available"),
            );
            let request = process_state.request();
            let cleanup = block_on(run.close());
            assert_eq!(terminal.status(), &TerminalStatus::Completed);
            assert_eq!(request.environments, ["qwen-saved-environment"]);
            assert_eq!(
                request.working_resource.as_deref(),
                Some("workspace.current")
            );
            Invocation {
                started: process_state.started(),
                error_code: None,
                arguments: request.arguments,
                cleanup,
                releases: state.releases(),
                resolved: state.resolved(),
            }
        }
    }
}

fn selected_arguments() -> Vec<String> {
    let protocol: Value =
        serde_json::from_str(include_str!("fixtures/qwen-code-v0.19.11/protocol.json"))
            .expect("selected Qwen protocol fixture is valid");
    protocol["invocation"]["arguments"]
        .as_array()
        .expect("selected Qwen arguments are an array")
        .iter()
        .map(|value| {
            value
                .as_str()
                .expect("selected Qwen argument is text")
                .replace("{model_id}", "qwen3-coder-plus")
        })
        .collect()
}

#[test]
fn exact_current_qwen_invocation_keeps_local_resource_and_delegated_auth() {
    let workspace =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/qwen-code-v0.19.11");
    let invocation = invoke_with_resource(&workspace);

    assert!(invocation.started);
    assert_eq!(invocation.error_code, None);
    assert_eq!(invocation.cleanup, CleanupOutcome::Clean);
    assert_eq!(invocation.releases, 1);
    let resolved = invocation.resolved.expect("ordinary resource was resolved");
    assert_eq!(resolved.reference, "workspace.current");
    assert_eq!(resolved.path, workspace.to_string_lossy().as_ref());
    assert_eq!(resolved.access, ResourceAccess::Read);
    assert_eq!(resolved.representation, ResourceRepresentation::Filesystem);
    assert_eq!(invocation.arguments, selected_arguments());
}

#[test]
fn reserved_ssh_workspace_and_path_aliases_fail_before_provider_work() {
    let fixture = FixtureDirectory::new();
    let workspace = reserved_workspace(fixture.path());
    let normalized_alias = workspace.join("..").join("workspace");
    let mut aliases = vec![workspace.clone(), normalized_alias];
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let symlink_alias = fixture.path().join("workspace-link");
        symlink(&workspace, &symlink_alias).expect("workspace symlink alias is created");
        aliases.push(symlink_alias);
    }

    for path in aliases {
        let invocation = invoke_with_resource(&path);
        assert!(!invocation.started);
        assert_eq!(
            invocation.error_code.as_deref(),
            Some("swallowtail.qwen.headless.ssh_workspace_rejected")
        );
        assert_eq!(invocation.releases, 1);
        let resolved = invocation.resolved.expect("reserved resource was resolved");
        assert_eq!(
            Path::new(&resolved.path)
                .canonicalize()
                .expect("resolved alias path"),
            workspace
                .canonicalize()
                .expect("reserved path canonicalizes")
        );
    }
}
