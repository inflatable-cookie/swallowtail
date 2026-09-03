//! Descendant-tree ownership proofs for foreign-language SDK sidecars.
//!
//! Contract 019 requires the execution host to own and terminate the whole
//! descendant tree rooted at a sidecar, and requires the launch recipe to
//! prove descendant enrollment on every supported platform. These cases run
//! the same portable topology wherever the suite runs, so enrollment is
//! observed rather than asserted, and they include the counterexample the
//! Review Oracle names: a nearest-child join that succeeds while a
//! provider-owned descendant survives.

use super::*;
use std::path::Path;
use std::time::{Duration, Instant};

/// The fixture descendant lives for three seconds; every wait here is longer
/// so an absent marker means termination, not impatience.
const DESCENDANT_LIFETIME: Duration = Duration::from_secs(3);
const MARKER_TIMEOUT: Duration = Duration::from_secs(8);

#[test]
fn host_tree_termination_reaches_the_native_grandchild() {
    let resource_directory = temporary_resource();
    let (host, executable, environment, resource) = fixture_host(
        "sidecar-with-native-descendant",
        LocalProcessLimits::default(),
        &resource_directory,
    );
    let process = start(&host, request(&executable, &environment, &resource))
        .expect("sidecar fixture starts");
    await_marker(&resource_directory, "descendant-started");

    // The sidecar is still running: only the host's tree authority can reach
    // the descendant the SDK launched.
    block_on(process.force_stop()).expect("host terminates the descendant tree");
    let _ = block_on(process.wait());
    std::thread::sleep(DESCENDANT_LIFETIME + Duration::from_secs(1));
    assert!(
        !resource_directory.join("descendant-survived").exists(),
        "the native descendant must not survive host tree termination"
    );
    std::fs::remove_dir_all(resource_directory).expect("fixture resource is removed");
}

#[test]
fn a_nearest_child_join_without_host_tree_ownership_leaves_the_descendant_running() {
    // Control: the same topology started outside the host's tree authority.
    // The nearest child exits cleanly and its join reports success, yet the
    // provider-owned descendant runs on. A nearest-child join is therefore
    // never evidence that the tree stopped.
    let control_directory = temporary_resource();
    let mut control = std::process::Command::new(std::env::current_exe().expect("test executable"))
        .args(fixture_arguments())
        .env_clear()
        .envs(fixture_environment("sidecar-exits-with-native-descendant"))
        .current_dir(&control_directory)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("control sidecar starts");
    let control_exit = control.wait().expect("the nearest child joins");
    assert!(control_exit.success(), "the nearest child exited cleanly");
    await_marker(&control_directory, "descendant-survived");
    std::fs::remove_dir_all(control_directory).expect("control resource is removed");

    // The host-owned launch recipe covers the same topology: the descendant
    // is enrolled in the tree and never reaches its survival marker.
    let resource_directory = temporary_resource();
    let (host, executable, environment, resource) = fixture_host(
        "sidecar-exits-with-native-descendant",
        LocalProcessLimits::default(),
        &resource_directory,
    );
    let process = start(&host, request(&executable, &environment, &resource))
        .expect("sidecar fixture starts");
    await_marker(&resource_directory, "descendant-started");
    let _ = block_on(process.wait());
    std::thread::sleep(DESCENDANT_LIFETIME + Duration::from_secs(1));
    assert!(
        !resource_directory.join("descendant-survived").exists(),
        "host tree ownership, not the nearest-child join, is what stopped the descendant"
    );
    std::fs::remove_dir_all(resource_directory).expect("fixture resource is removed");
}

fn await_marker(directory: &Path, marker: &str) {
    let path = directory.join(marker);
    let deadline = Instant::now() + MARKER_TIMEOUT;
    while !path.exists() {
        assert!(
            Instant::now() < deadline,
            "fixture marker {marker} never appeared"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
}
