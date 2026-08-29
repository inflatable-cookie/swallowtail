# 259 Process Containment Backend Evidence

Status: promoted
Owner: Tom
Created: 2026-08-29
Updated: 2026-08-29
Question source: PR 117 review / g05.003 card 009 stop condition

## Question

Can the default local process boundary enforce Contract 059's rule that no
watcher-owned descendant outlives its turn, including a child that leaves the
launch process group?

## Decision

No portable or default-macOS guarantee is available from ordinary process
groups, `launchd`, process-table polling, or public `kqueue` process events.

Contract 059 keeps the hard no-outliving invariant. Process-backed watcher
support is capability-gated behind an exact host containment backend whose
owned lease contains descendants by construction and proves the containment
scope empty before joined cleanup. A process group, root-process handle, or
polling observer is not that capability. Hosts without a qualified backend do
not advertise process-backed watcher support and reject any such start before
work.

This is an operator decision recorded on 2026-08-29. It does not select a
macOS containment implementation or qualify Windows or Linux support.

## Evidence

### macOS

Apple's current XNU public header exposes process exit, fork, and exec events,
but explicitly says recursive `NOTE_TRACK`, `NOTE_TRACKERR`, and `NOTE_CHILD`
tracking has been unsupported since macOS 10.5. `NOTE_FORK` observation alone
does not provide an unforgeable descendant handle or close fork, exit, and PID
reuse races.

Apple's `launchd.plist(5)` contract says that when a job dies, `launchd` kills
remaining processes with the same process-group ID. The same document tells
managed jobs not to call `setsid` or daemonize. That is a cooperative job rule,
not containment of a child that deliberately or accidentally leaves the
group.

PR 117's closed-pipe fixture supplies local proof of the consequence: a child
can call `setsid`, close inherited output pipes, and remain alive after the
owned group exits. Polling can detect some observed escapes and fail closed,
but it can miss a fork/reparent race and cannot safely signal a later numeric
PID without foreign-process risk.

### Other platform shapes

Windows Job Objects can keep child processes in a non-breakaway job and
terminate the associated hierarchy. Linux cgroup v2 exposes `cgroup.kill`,
whose kernel documentation says it kills the cgroup tree and handles
concurrent forks and migrations. Both still require an exact host backend,
authority, lifecycle implementation, and conformance proof. Their existence
does not make containment portable or available on macOS.

## Contract Consequence

- watcher coordination and process containment are separate capabilities;
- registration of a watcher registry does not imply a process executor;
- an accepted process-backed watcher must bind a containment lease before its
  public watcher identity returns;
- stop, cancellation, deadline, failure, and close target the lease, never a
  caller-supplied PID;
- joined truth requires the containment scope to be empty and its supervisor
  work joined;
- task-backed work qualifies only when the host proves it cannot create
  unmanaged child work or binds any such work to the same containment lease;
- default `swallowtail-host-local` process groups remain useful cleanup
  mechanics but do not satisfy Contract 059 containment;
- Claude bridge and production watcher claims remain gated until an exact
  containment-capable host composition is supplied and proved.

## Frozen Sources

Retrieved 2026-08-29.

| Source | Binding evidence | SHA-256 |
| --- | --- | --- |
| [Apple XNU `bsd/sys/event.h`](https://github.com/apple-oss-distributions/xnu/blob/main/bsd/sys/event.h) | recursive `NOTE_TRACK` family unsupported since 10.5 | `a172b971272e10ced09ea58b2d74aea333901b8a7ed27864a261df79adc4714c` |
| [Apple `launchd.plist(5)`](https://github.com/apple-oss-distributions/launchd/blob/main/man/launchd.plist.5) | same-process-group cleanup; jobs should not `setsid` or daemonize | `9d524845e3d92d8c31ffacb65ed565025e4c26a7636ec161fc5683b19dd282c7` |
| [Microsoft Job Objects](https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects) | inherited job membership, optional breakaway, hierarchy termination | `1e8543ee83aa5778ada6c3b40b9e140b516875e00ac9641f7be047face5dd9e8` |
| [Linux cgroup v2](https://www.kernel.org/doc/html/latest/admin-guide/cgroup-v2.html) | `cgroup.kill` covers descendants, concurrent forks, and migrations | `2d716cb798eacf8fbcf431b52697fbb8c5ed8ea9f58ac15b2b55351247c0bf34` |

## Non-Claims

- No default macOS containment backend is identified.
- No Windows Job Object or Linux cgroup implementation is accepted.
- No container, VM, privileged helper, Endpoint Security entitlement, or
  private Darwin API becomes a Swallowtail prerequisite.
- Detection of an escaped process is not ownership or cleanup.
- Contract 059 is not weakened to best-effort orphan reporting.

## Promotion

Promoted into Contract 059, Contract 010, the architecture guardrails,
g05.003, and revised card 009. PR 117 must be repaired and restacked against
that authority before merge. Card 010 remains gated.
