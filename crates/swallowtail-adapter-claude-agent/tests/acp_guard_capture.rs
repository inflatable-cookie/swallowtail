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
    LocalExpectedSource,
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
