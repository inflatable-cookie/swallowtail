use std::ffi::OsString;
use std::path::PathBuf;
use swallowtail_adapter_claude_agent::CLAUDE_CODE_RESPONSE_ONLY_AXIS;
use swallowtail_core::{ExecutionHostId, InterfaceVersionAxis};
use swallowtail_host_local::{
    LocalExecutableLaunch, LocalHostServices, LocalProcessHost, LocalProcessLimits,
};
use swallowtail_runtime::{EnvironmentRef, ExecutableRef};

pub(super) fn live_host() -> (
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

pub(super) fn git_status() -> Vec<u8> {
    std::process::Command::new("git")
        .args(["status", "--porcelain=v1", "--untracked-files=all"])
        .output()
        .expect("git status runs")
        .stdout
}
