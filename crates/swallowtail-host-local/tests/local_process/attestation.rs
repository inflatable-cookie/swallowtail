//! Adversarial counterexamples for owned-tree attestation (card 059).
//!
//! Card 057 landed the provider-neutral distinction between a root exit and an
//! attested-empty owned tree, and left the local host reporting
//! [`ProcessTreeCompletion::RootOnly`] on every platform. Card 059 asked
//! whether the operator-authorized `unsafe`/dependency boundary lets a
//! supported Unix host construct the positive state soundly.
//!
//! These tests are the falsification half of that question. Each one falsifies
//! one candidate primitive natively on the host that runs it, by driving one of
//! the review oracle's counterexample classes through it:
//!
//! - a `setsid` descendant escapes any observation scoped to the launcher's
//!   owned process group;
//! - an inherited liveness descriptor reaches end-of-file while a descendant is
//!   still alive, because the descendant may close or not inherit it;
//! - a released process-group number stops existing and is free for the kernel
//!   to reuse, so probing it after the owner is reaped observes nothing sound;
//! - an orphaned descendant is reparented to `launchd` after its intermediate
//!   parent exits, so an ancestry walk from the launcher loses it.
//!
//! The tests falsify primitive candidates; they do not exercise an integrated
//! host implementation. Together they establish that a bare process group, an
//! inherited descriptor's end-of-file, and an ancestry walk are each
//! insufficient, so no sound owned-tree observation was found and validated
//! within the current ordinary host-local authority — `forbid(unsafe_code)`, no
//! privileged capability, and no system extension. A sound observation would
//! require a kernel-enforced owned-tree container with exclusive host ownership
//! and denied migration; evaluating one is outside this bounded lane. The host
//! stays root-only rather than publishing a best-effort claim.

#![cfg(unix)]

use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use nix::errno::Errno;
use nix::sys::signal::{Signal, kill, killpg};
use nix::unistd::Pid;

/// SIGKILL a witness pid, ignoring an already-exited process.
fn reap_witness(pid: u32) {
    if let Ok(pid) = i32::try_from(pid) {
        let _ = kill(Pid::from_raw(pid), Signal::SIGKILL);
    }
}

/// Reads a numeric field from `ps` for a live pid, e.g. `pgid` or `ppid`.
fn read_ps_field(pid: u32, field: &str) -> Option<i32> {
    let output = Command::new("/bin/ps")
        .args(["-o", &format!("{field}="), "-p", &pid.to_string()])
        .output()
        .expect("ps runs");
    String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<i32>()
        .ok()
}

/// Reads the current process-group id for a live pid through `ps`.
fn read_process_group(pid: u32) -> Option<i32> {
    read_ps_field(pid, "pgid")
}

