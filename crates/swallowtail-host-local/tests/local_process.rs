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

#[test]
fn pipe_inheriting_descendant_does_not_stall_wait_or_output_drain() {
    let resource_directory = temporary_resource();
    let (host, executable, environment, resource) = fixture_host(
        "spawn-descendant",
        LocalProcessLimits::default(),
        &resource_directory,
    );
    let process = start(&host, request(&executable, &environment, &resource))
        .expect("descendant fixture starts");
    let (stdout, stderr) = collect_output(&*process).expect("output drains to terminal");
    let exit = block_on(process.wait()).expect("descendant fixture joins");
    assert!(exit.success());
    assert!(
        stdout
            .windows(b"descendant-spawned".len())
            .any(|window| window == b"descendant-spawned")
    );
    assert!(stderr.is_empty());
    std::fs::remove_dir_all(resource_directory).expect("fixture resource is removed");
}

#[test]
fn descendant_holding_output_pipes_past_the_join_bound_is_bounded() {
    let resource_directory = temporary_resource();
    let (host, executable, environment, resource) = fixture_host(
        "spawn-long-descendant",
        LocalProcessLimits::default(),
        &resource_directory,
    );
    let process = start(&host, request(&executable, &environment, &resource))
        .expect("long descendant fixture starts");
    let started = std::time::Instant::now();
    let (stdout, stderr) = collect_output(&*process).expect("abandoned output drains to terminal");
    let exit = block_on(process.wait()).expect("long descendant fixture joins");
    assert!(exit.success());
    assert!(started.elapsed() < std::time::Duration::from_secs(8));
    assert!(
        stdout
            .windows(b"long-descendant-spawned".len())
            .any(|window| window == b"long-descendant-spawned")
    );
    assert!(stderr.is_empty());
    let after_terminal = block_on(process.read_output()).expect("terminal read stays terminal");
    assert!(after_terminal.is_none());
    std::fs::remove_dir_all(resource_directory).expect("fixture resource is removed");
}

#[test]
fn force_stop_racing_natural_exit_never_misreports_clean_exit() {
    let resource_directory = temporary_resource();
    for _ in 0..12 {
        let (host, executable, environment, resource) = fixture_host(
            "exit-zero",
            LocalProcessLimits::default(),
            &resource_directory,
        );
        let process = start(&host, request(&executable, &environment, &resource))
            .expect("fast-exit fixture starts");
        // The fixture exits on its own immediately. The stop either lands
        // while it still runs (killed exit), races the natural exit (clean
        // exit), or finds the supervision already finished (control closed).
        // Every ordering is a real outcome; a clean exit must never be
        // reported as force_stop_failed.
        let _ = block_on(process.force_stop());
        let _exit = block_on(process.wait()).unwrap_or_else(|failure| {
            panic!(
                "force stop must never misreport a racing clean exit, got {}",
                failure.diagnostic().code()
            )
        });
        let _ = block_on(process.read_output());
        drop(process);
    }
    std::fs::remove_dir_all(resource_directory).expect("fixture resource is removed");
}
