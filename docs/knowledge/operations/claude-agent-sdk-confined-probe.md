# Claude Agent SDK confined probe

Private fake-only execution contract for Q-009. Original/native execution and
qualification remain gated. Production stays SDK 0.3.317/native 2.1.317.

## Scope and identity

The launcher owns a fresh fixture below the run's TMPDIR. It verifies the
externally supplied digest of `confined-execution.sha256`, reads regular source
files without symlinks, snapshots their bytes, and denies child writes to the
staged code. The detached record binds the launcher, Linux entry, startup
carriers, drivers, hostile fake, tests, CI test routing, accepted plan/admission/
options/peers and unchanged shipped sidecar without a hash cycle. A replacement
record chosen by a caller supplies no authority.

Each run binds the resolved Node executable digest, explicit startup argv,
policy bytes and accepted fake tuple. The native startup carrier reports its
actual `process.platform`, `process.arch` and `process.version`. The suite
prints `SDK_CONFINED_PLATFORM_PROOF` with host OS/kernel/architecture, Node
version/digest, source-record digest and observed kernel/settings capabilities.
Platform labels supply no qualification.

## Fixed enforcement boundary

Two supervisor-owned sibling processes run the selected SDK driver and native
fake. The one-use spawn hook connects their inherited pipes for the exact
native argv/cwd; it creates no child. The supervisor accounts native bytes
before relaying them, stops only its owned PIDs and observes both actual exits.
SDK join promises supply no cleanup verdict. Normal iterator completion with a
hanging native remains a failure. CLI SIGINT/SIGTERM requests the same cleanup.

Each invocation permits one attempt, two processes, ten wall seconds, ten CPU
seconds per process, 64 descriptors per process, no core dump, one existing
writable file capped at 1,000,000 bytes, and 1,000,000 captured stdout/stderr
bytes. No sampled directory-size check supplies enforcement. Native Node
threads share their process; Linux clone is restricted to that case.

| Property | Darwin arm64 | Linux x64 |
| --- | --- | --- |
| Files | Deny-default Seatbelt; declared runtime paths, path ancestors, staged reads and one explicit fake administrator file | Landlock ABI >=3; staged/runtime/fake-settings reads, one writable file; missing ABI refuses launch |
| Metadata | Outside-file stat and mutation denied | Stat is not mediated by Landlock; metadata-changing syscalls are denied by seccomp. This limit is explicit |
| Egress | TCP at the declared loopback port, IPv4 or IPv6 | New sockets/connect/bind and io_uring denied; one already-connected owned endpoint descriptor may be inherited |
| Processes | Further forks denied | fork/vfork denied; clone3 returns ENOSYS; clone requires CLONE_THREAD; foreign-process signal/control paths denied |
| Storage | Finite writable-path policy plus hard RLIMIT_FSIZE | Finite writable-path policy plus hard RLIMIT_FSIZE; anonymous memfd and SysV IPC creation also denied |

Children receive explicit PATH, HOME, TMPDIR/TMP/TEMP and locale values. They
start with `--no-addons --openssl-config=/dev/null`; credentials, proxies and
ambient Node startup variables are absent. Linux requires an unprivileged uid
with no effective/permitted/ambient capabilities before applying no-new-privs,
Landlock and seccomp. Kernel refusal is a failed proof, never a fallback.

## Existing CI hosts

The planner's 2026-11-13 continuation names the existing Ubuntu nextest shards
and macOS process job, both already configured with Node 22.23.2. No workflow,
installation or VM change is required. The Rust test
`confined_platform_boundary_matches_running_host` runs the Python suite in its
own temporary fixture. The nextest override exposes its successful proof
output on both jobs. CI refuses an actual Node version other than 22.23.2;
architecture and kernel identity come from the running processes.

Local fake tests may use the installed Node outside the qualified segment.
That observation gives no Node support credit. CI proof must be read at the
exact pushed head before naming either selected platform proven.

## Endpoint and settings proof

The owned endpoint accepts one bounded, credential-free POST to `/v1/messages`
per protocol attempt. Its response streams message-start, content-block-start,
text-delta, content-block-stop, message-delta and message-stop events. Independent
observations bind the request, frame order, end-turn reason and usage counters.
The fake native startup carrier validates the stream before the existing
stdio-native fake exercises init, input/result/error/usage, permission denial,
interrupt and joined close. This proves an owned protocol peer; it does not
prove an original native's provider transport.

