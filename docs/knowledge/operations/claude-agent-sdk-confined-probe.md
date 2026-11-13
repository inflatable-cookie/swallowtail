# Claude Agent SDK confined probe

Private test/proof contract for Q-009. Original SDK/native execution remains
disabled. Production claims and admission stay unchanged.

## Enforcement contract

The launcher owns a fresh directory under the run's TMPDIR. It reads and
verifies the detached source ledger before staging executable source, the
probe admission, options and fake peers. It rejects symlinks and changed
bytes before import. Children receive explicit values for HOME, TMPDIR, TMP,
TEMP, PATH and locale; no inherited Node startup options, proxies or credentials.

On Darwin arm64, test a fixed topology: the supervisor starts one driver and
one native, each under a deny-default Seatbelt policy. Neither may fork.
The spawn hook receives a pipe-backed child supplied by the supervisor; it
does not launch another process. The supervisor observes both wait results,
including normal iterator completion with a hanging native. A missing SDK
join method cannot supply cleanup evidence.

Allow only TCP at the declared loopback port (Seatbelt's localhost filter
covers IPv4 and IPv6), staged read-only sources and system runtime files.
Writable paths must be a finite list with hard per-file size limits, so their
sum bounds storage. Bound captured output during reading,
not after the run. A monotonic deadline or cancellation stops both owned
PIDs and observes both exits. No process-name killing or directory-tree size
polling supplies enforcement.

Use only owned hostile fakes and local endpoints to test each enforcement
property. A failed or revoked mechanism refuses launch. A declared platform
label, caller marker, callback or successful fake is never platform evidence.

## Identity and custody

The detached ledger binds the final launcher, runner, startup shim, plan,
admission, option template and fake sources without a hash cycle. Snapshot
verified bytes into the owned tree and deny child writes to those paths.
Bind the actual Node executable by digest and explicit startup argv.

This does not establish resistance to another hostile process with the same
uid replacing an interpreter or staged source between verification and use.
Do not turn private directory permissions into an immutable-custody claim.
Original activation requires a settled host custody contract as well as
exact operator authority and Nucleus's declared source-immutable proof route.

## Platform gates

The current worker host is Darwin arm64. Its installed sandbox interfaces
are private/deprecated; successful tests establish only the tested host
mechanism. Linux x64 needs an existing concrete host with tested kernel
filesystem/network/process/storage constraints and the exact Node artifact.
Do not install a helper, start or alter a VM, or substitute an arm64 guest.

Before original execution can be requested, retain an exact runnable command,
host identities, all Node/SDK/native digests, settings injection, local endpoint
protocol, finite attempts and limits, independent observations and cleanup.
An unavailable mechanism or unresolved custody boundary blocks that request.
No authority token enables missing confinement.

## Runnable fake stage

From the repository root, with this run's TMPDIR/TMP/TEMP set:

```sh
effigy validate:claude-agent-sdk-confined-probe
effigy check:claude-agent-sdk-confined-probe
effigy check:format:claude-agent-sdk-confined-probe
effigy check:format:claude-agent-sdk-confined-javascript
```

The direct fake command is `python3 -B
scripts/claude-agent-sdk-confined-probe.py --node <absolute-node>
--node-sha256 <exact-executable-digest> --execution-sha256
<confined-execution.sha256-digest> --sdk-version 0.3.322`. Execution selectors
remain the validation route. The source record is detached: it binds the
launcher, both drivers, the startup shim, hostile fake, accepted runner/peers,
old plan/admission/options, new plan and unchanged shipped sidecar. The trusted
host must supply the expected record digest; computing a replacement from
arbitrary caller bytes supplies no authority.

Each invocation makes one attempt, with two processes, ten wall seconds,
ten CPU seconds per process, 64 descriptors per process, no core dump, one
pre-existing writable file capped at 1,000,000 bytes, and 1,000,000 captured
bytes across native and driver stdout/stderr. The supervisor counts native
bytes before relaying them, stops at a monotonic deadline or controlled
cancellation barrier (also SIGINT/SIGTERM on the CLI), kills only its actual
PIDs and observes both wait results.
Denied fork means those PIDs are the entire child tree. The native exit does
not come from an SDK promise. Normal completion closes the pipes and joins;
an iterator ending while a native hangs is a failed run.

The options come from the accepted template. The startup shim supplies a
single child-like pipe handle for the exact native argv and cwd, preserving
the selected SDK flow. The shipped-sidecar driver imports the unchanged asset
and drives open, query, callback denial and joined close. The prepared Rust
driver remains covered by the existing native-proof harness selector; this
does not claim a new public host/process capability.

Children receive explicit environment values and start with `--no-addons`
and `--openssl-config=/dev/null`. The launcher hashes the resolved installed
Node executable and enumerated non-system Mach-O libraries, sandbox-exec and
the installed dyld support profile. It snapshots verified repository bytes
before import and denies child writes to executable sources. It does not
freeze the interpreter, its libraries, the Python supervisor or OS policy
against a hostile same-uid process. That custody gap remains a blocking gate.

## Concrete host and missing original proof

Read-only inventory on 2026-11-13 found Darwin arm64, macOS 26.6.2 / Darwin
25.6.0, with installed sandbox-exec. The available PATH Node is 26.11.0,
SHA-256 `832eaf52cafa7ff1364c2ca378451db14fb043d8d23a5cfd18a0a103d66226fc`.
It is a fake-test tool, outside the qualified Node segment. No Node 26 support
or qualification follows. Existing Colima profiles `effigy` and
`effigy-release` are running aarch64 guests; neither is a Linux x64 proof host.
No guest was started or changed.

Original execution is not runnable or admitted on this inventory. Missing:

- A settled interpreter/source custody contract; no weaker same-user
  assumption is adopted here.
- An existing Linux x64 host with exercised kernel confinement and exact
  Node identity. The launcher refuses Linux rather than simulating Darwin
  proof with a platform label.
- Provisioned exact Node 22.23.2 or 22.23.3 executables on both selected
  hosts. The accepted Darwin identities remain in the Node fixture; a Linux
  identity is not invented from that evidence.
- Isolated native administrator-tier settings injection without modifying
  the host. `managed-deny` reproduces the outcome in an owned fake; it does
  not prove a real binary's administrator discovery or settings enforcement.
- A reviewed local provider-protocol endpoint and exact original commands
  for every frozen pair. The fake endpoint records one owned POST; it is not
  a proven substitute for the native's provider transport.
- Exact operator authority and a declared Nucleus source-immutable
  author-side proof selector. Do not activate that route from this launcher.

The accepted `probe-plan.json` retains every wrapper/native digest for
0.3.318/.319/.321/.322 and the .317 control on Darwin arm64 and Linux x64.
Those artifacts are not downloaded again or executed. Until the missing
capabilities and authority exist, Q-009 stays open and production remains
SDK 0.3.317/native 2.1.317, with all older points and exclusions retained.

Python formatting uses installed Ruff 0.14.1 through the repository's
check-only helper. The JavaScript format route uses installed eclint with
explicit two-space, UTF-8, LF and trailing-whitespace/final-newline rules;
`node --check` separately proves syntax. No formatter is installed by a selector.
