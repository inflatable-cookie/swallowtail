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
    LocalTrustedRuntime,
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
    .canonicalize()
    .expect("canonical owned fixture")
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

fn capture_native_graph(root: &Path, entry: &str) -> LocalCapturedGraph {
    use std::os::unix::fs::PermissionsExt;
    let native_path = root.join("fake-native");
    let native_source = b"#!/bin/sh\nprintf fake-native";
    std::fs::write(&native_path, native_source).expect("owned native fake");
    std::fs::set_permissions(&native_path, std::fs::Permissions::from_mode(0o700))
        .expect("executable fake");
    let native = LocalTrustedRuntime::capture(&native_path, Sha256::digest(native_source).into())
        .expect("held fake native");
    let prefix = "import {starts} from './acp/dist/session-index/live-registry.js'; if (!(await starts()).includes(String(process.pid))) throw Error('system process helper mismatch'); import {createRequire} from 'node:module'; import {execFileSync} from 'node:child_process'; import data from './data.json' with {type:'json'}; const get = createRequire(import.meta.resolve('./sdk.mjs')); const value = get('./bridge.cjs'); if (value !== data.value) throw Error('captured data mismatch'); const native = get.resolve('@fake/native/claude'); if (execFileSync(native,['--version'],{env:{}}).toString() !== 'fake-native') throw Error('native fake mismatch');\n";
    let entry = format!("{prefix}{entry}");
    let inputs = [
        ("entry.mjs", entry.as_str(), "module"),
        ("idle.mjs", IDLE, "module"),
        ("sdk.mjs", "export const sdk = true;", "module"),
        (
            "bridge.cjs",
            "module.exports = require('./leaf.cjs');",
            "commonjs",
        ),
        ("leaf.cjs", "module.exports = 'captured';", "commonjs"),
        ("data.json", "{\"value\":\"captured\"}", "json"),
        (
            "acp/dist/session-index/live-registry.js",
            "import {execFile} from 'node:child_process'; export function starts(){return new Promise((resolve,reject)=>execFile('ps',['-o','pid=,lstart=','-p',String(process.pid)],{timeout:1000,env:{}},(error,stdout)=>error?reject(error):resolve(stdout)));}",
            "module",
        ),
    ];
    let mut files = serde_json::Map::new();
    for (name, source, format) in inputs {
        std::fs::create_dir_all(root.join(name).parent().expect("parent"))
            .expect("owned source directory");
        std::fs::write(root.join(name), source).expect("fake graph member");
        files.insert(name.into(),serde_json::json!({"sha256":format!("{:x}",Sha256::digest(source.as_bytes())),"format":format}));
    }
    let native_record = serde_json::json!({"sha256":format!("{:x}",Sha256::digest(native_source)),"bytes":native_source.len(),"specifier":"@fake/native/claude"});
    let manifest = serde_json::json!({"vendor":"11".repeat(32),"entry":"entry.mjs","idle":"idle.mjs","files":files,
        "imports":[{"parent":"entry.mjs","specifier":"./acp/dist/session-index/live-registry.js","target":"acp/dist/session-index/live-registry.js"},{"parent":"entry.mjs","specifier":"./idle.mjs","target":"idle.mjs"},{"parent":"entry.mjs","specifier":"./sdk.mjs","target":"sdk.mjs"},{"parent":"entry.mjs","specifier":"./data.json","target":"data.json"},{"parent":"sdk.mjs","specifier":"./bridge.cjs","target":"bridge.cjs"},{"parent":"bridge.cjs","specifier":"./leaf.cjs","target":"leaf.cjs"}],
        "builtins":[{"parent":"acp/dist/session-index/live-registry.js","specifier":"node:child_process","target":"node:child_process"},{"parent":"entry.mjs","specifier":"node:module","target":"node:module"},{"parent":"entry.mjs","specifier":"node:child_process","target":"node:child_process"}],
        "require_parents":[["entry.mjs","sdk.mjs"]],"native_parents":["sdk.mjs"],"native":{"darwin-arm64":native_record,"linux-x64":native_record},"platform_programs":[{"parent":"acp/dist/session-index/live-registry.js","name":"ps","arguments":"live-process-starts-v1"}],"denied_dynamic":[],"source_cautions":[]});
    let bytes = serde_json::to_vec(&manifest).expect("fake manifest");
    let platform = if cfg!(target_os = "macos") {
        "darwin-arm64"
    } else {
        "linux-x64"
    };
    let ps_path = if cfg!(target_os = "macos") {
        Path::new("/bin/ps")
    } else {
        Path::new("/usr/bin/ps")
    };
    let ps = LocalTrustedRuntime::capture(
        ps_path,
        Sha256::digest(std::fs::read(ps_path).expect("system helper source")).into(),
    )
    .expect("trusted platform ps");
    assert!(
        LocalCapturedGraph::capture_manifest(root, &bytes, platform, &native, None).is_err(),
        "missing system helper custody"
    );
    for change in ["worker", "format", "source", "edge", "native", "caution"] {
        let mut changed = manifest.clone();
        match change {
            "worker" => changed["builtins"][0]["target"] = "node:worker_threads".into(),
            "format" => changed["files"]["entry.mjs"]["format"] = "unknown".into(),
            "source" => changed["files"]["entry.mjs"]["sha256"] = "00".repeat(32).into(),
            "edge" => changed["imports"][0]["target"] = "missing.mjs".into(),
            "native" => changed["native"][platform]["sha256"] = "00".repeat(32).into(),
            "caution" => {
                changed["source_cautions"] = serde_json::json!([{"kind":"worker-nonliteral"}])
            }
            _ => unreachable!(),
        }
        assert!(
            LocalCapturedGraph::capture_manifest(
                root,
                &serde_json::to_vec(&changed).expect("changed manifest"),
                platform,
                &native,
                Some(&ps)
            )
            .is_err(),
            "{change}"
        );
    }
    LocalCapturedGraph::capture_manifest(root, &bytes, platform, &native, Some(&ps))
        .expect("admitted fake manifest")
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
fn captured_recipe_refuses_dynamic_code_and_native_addons() {
    let root = fixture();
    for expression in [
        "eval(\"process.stdout.write('effect')\")",
        "Function(\"process.stdout.write('effect')\")()",
        "process.dlopen({}, '/unapproved.node')",
        "process.getBuiltinModule('module')",
    ] {
        let graph = capture(&root, &format!("import './idle.mjs'; {expression};"));
        let output = guarded(&graph);
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
    }
}

#[test]
fn synchronous_capture_serves_aliased_commonjs_and_refuses_unknown_edges() {
    let root = fixture();
    let entry = "import {createRequire as make} from 'node:module'; import './idle.mjs'; const get = make(import.meta.url); const name = get.resolve('./nested.cjs'); process.stdout.write(get(name).value);";
    let nested = "module.exports = {value: require('./value.cjs')};";
    let value = "module.exports = 'captured';";
    let inputs = [
        ("entry.mjs", entry),
        ("idle.mjs", IDLE),
        ("nested.cjs", nested),
        ("value.cjs", value),
    ];
    for (name, source) in inputs {
        std::fs::write(root.join(name), source).expect("fixture source");
    }
    let manifest: Vec<_> = inputs
        .iter()
        .map(|(name, source)| {
            LocalExpectedSource::new(*name, Sha256::digest(source.as_bytes()).into())
        })
        .collect();
    let sources = LocalCapturedSources::capture(&root, &manifest).expect("capture sources");
    let imports = [
        LocalCapturedImport::new("entry.mjs", "./idle.mjs", "idle.mjs"),
        LocalCapturedImport::new("entry.mjs", "./nested.cjs", "nested.cjs"),
        LocalCapturedImport::new("entry.mjs", "/swallowtail-capture/nested.cjs", "nested.cjs"),
        LocalCapturedImport::new("nested.cjs", "./value.cjs", "value.cjs"),
    ];
    let graph = LocalCapturedGraph::with_builtins(
        &sources,
        Path::new("entry.mjs"),
        Path::new("idle.mjs"),
        &imports,
        &[(PathBuf::from("entry.mjs"), "node:module".into())],
    )
    .expect("fake graph");
    std::fs::write(root.join("value.cjs"), "throw Error('replacement')").expect("replace source");
    let child = guarded(&graph);
    assert!(
        child.status.success(),
        "{}",
        String::from_utf8_lossy(&child.stderr)
    );
    assert_eq!(child.stdout, b"captured");
    let without_builtin = LocalCapturedGraph::new(
        &sources,
        Path::new("entry.mjs"),
        Path::new("idle.mjs"),
        &imports,
    )
    .expect("graph without builtin");
    assert!(!guarded(&without_builtin).status.success());
}

#[test]
fn captured_module_facade_refuses_hook_replacement_and_foreign_require_parents() {
    for action in [
        "if (module.registerHooks || module.Module || module._load) throw Error('raw module escaped'); process.stdout.write('bounded');",
        "module.createRequire('file:///unapproved/parent.mjs'); process.stdout.write('effect');",
        "module.createRequire(import.meta.url).resolve('./missing.cjs', {paths:['/unapproved']}); process.stdout.write('effect');",
    ] {
        let root = fixture();
        let entry = format!("import module from 'node:module'; import './idle.mjs'; {action}");
        std::fs::write(root.join("entry.mjs"), &entry).expect("entry");
        std::fs::write(root.join("idle.mjs"), IDLE).expect("idle");
        let sources = LocalCapturedSources::capture(
            &root,
            &[
                LocalExpectedSource::new("entry.mjs", Sha256::digest(entry.as_bytes()).into()),
                LocalExpectedSource::new("idle.mjs", Sha256::digest(IDLE.as_bytes()).into()),
            ],
        )
        .expect("capture");
        let graph = LocalCapturedGraph::with_builtins(
            &sources,
            Path::new("entry.mjs"),
            Path::new("idle.mjs"),
            &[LocalCapturedImport::new(
                "entry.mjs",
                "./idle.mjs",
                "idle.mjs",
            )],
            &[("entry.mjs".into(), "node:module".into())],
        )
        .expect("graph");
        let output = guarded(&graph);
        if action.contains("bounded") {
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            assert_eq!(output.stdout, b"bounded");
        } else {
            assert!(!output.status.success());
            assert!(output.stdout.is_empty());
        }
    }
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
    let mut pending = Box::pin(host.start_with_bootstrap(
        ScopeId::new("bootstrap.cancel").expect("scope"),
        request,
        &bytes,
        Duration::from_secs(15),
    ));
    let started = Instant::now();
    while !root.join("child.pid").exists() {
        tokio::select! {
            result = &mut pending => panic!("bootstrap completed before cancellation: {}", result.is_ok()),
            () = tokio::time::sleep(Duration::from_millis(20)) => {}
        }
        assert!(
            started.elapsed() < Duration::from_secs(10),
            "child readiness was not observed"
        );
    }
    drop(pending);
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

#[test]
fn trusted_runtime_custody_detects_drift_replacement_and_revocation() {
    use std::io::Write;
    use std::os::unix::fs::{OpenOptionsExt, PermissionsExt, symlink};
    let root = fixture();
    let runtime = root.join("runtime");
    let mut fixture = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o700)
        .open(&runtime)
        .expect("runtime fixture");
    fixture
        .write_all(b"host-selected runtime")
        .expect("runtime bytes");
    let digest = Sha256::digest(b"host-selected runtime").into();
    assert!(LocalTrustedRuntime::capture(&runtime, [0; 32]).is_err());
    let held = LocalTrustedRuntime::capture(&runtime, digest).expect("host-owned runtime");
    assert_eq!(held.digest(), &digest);
    held.verify().expect("unchanged runtime");
    std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o770))
        .expect("make runtime ancestor group writable");
    assert!(held.verify().is_err(), "writable ancestor is refused");
    assert!(LocalTrustedRuntime::capture(&runtime, digest).is_err());
    std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o700))
        .expect("restore runtime ancestor");
    held.verify().expect("restored ancestor");
    std::fs::set_permissions(&runtime, std::fs::Permissions::from_mode(0o720))
        .expect("fixture permissions");
    assert!(held.verify().is_err(), "group-writable runtime is refused");
    std::fs::set_permissions(&runtime, std::fs::Permissions::from_mode(0o700))
        .expect("restore fixture permissions");
    assert!(
        held.verify_launch(&LocalExecutableLaunch::new(&runtime))
            .is_err()
    );
    let approved = LocalExecutableLaunch::new(&runtime).with_node_startup_environment_filter();
    held.verify_launch(&approved)
        .expect("matching filtered launch");
    let mismatch =
        LocalExecutableLaunch::new(root.join("other")).with_node_startup_environment_filter();
    assert!(held.verify_launch(&mismatch).is_err());
    let alias = root.join("alias");
    symlink(&runtime, &alias).expect("alias fixture");
    assert!(LocalTrustedRuntime::capture(&alias, digest).is_err());
    std::fs::write(&runtime, b"changed in place").expect("mutate runtime");
    assert!(held.verify().is_err());
    std::fs::write(&runtime, b"host-selected runtime").expect("restore bytes");
    held.verify().expect("held file matches again");
    let replacement = root.join("replacement");
    let mut replacement_file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o700)
        .open(&replacement)
        .expect("replacement");
    replacement_file
        .write_all(b"host-selected runtime")
        .expect("replacement bytes");
    std::fs::rename(replacement, &runtime).expect("replace path");
    assert!(
        held.verify().is_err(),
        "equal bytes do not preserve file custody"
    );
    let fresh = LocalTrustedRuntime::capture(&runtime, digest).expect("new custody");
    let clone = fresh.clone();
    fresh.revoke();
    assert!(clone.verify().is_err(), "revocation covers clones");

    let directory = root.join("owned");
    std::fs::create_dir(&directory).expect("ancestor fixture");
    let nested = directory.join("node");
    std::fs::copy(&runtime, &nested).expect("nested runtime");
    let nested_held = LocalTrustedRuntime::capture(&nested, digest).expect("nested custody");
    std::fs::rename(&directory, root.join("moved")).expect("move ancestor");
    symlink(root.join("moved"), &directory).expect("replace ancestor with link");
    assert!(nested_held.verify().is_err(), "ancestor symlink is refused");
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
