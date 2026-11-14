# SDK loopback transport proposal

Private, fake-only probe preparation for Q-009. The single namespace attempt
below produced partial observations and refused its result; no complete host
or original activation is admitted.
Tom settled SDK custody in decision `89db0f25-dc94-4feb-b447-8b9fabe52561` v2:
"Explicit trusted-host SDK custody." The private implementation below owns it.
The canonical-fixture grant is consumed; no canonical path is touched here.
Production stays SDK 0.3.317/native 2.1.317, frozen target 0.3.322.

## Contract before implementation

Use one fresh Ubuntu GitHub-hosted job, not a general host/container service.
The concrete precedent is the Ubuntu x86_64 hosts in run 43420542403, kernels
7.0.0-1019-azure and 7.0.0-1016-aws. Their successful file-fixture proof does
not establish namespace, pidfd, subreaper or privilege availability.

The proposed root monitor owns one guest and enables child-subreaper behavior
before forking. The guest closes inherited network/foreign descriptors, creates
one unnamed `CLONE_NEWNET` namespace and raises only `lo`. It adds no veth,
external interface, gateway, default unicast route, DNS or host network policy.
Actual namespace device/inode, `/proc` interface/routes and credentials must
agree before entry. The endpoint and two supervised fake Node children share
that namespace. Ordinary TCP reaches the local Messages endpoint without
patching connection functions or inheriting a connected socket.

After setup, the guest clears supplementary groups/capabilities, permanently
drops to the original non-root runner uid/gid and sets no-new-privileges.
Node children receive the selected owned SDK/native sources, read-only staging,
isolated HOME/workspace/TMP and an explicit credential-free environment.
Landlock limits files; seccomp denies forks, namespace escape, foreign-process
access/signalling, non-TCP sockets and storage bypasses. Only one existing
writable file is permitted. Missing enforcement refuses the kernel proposal.

Bounds: one attempt, one guest, two Node processes, endpoint as a guest thread;
30 seconds including setup/protocol/cleanup, 10 CPU seconds per Node process,
64 descriptors per Node process, 1,000,000 captured bytes and one writable file
capped at 1,000,000 bytes. The job has a five-minute outer limit. Setup and
protocol phases must report progress under a root-owned deadline; cleanup has
a separate finite join deadline. Failure never implies permission to retry.

Root teardown stops only observed owned descendants, binds PID identity through
pidfds/start time/namespace and observes waits, including adopted descendants.
The guest separately observes both Node waits. Missing joins, failed cleanup,
unknown children or identity drift are failed settlement, never a success
marker. No namespace name or namespace descriptor is kept after settlement.
An outer hard kill or missing receipt is a failed proof requiring disposal of
the dedicated ephemeral job, not evidence of successful cleanup.

## Current proof limits

Current authority permits owned fake namespace/tool operations, ordinary local
endpoint/SDK processes, source checks and scratch cleanup only. Model results
must say `kernelEnforced: false`. They exercise orchestration, protocol and
refusal paths, not `unshare`, loopback setup, capability removal, root teardown
or kernel namespace isolation. The proposed root branch is never called by
current selectors. Original mode remains closed.

Required rehearsals: setup refusal; cancellation/deadline at every phase;
partial setup; cleanup failure; wrong namespace; inherited socket; missing
join; source/admission drift. Real owned processes are stopped and waited even
when a simulated observation fails. No real user file or provider endpoint is
a negative target. Existing passing kernel and canonical-fixture evidence is
retained without replay.

The eventual setup request must bind exact helper, entry, source/admission,
Node and startup bytes, the approved preparation diff and actual host tool
observations. It grants only one namespace-only fake attempt. It cannot grant
original code, another canonical fixture attempt, a custody model, installation,
host provisioning or a public runtime capability. Operator approval of an
exact new execution source is required before dispatching that job.

