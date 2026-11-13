#![cfg(unix)]

#[path = "acp_guard_prepared_child.rs"]
mod prepared_child;
#[allow(dead_code, unused_imports)]
#[path = "support/mod.rs"]
mod support;

use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use swallowtail_adapter_claude_agent::CLAUDE_AGENT_ACP_GUARD_LOADER_SOURCE;
use swallowtail_host_local::{
    LocalCapturedGraph, LocalCapturedImport, LocalCapturedNodeRecipe, LocalCapturedSources,
    LocalExecutableLaunch, LocalExpectedSource, LocalProcessHost, LocalProcessLimits,
};
use swallowtail_runtime::{
    ExecutableRef, ProcessInputChunk, ProcessRequest, ScopeId, WorkingResourceRef,
};

const IDLE: &str = "export class IdleCompaction { constructor() { this.armed = 0; this.fired = 0; } onIdle() { this.armed++; } async fire() { this.fired++; } }";
const ENTRY: &str = "import { IdleCompaction } from './idle.mjs'; const idle = new IdleCompaction(); idle.onIdle(); await idle.fire(); process.stdout.write(JSON.stringify({armed: idle.armed, fired: idle.fired}));";

fn fixture() -> PathBuf {
    let template = std::env::temp_dir().join("acp-captured-child.XXXXXXXX");
    let output = Command::new("/usr/bin/mktemp")
        .arg("-d")
        .arg(template)
        .env_clear()
        .output()
        .expect("mktemp starts");
    assert!(output.status.success());
    PathBuf::from(
        String::from_utf8(output.stdout)
            .expect("fixture path")
            .trim(),
    )
}

fn capture(root: &Path, entry: &str) -> LocalCapturedGraph {
    std::fs::write(root.join("entry.mjs"), entry).expect("entry fixture");
    std::fs::write(root.join("idle.mjs"), IDLE).expect("idle fixture");
    let sources = LocalCapturedSources::capture(
        root,
        &[
            LocalExpectedSource::new("entry.mjs", Sha256::digest(entry.as_bytes()).into()),
            LocalExpectedSource::new("idle.mjs", Sha256::digest(IDLE.as_bytes()).into()),
        ],
    )
    .expect("capture fake sources");
    LocalCapturedGraph::new(
        &sources,
        Path::new("entry.mjs"),
        Path::new("idle.mjs"),
        &[LocalCapturedImport::new(
            "entry.mjs",
            "./idle.mjs",
            "idle.mjs",
        )],
    )
    .expect("capture fake graph")
}

fn node(arguments: &[&str]) -> Output {
    let search_path = std::env::var_os("PATH").expect("fixture node search path");
    Command::new("node")
        .args(arguments)
        .env_clear()
        .env("PATH", search_path)
        .output()
        .expect("fake Node process starts")
}

