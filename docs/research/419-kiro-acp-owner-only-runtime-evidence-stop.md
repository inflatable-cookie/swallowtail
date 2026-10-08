# Research 419: Kiro ACP Owner-Only Runtime Evidence Stop

Status: finite runtime evidence stop; no compatibility claim changed.
Date: 2026-10-08.
Route: `kiro.acp` on `kiro-cli.release`.
Task: swallowtail#150.
Authority: decisions `73ff6b53-f8be-4d7f-9031-c8b0a81e3ff1` and
`8079018e-2ec0-45fd-aaf3-0351c1e3ff1f`; Contracts 023 and 029; Research 394.

## Outcome

The approved offline ARM64 GNU environment passed its fake-first containment
and trace checks. Exact `2.26.1` started and touched synthetic Kiro settings,
then exited before an ACP `initialize` response. The stderr classifier found
an authentication-related startup indicator. The runner did not send
`session/new`, and no `session/load` probe ran. It stopped before executing
`2.27.0`, `2.27.1`, or `2.28.0`.

No owner-sweep result follows. The selected session tree was not observed in
the syscall paths, and the run did not establish whether the V2 ACP startup
initializer or owner-sweep worker is reachable after authentication. The
existing exact `2.21.4` claim, exclusions, and HTTP MCP live gate remain
unchanged.

## Exact artifact identities

All four frozen Linux AArch64 GNU archives from Research 394 were downloaded,
matched by archive digest and size, extracted in the task container, and
checked against the frozen `BUILD-INFO`, `kiro-cli`, and `kiro-cli-chat`
identities. Only `2.26.1` was executed.

| Version | Archive SHA-256 | Bytes | Build hash |
| --- | --- | ---: | --- |
| `2.26.1` | `91da83c971bab9f8376bbaf73fb37993bca48a5ce68d34ed30ed971dac52de49` | 130634712 | `15d7f349b2760a900ebf8bec43e05d1bfbef6165` |
| `2.27.0` | `e983d1ba524dcfb0265b405324e5e203e731bc0cf15c355781bfebd66191311c` | 130827468 | `7c1a246f80bfdf3266858f4391fcdf75dd9faf62` |
| `2.27.1` | `f7fddc8c6f3d19f3a24c1077ae74f253f9898d323d2d6e768e12e779d039f459` | 130936664 | `7c1a246f80bfdf3266858f4391fcdf75dd9faf62` |
| `2.28.0` | `2b9b26915737d3f8086bc28b9830b9ebe3d4452a26c7c092d9a190d6355937be` | 131008436 | `877b466eda188c6dc44136df230f7e193a4edbfa` |

The selected `BUILD-INFO`, wrapper, and chat binary digests match the exact
Research 394 inventory at each point. The complete archive identities are in
the local evidence summary; no artifact contents were copied into this
record.

## Containment and fake-first proof

The host was Darwin ARM64. Fresh Colima profile `swallowtail-kiro-owner-proof`
used VZ with an AArch64 Ubuntu 24.04.4 GNU guest (`glibc 2.39`). The guest and
Docker backend reported `aarch64` and `linux/aarch64`. Rosetta and Colima
foreign-architecture binfmt were disabled. The only registered handler was
the guest's native CPython 3.12 bytecode handler; its magic matched the native
AArch64 interpreter. No QEMU handler or host filesystem mount was present.

The evidence container was
`swallowtail-kiro-owner-proof-trace-exec`, container ID
`054af7bb950622a399dcd12d405b48f56d2992425ec3b8d98ac89db775df0451`, image
`sha256:8a2b18716e25ab4ced89fef612ae48d79a5e1cdf27a936c7e386820c771f05cb`.
It had network mode `none`, a private PID namespace, a read-only rootfs, no
binds, runtime mounts, devices, or device requests, and only `/scratch` as a
writable tmpfs (`rw,exec,nosuid,nodev`, 8 GiB). The container dropped all
capabilities, then added only `SYS_PTRACE`, `SETUID`, and `SETGID` for the
root-owned tracer to follow its two disposable UID targets. The tracer ran
with no-new-privileges; each Kiro target ran as UID 21001 or 21002. CPU, memory,
and PID limits were 4 CPUs, 5 GiB, and 128 processes. The container and Colima
profile remain running for review.

