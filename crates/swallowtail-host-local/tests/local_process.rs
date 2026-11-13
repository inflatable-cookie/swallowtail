#[path = "local_process/attestation.rs"]
mod attestation;
#[path = "local_process/descendant_tree.rs"]
mod descendant_tree;
#[path = "local_process/supervision.rs"]
mod supervision;
#[path = "local_process/support.rs"]
mod support;

use std::ffi::OsString;
use support::*;
use swallowtail_core::InterfaceVersionAxis;
use swallowtail_host_local::{LocalExecutableLaunch, LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    CleanupOutcome, InstalledExecutableTarget, ProcessInputChunk, ProcessOutputStream,
    ProcessRequest, ResourceAccess, ResourceRepresentation, WorkingResourceService,
};

#[test]
fn only_host_approved_references_and_arguments_spawn() {
    let executable = executable_ref();
    let environment = environment_ref();
    let working_resource = working_resource_ref();
    let limits = LocalProcessLimits::new(3, 1024, 64, 1024, 1024);
    let empty = LocalProcessHost::builder(limits).build();

    assert_failure_code(
        start(
            &empty,
            request(&executable, &environment, &working_resource),
        ),
        "swallowtail.local_process.executable_not_approved",
    );

    let executable_only = LocalProcessHost::builder(limits)
        .approve_executable(
            executable.clone(),
            std::env::current_exe().expect("test executable"),
        )
        .build();
    assert_failure_code(
        start(
            &executable_only,
            request(&executable, &environment, &working_resource),
        ),
        "swallowtail.local_process.environment_not_approved",
    );

    let environment_only = LocalProcessHost::builder(limits)
        .approve_executable(
            executable.clone(),
            std::env::current_exe().expect("test executable"),
        )
        .approve_environment(environment.clone(), fixture_environment("echo"))
        .build();
    assert_failure_code(
        start(
            &environment_only,
            request(&executable, &environment, &working_resource),
        ),
        "swallowtail.local_process.working_resource_not_approved",
    );

    let arguments_limited = LocalProcessHost::builder(LocalProcessLimits::new(1, 8, 64, 64, 64))
        .approve_executable(
            executable.clone(),
            std::env::current_exe().expect("test executable"),
        )
        .build();
    assert_failure_code(
        start(
            &arguments_limited,
            ProcessRequest::new(executable).with_arguments(fixture_arguments()),
        ),
        "swallowtail.local_process.argument_limit_exceeded",
    );
}

#[test]
fn missing_executable_emits_bounded_not_found_diagnostic() {
    let directory = temporary_resource();
    let missing_path = directory.join("missing-executable");
    let executable = executable_ref();
    let host = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable(executable.clone(), missing_path.clone())
        .build();

    let failure = start(&host, ProcessRequest::new(executable))
        .err()
        .expect("missing executable must fail at the host boundary");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_process.executable_not_found"
    );
    assert_eq!(
        failure.diagnostic().message(),
        "Local executable was not found"
    );
    assert!(!format!("{failure:?}").contains(missing_path.to_string_lossy().as_ref()));

    std::fs::remove_dir_all(directory).expect("fixture directory is removed");
}

#[test]
fn non_not_found_spawn_error_remains_safe_failed_diagnostic() {
    let directory = temporary_resource();
    let executable = executable_ref();
    let host = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable(executable.clone(), directory.clone())
        .build();

    let failure = start(&host, ProcessRequest::new(executable))
        .err()
        .expect("a directory is not a spawnable executable");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_process.spawn_failed"
    );
    assert_eq!(
        failure.diagnostic().message(),
        "Local process could not be started"
    );
    assert!(!format!("{failure:?}").contains(directory.to_string_lossy().as_ref()));

    std::fs::remove_dir_all(directory).expect("fixture directory is removed");
}

