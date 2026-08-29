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

#[cfg(unix)]
#[test]
fn escaped_descendant_can_outlive_ordinary_process_group_cleanup() {
    use nix::sys::signal::{Signal, kill};
    use nix::unistd::Pid;

    let resource_directory = temporary_resource();
    let (host, executable, environment, resource) = fixture_host(
        "spawn-escaped-descendant-closed-pipes",
        LocalProcessLimits::default(),
        &resource_directory,
    );
    let process = start(&host, request(&executable, &environment, &resource))
        .expect("pipe-closing escaped descendant fixture starts");
    let (stdout, stderr) = collect_output(&*process).expect("closed pipes drain to terminal");
    assert!(
        stdout
            .windows(b"escaped-descendant-closed-pipes-spawned".len())
            .any(|window| window == b"escaped-descendant-closed-pipes-spawned")
    );
    assert!(stderr.is_empty());

    let pid = std::fs::read_to_string(resource_directory.join("escaped-descendant.pid"))
        .expect("fixture records the escaped descendant pid")
        .trim()
        .parse::<u32>()
        .expect("fixture pid is numeric");
    let _cleanup = EscapedProcessCleanup(Some(pid));
    let _exit = block_on(process.wait()).expect(
        "ordinary process-group cleanup may report success without containing a setsid child",
    );
    assert!(
        process_is_alive(pid),
        "escaped descendant remains independently live after group cleanup"
    );

    let _ = kill(
        Pid::from_raw(i32::try_from(pid).expect("fixture pid fits the host pid type")),
        Signal::SIGKILL,
    );
    std::fs::remove_dir_all(resource_directory).expect("fixture resource is removed");
}

#[cfg(unix)]
struct EscapedProcessCleanup(Option<u32>);

#[cfg(unix)]
impl Drop for EscapedProcessCleanup {
    fn drop(&mut self) {
        if let Some(pid) = self.0.take()
            && let Ok(pid) = i32::try_from(pid)
        {
            let _ = nix::sys::signal::kill(
                nix::unistd::Pid::from_raw(pid),
                nix::sys::signal::Signal::SIGKILL,
            );
        }
    }
}

#[cfg(unix)]
fn process_is_alive(pid: u32) -> bool {
    std::process::Command::new("/bin/ps")
        .args(["-p", &pid.to_string(), "-o", "pid="])
        .output()
        .map(|output| output.status.success() && !output.stdout.iter().all(u8::is_ascii_whitespace))
        .unwrap_or(false)
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