The fake-first gate passed before vendor archive downloads or execution. It
observed native ARM64 GNU, loopback only, no route, a denied connection probe
(`OSError:101`), denied rootfs write (`EACCES`), an unchanged image sentinel,
and complete cleanup of its bounded child. Fake `strace` observed the
sentinel `openat`; the fake trace stderr was empty. No host home, repository,
credential, or device mount was visible.

## Exact control attempt

The fsynced execution plan SHA-256 was
`ccd429ed8b805f884dd85102d7d4c9f47e06aa69e809d7c0baa1224c937e490c`; the
bound start record SHA-256 was
`b31c72531d23cf0b86b13f2db65f1f089d5883f0ade112c6d79c9cb070f2bf18`.
The exact selected command was `kiro-cli acp`, running as UID 21001 with fresh
`HOME`, `KIRO_HOME`, `TMPDIR`, and `cwd`. The only RPC sent was the adapter's
V2 `initialize` request (protocol version 1, filesystem and terminal
capabilities false). It returned no JSON-RPC response. The process exited 1
without timing out. Its 64-byte stderr was not retained; SHA-256 is
`a36c5aae8bc199708af1c4c6f60e494b3741720694a6ff82ab353c02b6d89d94`. A
streaming classifier recorded authentication-related and generic-startup
indicators, with no network, permission, or owner-state indicators. This is
an authentication-related startup stop, not a captured ACP authentication
error object.

The harness did not send `session/new` because `initialize` did not succeed.
It sent no prompt, tool call, permission approval, or `session/load` request.
The runner supplied no credentials and the container had no egress.

## Observed state paths

The initial synthetic session files under
`KIRO_HOME/sessions/cli/<synthetic-id>` remained at their seeded owner and
modes. The exact KIRO_HOME-scoped filter over the five retained raw trace
files found 14 `openat` calls, all for `KIRO_HOME/settings/cli.json`; it found
no syscall path under `KIRO_HOME/sessions`, no KIRO_HOME `getdents64` path,
and no KIRO_HOME chmod call. Before/after metadata showed only two new KIRO_HOME
entries: `settings` (`0755`) and `settings/cli.json` (`0644`, 2 bytes). The file
contents were not read into evidence. No seeded session owner or mode changed.

The process also created and tightened its separate synthetic
`HOME/.local/share/kiro-cli/data.sqlite3` file to mode `0600`; that path was
outside `KIRO_HOME`. The raw trace has `getdents64` calls without a decoded
directory path. None identifies `KIRO_HOME` or its seeded session tree, so
those calls are not attributed to the owner sweep.

The first generated aggregation counted any syscall line containing the case
root, including relative operations whose `AT_FDCWD` was the test `cwd`. The
Research counts above come from re-filtering the retained raw trace by exact
`KIRO_HOME` or designated outside-home sentinel paths. The runner now applies
that exact path filter.

## Finite inaccessible edges

This run establishes the no-credential startup boundary on exact `2.26.1`.
It does not establish whether the newer exact binaries can initialize without
login. Under the approved stop rule, they were not executed after the control
failed before ACP initialization. These selected-route facts remain
inaccessible in the approved run:

1. Whether `2.27.0`, `2.27.1`, or `2.28.0` V2 ACP initialization reaches the
   owner-sweep initializer and worker.
2. Which seeded session paths those versions enumerate, read, or change; how
   owner, alias, and symlink cases affect the sweep; and which modes change.
3. The selected `initialize`, `session/new`, owner `session/load`, and foreign
   owner `session/load` results on those versions.
4. Whether the startup barrier can be removed with an official provider-free
   mode that does not require a real login or provider request.

The absent session-path syscalls on this control attempt are not evidence that
the newer owner sweep is unreachable. A later proof needs an operator-approved
provider-free startup path. This record authorizes no credential use, login,
provider turn, egress change, qualification claim, route change, or consumer
mutation.

## Retained local evidence

The local evidence tree retains the verified public archives, containment
record, fake result, fsynced plans, process result, raw traces, and summary.
The host summary SHA-256 is
`f76fbc929e2efb661102293d19faa46cb03b6392f899f22f633bb5683c3c8a25`; the
state-path analysis SHA-256 is
`523435dd82120c6ffdd6b7c6292599ce75e37489d90adc507659efad6ef4b057`; the
container result and raw-trace archive SHA-256 is
`00ac0bd6a837007b6e45f2e296a729b0a90b24248064b48310fb8243b9e34efb`. No
session contents or raw stderr were retained. No live authentication,
provider call, host mutation, or compatibility claim change occurred.