Mechanism references: [network namespaces](https://man7.org/linux/man-pages/man7/network_namespaces.7.html),
[unshare](https://man7.org/linux/man-pages/man2/unshare.2.html),
[subreaper](https://man7.org/linux/man-pages/man2/PR_SET_CHILD_SUBREAPER.2const.html),
[pidfds](https://man7.org/linux/man-pages/man2/pidfd_open.2.html).

## Exact proposal surface

The [approved preparation diff](claude-agent-sdk-netns-ci.patch) contains the complete
preflight, one attempt and always-cleanup commands. It adds only one fresh
`ubuntu-latest` job, `contents: read`, Node 22.23.2 and a five-minute job bound.
The diff also carries the minimal exact-byte non-Cargo routing exception; its
mutation rehearsal refuses changed permissions and unrelated workflows.
Tom decision `05d81f49-83da-42d0-b7ab-8a9bc29212ee` v2 authorizes
application/registration of those exact bytes from source36fc153ac. The patch
SHA256 is5b804c0693aff7fc1d09a8654f32ad4ab761354236c5f51154732760c85ae700;
applied workflow SHA256 is07a24539f219a477886cae19f9de5d738cc021d998469401eefb3faa2c2c1d19.
Task236 registered the same workflow and narrow routing exception on canonical
main `93db082092203b1d87c2665c5b1d2a988191f709` (PR555). This branch
reconciles that commit without duplicating either addition. Read-only GitHub
verification on2026-11-13 reports workflow id417366274, path
`.github/workflows/sdk234-netns-proof.yml`, state `active`, and zero
`workflow_dispatch` runs. Registration grants no execution; the resulting
fresh source and detached record need separate activation authority. No retry
is granted. The historical proposal digest is not the current execution binding.

The source record `claude-agent-sdk-netns.sha256` binds the coordinator, root
monitor, child entry, ordinary-TCP fake, tests, admission, this contract, job
diff, Effigy selectors and retained confined-source record. It excludes itself;
the separately approved record digest is the external root. Verified bytes are
captured before importing helpers. The unchanged accepted peer record still
verifies the selected SDK/prepared-driver/sidecar sources before staging.

Read-only preflight records actual OS/kernel/architecture, Python and tool
paths/digests, Node version/digest, Landlock VERSION query, capabilities,
namespace readability and a self pidfd open/close. It creates no namespace,
filter or policy. Missing `sudo`, GNU `timeout`, Python, pidfds or Landlock ABI 3
refuses. The proposed Linux Node digest is
`3517c2df0b2f8cd7f422b4b8450ef81c6889f08eb03e281d6de9079b15e6a327`.
Python/timeout identities are observed at preflight and checked again against
`/proc/self/exe` and the monitor's parent executable. This detects observed
changes; it implements the approved trusted-host scope and does not
promise verified execution against a hostile same-user replacement.

The only proposed privileged operations are two exact `sudo -n timeout`
invocations shown in the diff: 35 seconds plus a two-second kill grace for
`--kernel-proposal`, and five seconds plus grace for `--cleanup-proposal`.
Both clear the environment with `/usr/bin/env -i`, preserve only explicit
private TMP/HOME, locale and system PATH, and pass exact record/Node/Python/
timeout identities and the original non-root uid/gid. Root creates an exclusive
0600 receipt inside the run's fresh 0700 runner-owned scratch. It enables
subreaper behavior, forks one guest and performs only pidfd observations,
bounded pipe exchanges, a host-loopback canary and owned waits/signals.
The guest alone calls `unshare(CLONE_NEWNET)`, raises `lo` by ioctl, drops all
bounding/ambient capabilities and supplementary groups, then sets gid/uid and
no-new-privileges. No named namespace, mount, veth, route change, install,
canonical settings file or host firewall change is involved.

Root acknowledges each Node child only after observing its parent, namespace,
uid/gid and empty capability sets. A pipe gate prevents Node entry before that
acknowledgment. The child entry checks namespace and inherited descriptors,
then applies Landlock, seccomp and resource bounds before exact Node exec.
The guest's endpoint remains in the new namespace; native TCP connects to it
normally. A second ordinary TCP connection targets a root-created host-loopback
canary: a definite refusal and independently unreached canary are required.
A timeout is failure, not isolation evidence. The monitor checks that only `lo`
and no external/unicast default route exist through cleanup.

The monitor uses pidfds and subreaper waits, with SIGCHLD default behavior,
and requires pidfd HUP for every registered process (reaped, not merely exited).
Independent cleanup reads the root receipt, checks source/directory/file/PID
identity and settled observations, refuses live or reused processes, and removes
only that receipt. Temporary payload teardown belongs to the guest's own fresh
fixture. Unknown or hard-killed monitor state fails and requires disposable-job
settlement; it cannot become a passed proof through a cleanup marker.

The shipped sidecar control uses the retained managed-denial scenario. The
selected probe separately exercises callback denial and managed denial; this
proposal does not extend the old sidecar driver's callback-response coverage.

## Required authority and unobserved capabilities

Request only one application/registration and one exact-source namespace-only
owned-fake job, with the diff's two setup/cleanup invocations. The first approved
source must stay immutable through dispatch; an unknown response is reconciled
by reads, never blindly replayed. A changed helper, source, admission, runtime
or job identity refuses and needs a new request.

The consumed namespace attempt below supplies limited new namespace,
credential and reaping observations. Complete peer result/transport/filter
observations and an exercised adoption case remain missing. The old Linux
kernel proof uses a different networking boundary and cannot supply them.
Source-only BPF interpretation and modeled namespace records cannot admit a
host. Capture actual preflight and independent kernel/credential/route/wait
records before claiming complete enforcement.

This request excludes another canonical-fixture attempt, original SDK/native
execution, provider credentials or spend, installs, host provisioning, Darwin
changes and production admission. Even successful fake TCP isolation would not
prove original administrator discovery or selected original behavior. The SDK
custody policy is settled; the future original proof still needs its
own concrete source-immutable Nucleus route and exact execution authority.

## Private custody admission

The host constructs a live private custody lease from captured verified source,
not a caller attestation. Decision89db0f25 v2 excludes hostile arbitrary
same-user replacement; hashes, descriptors and staging are not OS-enforced
verified execution. The admission remains fake-only and cannot authorize an
original artifact or admit an unobserved namespace mechanism.

The ordinary hosted preflight observes the exact Node executable and loaded
shared-library list using a fixed credential-free Node report command, with a
five-second timeout. No provider or probe payload runs in that observation.
Its output joins the existing private preflight file; root requires its runner
ownership, fixed Node22.23.2 identity and startup policy. The host then opens and
hashes exact Node/libraries, Python executable/imported modules/mapped libraries,
loader cache, source record, supervisor/entry/sidecar/SDK/native and loader inputs.
Root also binds its actual timeout parent executable. Ambient loader preload is
refused. Inputs must be regular, owned by root/the trusted runner and unwritable
by other users. Open descriptors and device/inode/owner/mode/size/mtime/ctime
observations stay owned through launch. Captured source bytes feed helper imports
and staged payloads. Each launch rechecks the live lease and exact command/env;
missing, replaced, changed or host-revoked inputs refuse before payload effects.

The lease is an in-process host object, bound to the owning PID and explicit
trusted fork, not a serializable success flag. The fixed two launch commands,
Node flags, isolated paths and explicit environment are registered before spawn.
Staged modules/configs are pinned before each child entry. Failure after one child
starts kills and actually waits that owned process and closes custody descriptors.
The host releases the lease only after protocol teardown; root retains its own
copy through namespace settlement. Independent host/runtime admission still
needs actual target-host observations in the separately authorized attempt.

The retained guest topology follows the design consultation's alternative:
the root monitor pins the guest from its own fork and peers from the trusted
guest's actual Popen identities under a start barrier, validating kernel parent,
namespace and credentials before release. Guest waits are recorded with their
actor; the root cannot wait a live grandchild. It independently requires pidfd
HUP for those registered peers and actually waits the guest or adopted children.
ECHILD or disappearance alone never establishes a peer join. A zero-status guest
wait and both exact peer wait identities are required for success. After final
setresgid/setresuid, parent-death SIGKILL is rearmed and the expected monitor is
rechecked before payload entry. Credential-drop/monitor-death uncertainty and
unknown cleanup remain failures, not disposal-as-join claims.

Darwin local rehearsal limitation: Node reports system dylibs whose bytes reside
in the OS shared cache without regular files at their advertised paths. The
rehearsal retains those names as `unresolvedSystemLibraries` and reports
`runtimeCustodyComplete: false`; this supplies no Darwin runtime admission.
Linux activation requires a nonempty complete regular-file library inventory
and refuses unresolved libraries. Pinning Darwin cache/runtime custody remains
required before any corresponding original-host admission; the policy ruling
does not waive that observed input gap.

The host reserves the five explicit protocol pipe pairs before opening custody
inputs, refuses unavailable descriptor numbers below64, and transfers each pair
to exactly one cleanup owner. The root monitor closes its copies immediately
after the trusted fork; it cannot keep a guest writer alive. A separate native
start pipe releases the ordinary-TCP fake only after both custody-checked peers
exist. This preserves the declared descriptor bound while holding the complete
input inventory. Host exit/notification timing is bounded retained observation,
not a replacement for actual guest waits and monitor pidfd reaping.

Linux preflight checks the known Node22.23.2 executable digest before executing
its read-only runtime report. A reported Linux vDSO is identified explicitly as
a kernel-provided mapping bound to the observed kernel release, rather than
inventing a regular-file digest. Unknown library names and missing file inputs
refuse; imported Python inputs are refreshed before configuring the two launches.
The thirty-second inner budget includes root custody capture; preflight remains
within the job's five-minute bound. No passing fixture or old kernel proof was
replayed to support these new source-only custody observations.

Read-only local inspection found existing cache components under
`/System/Volumes/Preboot/Cryptexes/OS/System/Library/dyld`, including the
arm64e index and multi-gigabyte subcaches. Their existence is a feasible
byte-inventory lead, not a cache identity or library-to-cache binding. This
Linux-only attempt neither hashes/adopts that cache set nor claims it absent.
Before original Darwin admission the host must bind the actual mapped cache
set and exact identities in preflight, with a separately bounded preparation
budget; current local rehearsals explicitly retain the unresolved names.

## Consumed namespace-only attempt — 2026-11-14

Tom decision `486c23c9-a77f-45ed-8299-84a01f501532` v2 authorized one
namespace-only fake attempt at source
`cc4ffc2248721a4da38fd291d8c9b1738d3a56aa`, detached record
`e579b92941aedd484ef4d9cf9ec04812f4130e5cf0d91afd0bf554d326faf4e0`.
Immediate local/remote/workflow/record verification found the exact requested
identities and zero prior dispatches. Exactly one command produced
[run43531230335](https://github.com/inflatable-cookie/swallowtail/actions/runs/43531230335),
attempt1, [job132249164711](https://github.com/inflatable-cookie/swallowtail/actions/runs/43531230335/job/132249164711).
The namespace fake step failed; independent settlement succeeded. The grant is
consumed. No rerun or second dispatch occurred.

Actual host: Linux x86_64, kernel7.0.0-1019-azure, Landlock VERSION query ABI8,
Node22.23.2 with digest3517c2df0b2f8cd7f422b4b8450ef81c6889f08eb03e281d6de9079b15e6a327.
Python3.14 executable digest wasbe9a2a5eada8c89c1c399fdfb8397179e877c20d8db0739c802f726d4d0e69fd.
Preflight retained actual library/tool identities, no unresolved Node system
libraries and the observed kernel-provided vDSO mapping. It is runtime evidence,
not a new SDK/native qualification.

The checked root receipt records a distinct guest network namespace shared by
the two registered peers. Guest/peer uid/gid sets are1001, supplementary groups
empty and all five recorded capability sets zero. All seven phase exchanges
reached cleanup. From the exact monitor's acknowledged checks, loopback/interface
and route guards completed; their raw snapshots were not separately retained.
The outside canary remained unreached; without the peer result this does not
prove that the forbidden connection was attempted or denied.

The failure is `[Errno 1] [Errno 27] monitor_message_limit`. The root custody
observation alone was18185 JSON bytes over157 inputs, exceeding the16384-byte
control-frame bound before adding guest/protocol evidence. Guest completion
could not send its complete result. The monitor waited guest2457 with status78
(raw wait status19968), and independently observed pidfd HUP for2457,2459,2460.
The receipt's settled observation and independent identity-checked cleanup
succeeded. Peer exit codes, full selected protocol output and socket/filter
observations were not retained; `kernelResult` is null. Do not infer peer
success, actual native administrator discovery or complete confinement from
these partial observations. No separate post-cleanup filesystem inspection is
claimed.

The source correction keeps the16384-byte frame and65536-byte root-receipt
limits. A complete result is capped at32768 bytes and sent as at most six
6000-byte chunks, each separately acknowledged, with exact sequence, length
and SHA256 verification before acceptance. Full custody evidence is preserved
instead of replaced by a success marker. Oversize, reordered, changed, truncated
or invalidly encoded results fail. Owned-pipe tests exercise the actual framing
without calling privileged namespace/credential methods. This correction has
not run on the hosted kernel and grants no retry.

The new private admission names `kernelProofAccepted: false`; it does not use
the old preparation flag `kernelActivationObserved: false` as a claim that no
namespace was ever created. A fresh exact-source authority is required for
another namespace-only attempt. The canonical-fixture grant remains spent;
original SDK/native execution and qualification remain separately gated.
