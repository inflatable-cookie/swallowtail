#[path = "local_process/support.rs"]
mod support;

use support::*;
use swallowtail_core::InterfaceVersionAxis;
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
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