fn guarded_with_bootstrap(recipe: &LocalCapturedNodeRecipe, bootstrap: &[u8]) -> Output {
    use std::io::Write;
    use std::process::Stdio;
    let search_path = std::env::var_os("PATH").expect("fixture node search path");
    let mut child = Command::new("node")
        .args(recipe.prefix_arguments())
        .env_clear()
        .env("PATH", search_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("fake Node process starts");
    child
        .stdin
        .take()
        .expect("child input")
        .write_all(bootstrap)
        .expect("fake bootstrap writes");
    child.wait_with_output().expect("fake child joins")
}

fn guarded(graph: &LocalCapturedGraph) -> Output {
    let recipe = LocalCapturedNodeRecipe::new(
        graph,
        CLAUDE_AGENT_ACP_GUARD_LOADER_SOURCE,
        [1; 32],
        [3; 32],
        [5; 32],
    )
    .expect("fake recipe");
    guarded_with_bootstrap(&recipe, recipe.bootstrap())
}

#[test]
fn captured_host_graph_drives_fake_node_after_source_replacement() {
    let root = fixture();
    let graph = capture(&root, ENTRY);
    let entry_path = root.join("entry.mjs");
    let plain = node(&[entry_path.to_str().expect("fixture entry")]);
    assert!(
        plain.status.success(),
        "{}",
        String::from_utf8_lossy(&plain.stderr)
    );
    assert_eq!(plain.stdout, br#"{"armed":1,"fired":1}"#);
    std::fs::write(
        root.join("entry.mjs"),
        "throw new Error('entry replacement')",
    )
    .expect("replace entry");
    std::fs::write(root.join("idle.mjs"), "throw new Error('idle replacement')")
        .expect("replace idle");
    let child = guarded(&graph);
    assert!(
        child.status.success(),
        "{}",
        String::from_utf8_lossy(&child.stderr)
    );
    assert_eq!(child.stdout, br#"{"armed":0,"fired":0}"#);
}

#[test]
fn unapproved_import_refuses_before_fake_entry_effects() {
    let root = fixture();
    let entry =
        "import 'node:worker_threads'; import './idle.mjs'; process.stdout.write('entry effect');";
    let graph = capture(&root, entry);
    let child = guarded(&graph);
    assert!(!child.status.success());
    assert!(child.stdout.is_empty());
    assert!(String::from_utf8_lossy(&child.stderr).contains("unapproved module edge"));
}

#[test]
fn truncated_graph_bootstrap_fails_before_fake_entry_effects() {
    let root = fixture();
    let graph = capture(&root, ENTRY);
    let recipe = LocalCapturedNodeRecipe::new(
        &graph,
        CLAUDE_AGENT_ACP_GUARD_LOADER_SOURCE,
        [1; 32],
        [3; 32],
        [5; 32],
    )
    .expect("recipe");
    let child = guarded_with_bootstrap(&recipe, &recipe.bootstrap()[..5]);
    assert!(!child.status.success());
    assert!(child.stdout.is_empty());
    assert!(String::from_utf8_lossy(&child.stderr).contains("incomplete graph bootstrap"));
}

#[test]
fn invalid_graph_bootstrap_refuses_before_fake_entry_effects() {
    let root = fixture();
    let graph = capture(&root, ENTRY);
    let recipe = LocalCapturedNodeRecipe::new(
        &graph,
        CLAUDE_AGENT_ACP_GUARD_LOADER_SOURCE,
        [1; 32],
        [3; 32],
        [5; 32],
    )
    .expect("recipe");
    for (bootstrap, diagnostic) in [
        (vec![0, 0, 0], "incomplete graph bootstrap"),
        (
            (33_554_433_u32).to_be_bytes().to_vec(),
            "graph bootstrap too large",
        ),
        (vec![0, 0, 0, 1, b'{'], "SyntaxError"),
    ] {
        let child = guarded_with_bootstrap(&recipe, &bootstrap);
        assert!(!child.status.success());
        assert!(child.stdout.is_empty());
        assert!(
            String::from_utf8_lossy(&child.stderr).contains(diagnostic),
            "{}",
            String::from_utf8_lossy(&child.stderr)
        );
    }
    let without_admission = &recipe.bootstrap()[..recipe.bootstrap().len() - 1];
    let child = guarded_with_bootstrap(&recipe, without_admission);
    assert!(!child.status.success());
    assert!(child.stdout.is_empty());
    assert!(String::from_utf8_lossy(&child.stderr).contains("incomplete graph bootstrap"));
    let mut wrong_admission = recipe.bootstrap().to_vec();
    *wrong_admission.last_mut().expect("admission byte") = 0;
    let child = guarded_with_bootstrap(&recipe, &wrong_admission);
    assert!(!child.status.success());
    assert!(child.stdout.is_empty());
    assert!(String::from_utf8_lossy(&child.stderr).contains("invalid bootstrap admission"));
}

fn bootstrap_host(root: &Path, script: &str) -> (LocalProcessHost, ProcessRequest) {
    let node = std::env::split_paths(&std::env::var_os("PATH").expect("PATH"))
        .map(|dir| dir.join("node"))
        .find(|candidate| candidate.is_file())
        .expect("fake Node interpreter");
    let executable = ExecutableRef::new("bootstrap.fake").expect("executable");
    let resource = WorkingResourceRef::new("bootstrap.workspace").expect("resource");
    let launch = LocalExecutableLaunch::new(node)
        .with_prefix_arguments([
            std::ffi::OsString::from("-e"),
            std::ffi::OsString::from(script),
        ])
        .with_node_startup_environment_filter();
    let host = LocalProcessHost::builder(LocalProcessLimits::new(8, 4096, 4, 4096, 4096))
        .approve_executable_launch(executable.clone(), launch)
        .approve_working_resource(resource.clone(), root)
        .build();
    (
        host,
        ProcessRequest::new(executable).with_working_resource(resource),
    )
}

#[test]
fn bootstrap_host_bounds_stalled_and_closed_pipes_and_rejects_oversize_before_spawn() {
    use std::time::{Duration, Instant};
    let root = fixture();
    let scope = ScopeId::new("bootstrap.test").expect("scope");
    let bytes = vec![b'x'; 1024 * 1024];
    let (host, request) = bootstrap_host(&root, "setInterval(() => {}, 1000)");
    let started = Instant::now();
    let error = futures_executor::block_on(host.start_with_bootstrap(
        scope.clone(),
        request,
        &bytes,
        Duration::from_millis(100),
    ))
    .err()
    .expect("never-reading child times out");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.local_process.bootstrap_timeout"
    );
    assert!(started.elapsed() < Duration::from_secs(8));

    let (host, request) = bootstrap_host(
        &root,
        "require('fs').closeSync(0); setInterval(() => {}, 1000)",
    );
    let error = futures_executor::block_on(host.start_with_bootstrap(
        scope.clone(),
        request,
        &bytes,
        Duration::from_secs(2),
    ))
    .err()
    .expect("closed child input fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.local_process.bootstrap_write_failed"
    );

    let (host, request) = bootstrap_host(&root, "require('fs').writeFileSync('spawned', 'yes')");
    let error = futures_executor::block_on(host.start_with_bootstrap(
        scope,
        request,
        &vec![0; 32 * 1024 * 1024 + 6],
        Duration::from_secs(1),
    ))
    .err()
    .expect("oversized bootstrap refused");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.local_process.bootstrap_limit"
    );
    assert!(!root.join("spawned").exists());
}

#[tokio::test]
async fn cancelled_bootstrap_stops_and_reaps_the_owned_child() {
    use std::time::{Duration, Instant};
    let root = fixture();
    let (host, request) = bootstrap_host(
        &root,
        "require('fs').writeFileSync('child.pid', String(process.pid)); setInterval(() => {}, 1000)",
    );
    let bytes = vec![b'x'; 1024 * 1024];
    let result = tokio::time::timeout(
        Duration::from_millis(500),
        host.start_with_bootstrap(
            ScopeId::new("bootstrap.cancel").expect("scope"),
            request,
            &bytes,
            Duration::from_secs(5),
        ),
    )
    .await;
    assert!(result.is_err(), "stalled write must be cancelled");
    let pid = std::fs::read_to_string(root.join("child.pid")).expect("child started");
    let pid: u32 = pid.parse().expect("fixture pid");
    let deadline = Instant::now() + Duration::from_secs(8);
    loop {
        let alive = Command::new("/bin/kill")
            .args(["-0", &pid.to_string()])
            .output()
            .expect("observe fixture process")
            .status
            .success();
        if !alive {
            break;
        }
        assert!(Instant::now() < deadline, "cancelled child was not reaped");
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}

#[test]
fn bootstrap_does_not_spend_provider_stdin_budget() {
    use std::time::Duration;
    let root = fixture();
    let (host, request) = bootstrap_host(
        &root,
        "let data=''; process.stdin.on('data', x => { data += x; if(data.length >= 10) { process.stdout.write(data); process.exit(0); } });",
    );
    let process = futures_executor::block_on(host.start_with_bootstrap(
        ScopeId::new("bootstrap.budget").expect("scope"),
        request,
        b"123456",
        Duration::from_secs(2),
    ))
    .expect("bootstrap completes");
    futures_executor::block_on(process.write_stdin(ProcessInputChunk::new(b"abcd".to_vec())))
        .expect("full independent provider budget remains");
    let error =
        futures_executor::block_on(process.write_stdin(ProcessInputChunk::new(b"e".to_vec())))
            .expect_err("provider budget remains enforced");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.local_process.stdin_limit_exceeded"
    );
    let mut output = Vec::new();
    while let Some(chunk) = futures_executor::block_on(process.read_output()).expect("output") {
        output.extend_from_slice(chunk.bytes());
    }
    assert_eq!(output, b"123456abcd");
    assert!(
        futures_executor::block_on(process.wait())
            .expect("child joined")
            .success()
    );
}

#[tokio::test]
async fn cancellation_before_and_after_admission_joins_owned_tree() {
    use std::time::{Duration, Instant};

    async fn wait_for(root: &Path, name: &str) -> String {
        let deadline = Instant::now() + Duration::from_secs(8);
        loop {
            if let Ok(value) = std::fs::read_to_string(root.join(name))
                && value.parse::<u32>().is_ok()
            {
                return value;
            }
            assert!(Instant::now() < deadline, "missing fixture marker {name}");
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }
    async fn terminated(pid: &str) {
        let pid: u32 = pid.parse().expect("fixture pid");
        let deadline = Instant::now() + Duration::from_secs(8);
        loop {
            let result = Command::new("/bin/ps")
                .args(["-p", &pid.to_string(), "-o", "stat="])
                .output()
                .expect("observe owned fixture");
            let state = String::from_utf8_lossy(&result.stdout);
            if !result.status.success() || state.trim().starts_with('Z') {
                return;
            }
            assert!(Instant::now() < deadline, "owned fixture was not reaped");
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }

    let before = fixture();
    let (host, request) = bootstrap_host(
        &before,
        "require('fs').writeFileSync('root.pid',String(process.pid)); setTimeout(() => { process.stdin.on('data',()=>require('fs').writeFileSync('entry','effect')); },3000); setInterval(()=>{},1000)",
    );
    let bytes = vec![0xa5; 1024 * 1024];
    let mut pending = Box::pin(host.start_with_bootstrap(
        ScopeId::new("bootstrap.before").expect("scope"),
        request,
        &bytes,
        Duration::from_secs(5),
    ));
    let root_pid = tokio::select! {
        result = &mut pending => panic!("unread bootstrap unexpectedly resolved: {}", result.is_ok()),
        pid = wait_for(&before, "root.pid") => pid,
    };
    drop(pending);
    terminated(&root_pid).await;
    assert!(!before.join("entry").exists());

    let after = fixture();
    let script = "process.stdin.once('data', b => { if(b[0] !== 165) process.exit(2); const fs=require('fs'); const c=require('child_process').spawn(process.execPath,['-e','setInterval(()=>{},1000)'],{stdio:'ignore'}); fs.writeFileSync('root.pid',String(process.pid)); fs.writeFileSync('descendant.pid',String(c.pid)); }); setInterval(()=>{},1000)";
    let (host, request) = bootstrap_host(&after, script);
    let process = host
        .start_with_bootstrap(
            ScopeId::new("bootstrap.after").expect("scope"),
            request,
            &[0xa5],
            Duration::from_secs(5),
        )
        .await
        .expect("admission committed");
    let root_pid = wait_for(&after, "root.pid").await;
    let descendant_pid = wait_for(&after, "descendant.pid").await;
    process.force_stop().await.expect("stop admitted tree");
    process.wait().await.expect("join admitted tree");
    terminated(&root_pid).await;
    terminated(&descendant_pid).await;
}