#[test]
fn installed_version_probe_uses_only_the_explicit_approved_target_and_joins() {
    let resource_directory = temporary_resource();
    let limits = LocalProcessLimits::new(8, 1024, 64, 1024, 1024);
    let (host, executable, environment, resource) =
        fixture_host("version", limits, &resource_directory);
    let target = InstalledExecutableTarget::new(
        executable.clone(),
        InterfaceVersionAxis::new("fixture.harness.package").expect("axis is valid"),
    );
    let probe = ProcessRequest::new(target.executable().clone())
        .with_arguments(fixture_arguments())
        .with_environment([environment])
        .with_working_resource(resource);
    let process = start(&host, probe).expect("approved target starts");
    block_on(process.close_stdin()).expect("probe input closes");
    let (stdout, stderr) = collect_output(process.as_ref()).expect("probe output is bounded");
    assert!(stderr.is_empty());
    assert!(
        String::from_utf8_lossy(&stdout).contains("fixture-harness 1.2.0\n"),
        "bounded fixture output must contain the exact version line"
    );
    assert!(
        block_on(process.wait())
            .expect("probe child joins")
            .success()
    );
    assert!(!format!("{target:?}").contains("fixture-local-process"));

    let unapproved = LocalProcessHost::builder(limits).build();
    assert_failure_code(
        start(
            &unapproved,
            ProcessRequest::new(target.executable().clone()).with_arguments(fixture_arguments()),
        ),
        "swallowtail.local_process.executable_not_approved",
    );

    std::fs::remove_dir_all(resource_directory).expect("fixture resource is removed");
}