/// Waits until a freshly spawned interpreter has reached its `setsid` call and
/// become its own process-group leader, so the sampled group id is stable.
///
/// A freshly spawned child inherits the launcher's process group until it calls
/// `setsid`; polling for the leader state avoids sampling that transient
/// inherited group. Returns the last group id observed before the deadline.
fn poll_until_group_leader(pid: u32) -> Option<i32> {
    let deadline = Instant::now() + Duration::from_secs(2);
    let mut last = None;
    loop {
        if let Some(group) = read_process_group(pid) {
            last = Some(group);
            if group == i32::try_from(pid).expect("witness pid fits the host pid type") {
                return last;
            }
        }
        if Instant::now() >= deadline {
            return last;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

/// Counterexample class one: session escape defeats process-group enumeration.
///
/// A candidate positive mechanism enumerates the host-owned process group and
/// calls the tree empty when no non-owner member remains. A descendant that
/// calls `setsid()` starts a new session and becomes its own process-group
/// leader, so its process-group id equals its own pid and no longer matches the
/// launcher's owned group. Enumeration scoped to that owned group therefore
/// reports empty while the descendant is still alive.
#[test]
fn a_setsid_descendant_escapes_owned_process_group_enumeration() {
    let mut witness = Command::new("/usr/bin/perl")
        .args(["-e", "use POSIX qw(setsid); setsid() or die; sleep 30;"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("setsid witness starts");
    let pid = witness.id();

    let group = poll_until_group_leader(pid).expect("witness exposes a process group");
    // The descendant became its own group leader: its process-group id equals
    // its pid. Any enumeration scoped to the launcher's distinct owned group
    // cannot see it, yet it is alive.
    assert_eq!(
        group,
        i32::try_from(pid).expect("witness pid fits the host pid type"),
        "a setsid descendant leads its own process group and leaves the owned group",
    );

    reap_witness(pid);
    let _ = witness.wait();
}

/// Counterexample class two: an inherited liveness descriptor reaches EOF while
/// a descendant is alive.
///
/// A candidate positive mechanism installs one extra pipe write end that every
/// enrolled process inherits, and treats end-of-file on the read end as proof
/// the tree is gone. This witness forks a child that inherits the write end,
/// closes it, and `exec`s a long sleep. The parent then observes EOF on the
/// read end although the child is alive, so descriptor EOF is not tree
/// emptiness. The host cannot prove a foreign provider descendant never closes
/// or fails to inherit the descriptor, so the mechanism is unsound on its own.
#[test]
fn an_inherited_liveness_descriptor_reaches_eof_while_a_descendant_is_alive() {
    let program = r#"
        $| = 1;
        pipe(my $read, my $write) or die "pipe";
        my $pid = fork();
        die "fork" unless defined $pid;
        if ($pid == 0) {
            close $read;
            close $write;            # drop the inherited liveness descriptor
            exec('/bin/sleep', '30') or die "exec";
        }
        close $write;                # parent releases its own write end
        my $buf = '';
        my $n = sysread($read, $buf, 1);   # 0 == EOF: no process holds the write end
        my $alive = kill(0, $pid) ? 1 : 0; # 1 == the descendant is still alive
        print "eof=$n alive=$alive pid=$pid\n";
    "#;
    let output = Command::new("/usr/bin/perl")
        .args(["-e", program])
        .stderr(Stdio::null())
        .output()
        .expect("liveness-descriptor witness runs");
    let text = String::from_utf8_lossy(&output.stdout);
    let mut eof: Option<i64> = None;
    let mut alive: Option<i64> = None;
    let mut child_pid: Option<u32> = None;
    for field in text.split_whitespace() {
        if let Some(value) = field.strip_prefix("eof=") {
            eof = value.parse().ok();
        } else if let Some(value) = field.strip_prefix("alive=") {
            alive = value.parse().ok();
        } else if let Some(value) = field.strip_prefix("pid=") {
            child_pid = value.parse().ok();
        }
    }
    if let Some(pid) = child_pid {
        reap_witness(pid);
    }

    assert_eq!(
        eof,
        Some(0),
        "the read end reached end-of-file: no process holds the write end",
    );
    assert_eq!(
        alive,
        Some(1),
        "the descendant is alive even though the liveness descriptor is at EOF",
    );
}

/// Counterexample class three: a released group number is not a stable identity.
///
/// The only way to observe the owned group "empty" by probing its number is to
/// first reap the owner that anchors it. This witness becomes its own group
/// leader through `setsid`, then exits. After it is reaped, the group number no
/// longer exists, so an existence probe returns `ESRCH`. The kernel is then
/// free to reassign that number to an unrelated process, so a probe of a
/// released group number can never be a sound emptiness observation. The local
/// host already refuses to signal a bare group without a live owner for exactly
/// this reason.
#[test]
fn a_released_owned_group_number_stops_existing_and_frees_its_identity() {
    let mut leader = Command::new("/usr/bin/perl")
        .args(["-e", "use POSIX qw(setsid); setsid() or die; exit 0;"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("group-leader witness starts");
    let group = i32::try_from(leader.id()).expect("witness pid fits the host pid type");
    leader
        .wait()
        .expect("witness is reaped, leaving no zombie group member");

    // Poll briefly: the group ceases to exist once its sole member is gone.
    let deadline = Instant::now() + Duration::from_secs(2);
    let result: Result<(), Errno> = loop {
        match killpg(Pid::from_raw(group), None) {
            Err(error) => break Err(error),
            Ok(()) => {
                assert!(
                    Instant::now() < deadline,
                    "the released group number must stop existing",
                );
                std::thread::sleep(Duration::from_millis(20));
            }
        }
    };
    assert_eq!(
        result,
        Err(Errno::ESRCH),
        "a released process-group number no longer identifies any owned member",
    );
}

/// Counterexample class four: reparenting defeats an ancestry walk.
///
/// A candidate positive mechanism walks descendants by parent pid from the
/// launcher's root. This witness forks a grandchild that outlives it, then the
/// intermediate parent exits. On macOS the orphaned grandchild is reparented to
/// `launchd` (pid 1), because macOS has no child-subreaper
/// (`PR_SET_CHILD_SUBREAPER` is Linux-only), so an ancestry walk rooted at the
/// launcher can no longer reach the grandchild while it is alive.
#[test]
fn a_reparented_descendant_is_orphaned_and_lost_by_an_ancestry_walk() {
    // The grandchild pid travels through a file, not stdout, so the orphaned
    // grandchild never holds a captured pipe and cannot delay this test.
    let pid_file = std::env::temp_dir().join(format!(
        "swallowtail-reparent-{}-{:?}.pid",
        std::process::id(),
        Instant::now(),
    ));
    let program = r#"
        my $pidfile = $ARGV[0];
        my $pid = fork();
        die "fork" unless defined $pid;
        if ($pid == 0) {
            exec('/bin/sleep', '30') or die "exec";
        }
        open(my $fh, '>', $pidfile) or die "open";
        print $fh "$pid\n";
        close $fh;
        exit 0;               # orphan the grandchild to launchd
    "#;
    // `status` waits for and reaps the intermediate parent, orphaning the
    // grandchild. Null stdio keeps the grandchild off any inherited pipe.
    let status = Command::new("/usr/bin/perl")
        .args(["-e", program, &pid_file.to_string_lossy()])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("reparenting witness runs");
    assert!(status.success(), "reparenting witness exits cleanly");
    let grandchild = std::fs::read_to_string(&pid_file)
        .expect("witness records a grandchild pid")
        .trim()
        .parse::<u32>()
        .expect("witness pid is numeric");
    let _ = std::fs::remove_file(&pid_file);

    // The grandchild reparents to pid 1 once its parent is gone.
    let deadline = Instant::now() + Duration::from_secs(3);
    let mut parent = read_ps_field(grandchild, "ppid");
    while parent != Some(1) && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(20));
        parent = read_ps_field(grandchild, "ppid");
    }
    reap_witness(grandchild);

    assert_eq!(
        parent,
        Some(1),
        "an orphaned descendant reparents to launchd and leaves the launcher's ancestry",
    );
}