A read-only owned administrator fixture sits outside HOME/workspace. Opposite
user/project settings remain in those roots. The fake carrier reads only the
explicit administrator fixture and records its digest and effective denial;
callback denial and managed denial stay distinguishable. Records explicitly
say `originalAdministratorDiscovery: false`. This carrier is not a genuine
injection at the vendor's canonical administrator path.

The suite separately observes whether Linux permits
`unshare --user --map-root-user --mount -- /bin/true`, retaining the actual exit
and diagnostic. It neither installs a helper nor changes a mount/VM. A success
is namespace capability evidence only. Darwin Seatbelt has no path-remap
operation and the host administrator path is not provisioned by this task.

## Original activation gate and feasible proposal

Original mode refuses unconditionally. Missing:

- A settled interpreter/source custody contract. Staging, Node/library hashes
  and child write denial do not resist hostile same-uid replacement of the
  supervisor, interpreter, libraries or host startup policy.
- Genuine administrator-tier injection at the canonical path without host
  mutation. The documented paths are `/Library/Application Support/ClaudeCode/managed-settings.json`
  and `/etc/claude-code/managed-settings.json`. No host
  file at either path is read, written or used as a fake negative target.
- Exact original-artifact authority and a structurally declared Nucleus
  source-immutable author-side proof selector.

Feasible owner proposal: provide an already admitted ephemeral execution host
with the canonical administrator fixture preprovisioned, or admit a private
filesystem mapping where kernel namespace support is actually observed. Bind
that host policy, qualified Node executable and frozen SDK/native artifacts;
exercise the canonical-path mapping with owned fakes first. Darwin needs an
existing provisioned boundary because Seatbelt cannot supply that mapping.
This task grants no privileged helper, host provisioning or VM change.

The planner's 2026-11-13 continuation requests a concrete
[ephemeral-CI environment proposal](claude-agent-sdk-ephemeral-ci-proposal.md).
It supplies the exact job diff, canonical fixture states and bounded root
setup/cleanup commands. Tom's decision `32d31615-702b-4e3b-a972-56b4fb18bae2`
v2 permits applying that workflow; exact-SHA privileged activation is still
gated. Owned-scratch rehearsals test the fixture
lifecycle; they do not establish canonical injection or authorize provisioning.
The proposed fixture-only authority is separate from custody and original
SDK/native activation. Existing passing confinement evidence stays intact.

Concrete Linux observation: [Ubuntu CI job 131785252340](https://github.com/inflatable-cookie/swallowtail/actions/runs/43395484971/job/131785252340)
at source `0d4713088098238906534a540a11bbd9a6d16854` ran the fake boundary on
x86_64, kernel `7.0.0-1019-azure`, Landlock ABI 8 and Node 22.23.2 (executable
SHA-256 `3517c2df0b2f8cd7f422b4b8450ef81c6889f08eb03e281d6de9079b15e6a327`).
Its namespace probe exited 1: `unshare: write failed /proc/self/uid_map:
Operation not permitted`. That runner cannot supply the proposed rootless
canonical-path mapping. An owner-provided admitted boundary is required;
this diagnostic authorizes no privileged fallback.

The accepted `probe-plan.json` retains all exact wrapper/native identities for
0.3.318/.319/.321/.322 and the .317 control on both selected platforms. No
unchanged artifact is downloaded again or executed. Original activation still
needs exact authority through Nucleus; no token enables missing confinement.

## Commands and sources

Run the confined suite and its compile/format routes through Effigy:
`validate:claude-agent-sdk-confined-probe`,
`check:claude-agent-sdk-confined-probe`,
`check:format:claude-agent-sdk-confined-probe`, and
`check:format:claude-agent-sdk-confined-javascript`. Python uses installed Ruff
0.14.1; JavaScript uses installed eclint plus separate `node --check` syntax
proof. Selectors install nothing.

The direct fake proposal is `python3 -B
scripts/claude-agent-sdk-confined-probe.py --node <absolute-node>
--node-sha256 <exact-node-digest> --execution-sha256 <record-digest>
--sdk-version 0.3.322`. Execution selectors remain the validation route.

Primary mechanism references: [Landlock limitations](https://docs.kernel.org/userspace-api/landlock.html),
[nextest per-test output](https://www.nexte.st/docs/configuration/per-test-overrides/),
[Messages streaming](https://platform.claude.com/docs/en/build-with-claude/streaming),
and [administrator settings paths](https://code.claude.com/docs/en/settings).
These describe mechanisms and protocol shapes; they supply no frozen-native
behavior or qualification evidence.
