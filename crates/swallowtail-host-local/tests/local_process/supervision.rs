use super::*;

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