#[test]
fn env_shebang_without_interpreter_prefix_fails_with_host_recipe_hint() {
    let directory = std::env::temp_dir().join(format!(
        "swallowtail-shebang-fixture-{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&directory).expect("fixture directory");
    let script = directory.join("npm-launcher");
    std::fs::write(&script, "#!/usr/bin/env node\nconsole.log('unreachable')\n")
        .expect("fixture script is written");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = std::fs::metadata(&script).expect("metadata").permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(&script, permissions).expect("executable bit");
    }

    let executable = executable_ref();
    let host = LocalProcessHost::builder(LocalProcessLimits::new(3, 1024, 64, 1024, 1024))
        .approve_executable(executable.clone(), script)
        .build();
    assert_failure_code(
        start(
            &host,
            ProcessRequest::new(executable).with_arguments(["--version".to_owned()]),
        ),
        "swallowtail.local_process.interpreted_launcher_requires_host_recipe",
    );
    let _ = std::fs::remove_dir_all(directory);
}

#[test]
fn interpreted_launch_orders_prefix_and_driver_arguments_under_one_limit() {
    let executable = executable_ref();
    let launch =
        LocalExecutableLaunch::new(std::env::current_exe().expect("test executable is available"))
            .with_prefix_arguments([OsString::from("--exact")])
            .with_bootstrap_environment(fixture_environment("version"));
    let formatting = format!("{launch:?}");
    assert!(formatting.contains("prefix_argument_count: 1"));
    assert!(formatting.contains("bootstrap_environment_count: 2"));
    assert!(!formatting.contains("stderr-secret"));
    assert!(!formatting.contains("local_process"));

    let (builder, target) =
        LocalProcessHost::builder(LocalProcessLimits::new(3, 1024, 64, 1024, 1024))
            .approve_installed_executable_launch(
                executable.clone(),
                InterfaceVersionAxis::new("fixture.interpreted.package")
                    .expect("fixture axis is valid"),
                launch,
            );
    let host = builder.build();
    assert_eq!(target.executable(), &executable);
    assert!(!format!("{target:?}").contains("local_process"));
    let process = start(
        &host,
        ProcessRequest::new(target.executable().clone()).with_arguments([
            "support::process_fixture".to_owned(),
            "--nocapture".to_owned(),
        ]),
    )
    .expect("interpreted fixture starts");
    let (stdout, stderr) = collect_output(process.as_ref()).expect("fixture output is bounded");
    assert!(stderr.is_empty());
    assert!(String::from_utf8_lossy(&stdout).contains("fixture-harness 1.2.0"));
    assert!(block_on(process.wait()).expect("fixture joins").success());

    let limited = LocalProcessHost::builder(LocalProcessLimits::new(2, 1024, 64, 64, 64))
        .approve_executable_launch(
            executable.clone(),
            LocalExecutableLaunch::new(
                std::env::current_exe().expect("test executable is available"),
            )
            .with_prefix_arguments([OsString::from("--exact")]),
        )
        .build();
    assert_failure_code(
        start(
            &limited,
            ProcessRequest::new(executable).with_arguments([
                "support::process_fixture".to_owned(),
                "--nocapture".to_owned(),
            ]),
        ),
        "swallowtail.local_process.argument_limit_exceeded",
    );
}

#[test]
fn explicit_request_environment_overrides_bounded_bootstrap_environment() {
    let executable = executable_ref();
    let environment = environment_ref();
    let launch =
        LocalExecutableLaunch::new(std::env::current_exe().expect("test executable is available"))
            .with_prefix_arguments(fixture_arguments().map(OsString::from))
            .with_bootstrap_environment(fixture_environment("overflow"));
    let host = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable_launch(executable.clone(), launch)
        .approve_environment(environment.clone(), fixture_environment("version"))
        .build();
    let process = start(
        &host,
        ProcessRequest::new(executable).with_environment([environment]),
    )
    .expect("fixture starts with composed environment");
    let (stdout, stderr) = collect_output(process.as_ref()).expect("fixture output is bounded");
    assert!(stderr.is_empty());
    assert!(String::from_utf8_lossy(&stdout).contains("fixture-harness 1.2.0"));
    assert!(block_on(process.wait()).expect("fixture joins").success());

    let oversized = (0..33).map(|index| {
        (
            OsString::from(format!("SWALLOWTAIL_BOOTSTRAP_{index}")),
            OsString::from("value"),
        )
    });
    let executable = executable_ref();
    let host = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable_launch(
            executable.clone(),
            LocalExecutableLaunch::new(
                std::env::current_exe().expect("test executable is available"),
            )
            .with_bootstrap_environment(oversized),
        )
        .build();
    assert_failure_code(
        start(&host, ProcessRequest::new(executable)),
        "swallowtail.local_process.bootstrap_environment_limit_exceeded",
    );

    let executable = executable_ref();
    let host = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable_launch(
            executable.clone(),
            LocalExecutableLaunch::new(
                std::env::current_exe().expect("test executable is available"),
            )
            .with_bootstrap_environment([(
                OsString::from("SWALLOWTAIL_BOOTSTRAP_VALUE"),
                OsString::from("x".repeat(16 * 1024 + 1)),
            )]),
        )
        .build();
    assert_failure_code(
        start(&host, ProcessRequest::new(executable)),
        "swallowtail.local_process.bootstrap_environment_limit_exceeded",
    );
}

#[test]
fn node_startup_environment_filter_checks_bootstrap_and_approved_bindings_before_spawn() {
    let executable = executable_ref();
    let environment = environment_ref();
    for name in [
        "NODE_OPTIONS",
        "node_path",
        "LD_PRELOAD",
        "DYLD_INSERT_LIBRARIES",
    ] {
        let launch =
            LocalExecutableLaunch::new(std::env::current_exe().expect("fixture executable"))
                .with_prefix_arguments(fixture_arguments().map(OsString::from))
                .with_node_startup_environment_filter();
        let host = LocalProcessHost::builder(LocalProcessLimits::default())
            .approve_executable_launch(executable.clone(), launch)
            .approve_environment(
                environment.clone(),
                [(OsString::from(name), OsString::from("unsafe"))],
            )
            .build();
        assert_failure_code(
            start(
                &host,
                ProcessRequest::new(executable.clone()).with_environment([environment.clone()]),
            ),
            "swallowtail.local_process.unsafe_node_startup_environment",
        );
    }
    let launch = LocalExecutableLaunch::new(std::env::current_exe().expect("fixture executable"))
        .with_bootstrap_environment([(
            OsString::from("NODE_OPTIONS"),
            OsString::from("--require=unsafe"),
        )])
        .with_node_startup_environment_filter();
    let host = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable_launch(executable.clone(), launch)
        .build();
    assert_failure_code(
        start(&host, ProcessRequest::new(executable.clone())),
        "swallowtail.local_process.unsafe_node_startup_environment",
    );

    let launch = LocalExecutableLaunch::new(std::env::current_exe().expect("fixture executable"))
        .with_prefix_arguments(fixture_arguments().map(OsString::from))
        .with_bootstrap_environment(fixture_environment("version"))
        .with_node_startup_environment_filter();
    let host = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable_launch(executable.clone(), launch)
        .build();
    let process = start(&host, ProcessRequest::new(executable)).expect("safe fixture starts");
    let (stdout, _) = collect_output(process.as_ref()).expect("bounded output");
    assert!(String::from_utf8_lossy(&stdout).contains("fixture-harness 1.2.0"));
    assert!(block_on(process.wait()).expect("fixture joins").success());
}

#[cfg(unix)]
#[test]
fn checked_source_capture_freezes_verified_bytes_and_rejects_aliases() {
    use sha2::{Digest, Sha256};
    use std::path::Path;
    use std::process::Command;
    use swallowtail_host_local::{
        LocalCapturedGraph, LocalCapturedImport, LocalCapturedLaunchFingerprint,
        LocalCapturedNodeRecipe, LocalCapturedSources, LocalExpectedSource,
    };
    let template = std::env::temp_dir().join("checked-sources.XXXXXXXX");
    let output = Command::new("/usr/bin/mktemp")
        .arg("-d")
        .arg(template)
        .env_clear()
        .output()
        .expect("mktemp starts");
    assert!(output.status.success());
    let root = std::path::PathBuf::from(String::from_utf8(output.stdout).expect("path").trim())
        .canonicalize()
        .expect("canonical fixture root");
    std::fs::write(root.join("one.mjs"), b"one").expect("fixture");
    std::fs::write(root.join("two.mjs"), b"two").expect("fixture");
    let one = LocalExpectedSource::new("one.mjs", Sha256::digest(b"one").into());
    let two = LocalExpectedSource::new("two.mjs", Sha256::digest(b"two").into());
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(root.join("one.mjs"), std::fs::Permissions::from_mode(0o666))
        .expect("writable source fixture");
    assert!(LocalCapturedSources::capture(&root, std::slice::from_ref(&one)).is_err());
    std::fs::set_permissions(root.join("one.mjs"), std::fs::Permissions::from_mode(0o600))
        .expect("restore source ownership boundary");
    let captured =
        LocalCapturedSources::capture(&root, &[one.clone(), two.clone()]).expect("capture");
    let reversed = LocalCapturedSources::capture(&root, &[two.clone(), one.clone()])
        .expect("order independent capture");
    assert_eq!(captured.fingerprint(), reversed.fingerprint());
    assert_eq!(
        captured.source(Path::new("one.mjs")),
        Some(b"one".as_slice())
    );
    assert!(!format!("{captured:?}").contains("one.mjs"));
    let edge = LocalCapturedImport::new("one.mjs", "./two.mjs", "two.mjs");
    let graph = LocalCapturedGraph::new(
        &captured,
        Path::new("one.mjs"),
        Path::new("two.mjs"),
        std::slice::from_ref(&edge),
    )
    .expect("graph");
    let graph_again = LocalCapturedGraph::new(
        &reversed,
        Path::new("one.mjs"),
        Path::new("two.mjs"),
        std::slice::from_ref(&edge),
    )
    .expect("same graph");
    assert_eq!(graph.fingerprint(), graph_again.fingerprint());
    let config: serde_json::Value = serde_json::from_slice(graph.configuration()).expect("config");
    assert_eq!(config["files"][0]["source"], "one");
    assert!(
        LocalCapturedGraph::new(&captured, Path::new("one.mjs"), Path::new("two.mjs"), &[])
            .is_err()
    );
    assert!(
        LocalCapturedGraph::new(
            &captured,
            Path::new("one.mjs"),
            Path::new("two.mjs"),
            &[edge.clone(), edge]
        )
        .is_err()
    );
    assert!(
        LocalCapturedGraph::new(
            &captured,
            Path::new("one.mjs"),
            Path::new("two.mjs"),
            &[LocalCapturedImport::new(
                "one.mjs",
                "./missing",
                "missing.mjs"
            )]
        )
        .is_err()
    );
    let fingerprint =
        LocalCapturedLaunchFingerprint::new(&graph, [1; 32], [2; 32], [3; 32], [4; 32], [5; 32])
            .expect("candidate");
    for changed in 0..5 {
        let mut axes = [[1; 32], [2; 32], [3; 32], [4; 32], [5; 32]];
        axes[changed][0] ^= 0x80;
        let other = LocalCapturedLaunchFingerprint::new(
            &graph, axes[0], axes[1], axes[2], axes[3], axes[4],
        )
        .expect("candidate");
        assert_ne!(fingerprint.digest(), other.digest());
    }
    assert!(
        LocalCapturedLaunchFingerprint::new(&graph, [0; 32], [2; 32], [3; 32], [4; 32], [5; 32])
            .is_err()
    );
    assert!(!format!("{graph:?} {fingerprint:?}").contains("one.mjs"));
    let recipe = LocalCapturedNodeRecipe::new(
        &graph,
        "export function resolve() {}",
        [1; 32],
        [3; 32],
        [5; 32],
    )
    .expect("candidate recipe");
    let altered = LocalCapturedNodeRecipe::new(
        &graph,
        "export function load() {}",
        [1; 32],
        [3; 32],
        [5; 32],
    )
    .expect("altered candidate");
    assert_ne!(
        recipe.fingerprint().digest(),
        altered.fingerprint().digest()
    );
    assert_eq!(recipe.prefix_arguments().len(), 5);
    assert!(!format!("{recipe:?}").contains("one.mjs"));
    assert!(LocalCapturedNodeRecipe::new(&graph, "", [1; 32], [3; 32], [5; 32]).is_err());
    assert!(
        LocalCapturedNodeRecipe::new(&graph, &"x".repeat(128 * 1024), [1; 32], [3; 32], [5; 32])
            .is_err()
    );
    std::fs::write(root.join("one.mjs"), b"changed").expect("replace fixture bytes");
    assert_eq!(
        captured.source(Path::new("one.mjs")),
        Some(b"one".as_slice())
    );
    let after_replacement: serde_json::Value =
        serde_json::from_slice(graph.configuration()).expect("captured config");
    assert_eq!(after_replacement["files"][0]["source"], "one");
    assert!(LocalCapturedSources::capture(&root, std::slice::from_ref(&one)).is_err());
    assert!(LocalCapturedSources::capture(&root, &[two.clone(), two.clone()]).is_err());
    assert!(
        LocalCapturedSources::capture(
            &root,
            &[LocalExpectedSource::new(
                "../two.mjs",
                Sha256::digest(b"two").into()
            )]
        )
        .is_err()
    );
    std::os::unix::fs::symlink("two.mjs", root.join("link.mjs")).expect("symlink fixture");
    assert!(
        LocalCapturedSources::capture(
            &root,
            &[LocalExpectedSource::new(
                "link.mjs",
                Sha256::digest(b"two").into()
            )]
        )
        .is_err()
    );
    std::fs::hard_link(root.join("two.mjs"), root.join("alias.mjs")).expect("hardlink fixture");
    assert!(
        LocalCapturedSources::capture(
            &root,
            &[
                two,
                LocalExpectedSource::new("alias.mjs", Sha256::digest(b"two").into())
            ]
        )
        .is_err()
    );
}

#[test]
fn checked_launch_without_host_custody_refuses_before_ordinary_spawn() {
    use swallowtail_runtime::{CheckedProcessRef, ProcessService, ScopeId};
    let executable = executable_ref();
    let host = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable(
            executable.clone(),
            std::env::current_exe().expect("fixture executable"),
        )
        .build();
    let scope = ScopeId::new("checked-fixture").expect("scope");
    let binding = CheckedProcessRef::new("unissued-private-binding").expect("binding");
    let request = ProcessRequest::new(executable).with_arguments(fixture_arguments());
    assert_failure_code(
        block_on(host.validate_checked(scope.clone(), binding.clone(), request.clone())),
        "swallowtail.local_process.checked_binding_refused",
    );
    assert_failure_code(
        block_on(host.start_checked(scope, binding, request)),
        "swallowtail.local_process.checked_binding_refused",
    );
}

#[test]
fn bounded_stdio_round_trip_keeps_payloads_out_of_formatting() {
    let resource_directory = temporary_resource();
    std::fs::write(resource_directory.join("fixture-marker"), b"marker")
        .expect("fixture marker is written");
    let (host, executable, environment, resource) = fixture_host(
        "echo",
        LocalProcessLimits::new(8, 1024, 1024, 1024, 1024),
        &resource_directory,
    );
    let process = start(&host, request(&executable, &environment, &resource))
        .expect("approved process starts");
    let input = ProcessInputChunk::new(b"input-secret".to_vec());
    assert!(!format!("{input:?}").contains("input-secret"));
    block_on(process.write_stdin(input)).expect("bounded stdin writes");
    block_on(process.close_stdin()).expect("stdin closes");

    let (stdout, stderr) = collect_output(process.as_ref()).expect("output is bounded");
    let exit = block_on(process.wait()).expect("process cleanup joins");
    assert!(exit.success());
    assert!(String::from_utf8_lossy(&stdout).contains("fixture-stdout:input-secret"));
    assert!(String::from_utf8_lossy(&stderr).contains("stderr-secret"));
    assert!(
        !format!(
            "{:?}",
            swallowtail_runtime::ProcessOutputChunk::new(ProcessOutputStream::Stderr, stderr,)
        )
        .contains("stderr-secret")
    );

    std::fs::remove_dir_all(resource_directory).expect("fixture resource is removed");
}

#[test]
fn input_and_output_limits_fail_with_safe_dimensions() {
    let resource_directory = temporary_resource();
    let (input_host, executable, environment, resource) = fixture_host(
        "wait-for-eof",
        LocalProcessLimits::new(8, 1024, 4, 1024, 1024),
        &resource_directory,
    );
    let input_process = start(&input_host, request(&executable, &environment, &resource))
        .expect("input-limit fixture starts");
    assert_failure_code(
        block_on(input_process.write_stdin(ProcessInputChunk::new(b"too-long".to_vec()))),
        "swallowtail.local_process.stdin_limit_exceeded",
    );
    block_on(input_process.request_stop()).expect("eof stop is requested");
    assert!(
        block_on(input_process.wait())
            .expect("input fixture joins")
            .success()
    );

    let (output_host, executable, environment, resource) = fixture_host(
        "overflow",
        LocalProcessLimits::new(8, 1024, 64, 16, 1024),
        &resource_directory,
    );
    let output_process = start(&output_host, request(&executable, &environment, &resource))
        .expect("output-limit fixture starts");
    let output_failure = loop {
        match block_on(output_process.read_output()) {
            Ok(Some(_)) => {}
            Ok(None) => panic!("output overflow must remain visible"),
            Err(failure) => break failure,
        }
    };
    assert_eq!(
        output_failure.diagnostic().code(),
        "swallowtail.local_process.output_limit_exceeded"
    );
    assert!(
        block_on(output_process.wait())
            .expect("overflow fixture joins")
            .success()
    );

    std::fs::remove_dir_all(resource_directory).expect("fixture resource is removed");
}

#[test]
fn cancellation_and_deadline_paths_wait_for_child_cleanup() {
    let resource_directory = temporary_resource();
    let limits = LocalProcessLimits::new(8, 1024, 64, 1024, 1024);
    let (cancel_host, executable, environment, resource) =
        fixture_host("wait-for-eof", limits, &resource_directory);
    let cancelled = start(&cancel_host, request(&executable, &environment, &resource))
        .expect("cancellation fixture starts");
    block_on(cancelled.request_stop()).expect("graceful cancellation closes stdin");
    assert!(
        block_on(cancelled.wait())
            .expect("cancelled child joins")
            .success()
    );

    let (deadline_host, executable, environment, resource) =
        fixture_host("sleep", limits, &resource_directory);
    let timed_out = start(
        &deadline_host,
        request(&executable, &environment, &resource),
    )
    .expect("deadline fixture starts");
    block_on(timed_out.force_stop()).expect("expired deadline requests force stop");
    let exit = block_on(timed_out.wait()).expect("timed-out child joins");
    assert!(!exit.success());

    std::fs::remove_dir_all(resource_directory).expect("fixture resource is removed");
}

#[test]
fn operation_scoped_resources_feed_processes_then_release_after_exit() {
    let materialized = temporary_resource();
    let executable = executable_ref();
    let environment = environment_ref();
    let scope = process_scope();
    let host = LocalProcessHost::builder(LocalProcessLimits::default())
        .with_temporary_root(&materialized)
        .approve_executable(
            executable.clone(),
            std::env::current_exe().expect("test executable"),
        )
        .approve_environment(environment.clone(), fixture_environment("working-resource"))
        .build();
    let lease = block_on(host.create_temporary(
        scope.clone(),
        ResourceAccess::ReadWrite,
        ResourceRepresentation::TemporaryFile,
    ))
    .expect("operation-scoped resource is created");
    let request = ProcessRequest::new(executable)
        .with_arguments(fixture_arguments())
        .with_environment([environment])
        .with_working_resource(lease.reference().clone());

    let process = start_in_scope(&host, scope, request)
        .expect("same-scope process receives the temporary resource");
    assert!(block_on(process.wait()).expect("child joins").success());
    assert_eq!(
        std::fs::read_dir(&materialized)
            .expect("materialization root is readable")
            .count(),
        1
    );
    assert_eq!(block_on(host.release(lease)), CleanupOutcome::Clean);
    assert_eq!(
        std::fs::read_dir(&materialized)
            .expect("materialization root is readable")
            .count(),
        0
    );

    std::fs::remove_dir_all(materialized).expect("fixture root is removed");
}
