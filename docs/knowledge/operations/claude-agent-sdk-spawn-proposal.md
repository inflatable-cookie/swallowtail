# SDK child-spawn and combined-fixture proposal

Private Q-009 source/fake preparation. Original admission is closed.
Production stays SDK0.3.317/native2.1.317; the frozen target stays0.3.322.
The successful namespace fake proof at05af44845 supplies transport and fixed
peer custody/settlement evidence only. Its process filter forbids this new tree.
No old grant authorizes activation of the proposal below.

## Testable contract

Keep the unchanged shipped sidecar and its actual `spawnNative` callback. The
owned fake SDK invokes that callback once, creating a genuine child process;
no precreated native, spawn monkey patch, descriptor proxy or invented exit
notification stands in for it. The selected probe driver uses the same option
surface and a real child too. One trusted monitor owns one supervisor/endpoint;
that supervisor starts one driver, which starts one native through the SDK hook.
A captured host-owned Python entry blocks before exec of the owned native
payload. The payload does not implement or attest its own release.
The host validates the actual parent/command/identity before release. Caller
PIDs and success flags are not kernel custody. A second native request,
unregistered child, changed input, revocation or missing release refuses.

The unprivileged rehearsal uses only owned fakes, pipes/files and a local
endpoint in a fresh current-run scratch fixture. It reports
`kernelEnforced: false`, no complete independent grandchild reaping on hosts
without pidfd observation, and no administrator discovery. A real ChildProcess
exit/close and actual driver wait prove their software paths; disappearance,
ECHILD and host disposal never substitute for independent full-tree settlement.

For the future Linux host, retain the observed unnamed lo-only netns and
credential drop. A private exclusive cgroupv2 subtree bounds the whole payload,
including threads: pids.max32, memory.max256MiB, memory.swap.max0 and CPU quota
one core. Require actual controller delegation, cgroup.kill, freeze/events and
pidfd support before setup. Do not assume hosted availability. Inherit Landlock,
seccomp, no-new-privileges, explicit credential-free environment and finite
CPU/FD/file/output/deadline limits before the driver can start a child. The
changed process policy may permit bounded ordinary SDK spawning, never namespace,
FD-import, ptrace or cgroup escape. The task budget is a kernel process/thread
bound, not proof of exactly one start: the host separately refuses additional
starts and unexpected kernel-enumerated members, then stops the whole owned
subtree. Node thread requirements are measured, not guessed into qualification.

Pin the driver under a trusted fork barrier and the native under its trusted
entry barrier before payload release. Enumerate the exclusive cgroup through
kernel files; bind pidfds, parent/start/executable identity and exact launch
inputs. A proposed trusted native entry execs the exact artifact after release;
it supplies no SDK-level wire answer. Actual vendor argv/path classification
must agree with the frozen ledger before original admission. The current fake
entry rehearses that start gate; no opaque executable runs here.

Cleanup first asks the actual sidecar/probe to close, records its real native
wait, and independently observes pidfd reaping. At deadline/cancel/failure use
only the owned cgroup kill and trusted process identities. Freeze/enumerate to
register otherwise unobserved members; the monitor reaps actual adoptees after
parent exit and actually waits its own child. Require empty cgroup population,
all registered pidfd reap observations and actor-specific waits. Unknown reaping
or hard monitor death stays failed/unknown; do not call disposal successful join.
Every setup, entry, protocol and cleanup phase has a finite deadline. No kernel
or cgroup branch is activated by the ordinary selectors.

## Combined administrator fixture

Use three fresh Linux jobs, one each for absent, disabled and managed-deny.
Keep the exact SDK/native/Node/source/runtime identity ledger and owned local
endpoint in every job. In that same job, before the probe and after read-only
actual host preflight, use the existing exclusive root canonical-fixture helper
with its opposite HOME/project controls. Bind its root-owned receipt to that
job and the new launch record. Ordinary-uid fake/native reads must happen inside
the same admitted netns/process/filesystem boundary. Landlock grants only the
canonical fixture read, never write. Always perform checked fixture cleanup and
cgroup/process settlement independently; partial setup must clean only objects
whose exclusive identity this job created. Absence must be an observed exact
path absence, not a fake environment flag.

The proposed states are the existing fixture helper's exact paths and bytes;
no second invocation of its consumed workflow or reuse of its old grant is
requested. A future combined manual job needs separate exact workflow-source,
record, state/attempt and minimal privileged setup/cleanup authority. The exact
patch is applied under the preparation ruling below; execution remains gated.
No local canonical write, cgroup provisioning or host update is a rehearsal. Darwin needs separately admitted shared-cache/runtime custody and
its own combined boundary; this Linux proposal makes no Darwin claim.

## Required evidence and refusal cases

Rehearse real callback denial versus managed pre-callback denial, init, streamed
input/result/error/usage and actual SDK-triggered native exit through the shipped
sidecar/prepared driver. Exercise changed source/launch inputs, revoked custody,
second start, wrong parent, early/unregistered entry, setup refusal/partial setup,
cancel/deadline before and after native start, iterator completion with a hanging
native, output/storage/process excess, missing wait/pidfd and failed cleanup.
Kernel-shaped models must stay explicitly models. Publish exact commands and
source/record/host prerequisites, distinguishing runnable unprivileged rehearsals
from any not-yet-observed kernel admission. No original/native proof request is
ready until the changed process boundary and combined settings host are actually
admitted. No public runtime capability or generic sandbox service is added.

## Implemented software checkpoint

The source-only rehearsal runs the real shipped sidecar or its retained prepared
option driver, an owned SDK wrapper and an actual ChildProcess. The wrapper
checks the SDK-generated command/argv/cwd/environment, then uses the unchanged
sidecar spawn callback to start the captured Python entry. Host checks its
actual OS parent/group/start/command and live custody before approval; the entry
checks its own Python/source and target bytes and execs the exact Node fake
with unchanged PID/stdio and explicit environment. This is trusted-host custody,
not a claim that wrapper paths cannot be bypassed by arbitrary admitted code.
The prepared option-driver copy adds an explicit wait on the retained real
ChildProcess exit before its existing process exit; the accepted source itself
is unchanged. All captured and derived staged bytes are pinned.

Node-only startup flags apply to the owned Node peer, not an invented ELF-native
argv rule. macOS's runtime-added `__CF_USER_TEXT_ENCODING` is explicitly removed
from the private SDK handoff. The actual framework Python executable is resolved
from the host process and bound separately from its launcher. Local Darwin
shared-cache names remain unresolved and complete runtime custody remains false.
This rehearsal grants no Node22, Linux cgroup or Darwin original admission.

Current tests exercise callback versus managed denial across all three owned
settings states, real local HTTP/streaming native wire, actual native/driver
waits, parent/second-start refusal, revocation before driver/native entry, changed
staged native, source/symlink mismatch, cancellation, live output/storage and
hanging native limits, missing waits and cleanup failures. A refused signal
regression verifies endpoint threads and custody descriptors still close. Owned
scratch settings are not canonical administrator-path proof. Ordinary software mode activates no kernel or canonical setup; original mode
remains unavailable. The inactive adapter is described below.

Planner design answer to questionc84fed65: no blanket USER_NOTIF requirement,
and no new trust ruling. Custody does not make admitted SDK/native code trusted.
Before original admission, every reachable spawn/exec/worker/dynamic-code/addon
edge must be mediated, prohibited or source-proven unreachable for the exact
invocation. An executable bypass of the wrapper keeps that invocation refused;
detecting it after target effects is failure. Cgroup task limits include threads
and must apply before untrusted code, with migration/controller/monitor escape
blocked. Empty membership is not reaping. The vendor graph and actual changed-kernel admission remain unproved; this
source/fake milestone is not a ready original execution request. A seccomp-notification CONTINUE over mutable argv/path memory
would not itself close that gap.

Coordination: task227 supplies exact captured facade/loader evidence at
`c0d591438590f77f502650a47b44e96295cf23f3`, including
`crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-acp-guard-loader.mjs`
(digest512a2b607f94b29ff215ed2eb6028ab09bc6a06ee30e14b6d6c9f798d5f0c71a).
It is ACP-specific evidence, not SDK qualification or a complete native exec
mediator. Reuse its private invocation/event seam without changing public APIs,
borrowing authority or duplicating the cgroup/namespace implementation. ACP's
installed-compatible Node axis remains separate from this SDK target.

## Composable source/fake boundary

The private `swallowtail.sdk234.private-spawn.v1` record now has explicit finite
actor profiles. `sdk` binds trusted guest → checked Node driver → native child.
`acp-actor-shape` adds a captured Python group owner: trusted guest → group owner
→ checked Node driver → native child. The latter is an owned stand-in for the
outer Rust prepared host / LocalProcessHost group owner / checked ACP entry,
not an execution of ACP or an extension of its claim. Driver PID and group leader
are independent. The native's parent is the registered checked Node actor.
All actors come from OS observations of host-owned starts, never route trace
PIDs or a public runtime handle. ACP227 binds its own captured actors, runtime,
source closure and route tests to this seam; SDK22 and ACP's installed-compatible
Node remain separate axes.

Each invocation binds actual source-record/runtime identities, the captured
entry/facade/graph bytes, exact argv/environment/cwd, actor parent/group/start,
fixture state and limits. Events carry `seq`, `phase`, `kind` and actor-specific
identity or wait fields. Native start/release/entry, actual ChildProcess exit,
parent driver/group-owner waits, stop requests, independent terminal waits,
pidfd HUP, cgroup emptiness and fixture cleanup are separate observations.
A caller cannot instantiate admission by serializing this record.

The selected first-party graph uses captured synchronous resolve/load hooks,
including CJS and parent-bound `createRequire`, derived from the pinned ACP
loader above. Only the shipped sidecar and retained option driver may obtain
the one exact registered real ChildProcess spawn. Shells, alternate Python/Node
arguments, additional starts, workers, vm/string code, addons, data imports and
unregistered process bindings refuse before the owned effect marker. Fixed
startup includes `--no-addons`, `--disallow-code-generation-from-strings` and the
captured bootstrap. This closes the tested owned invocation, not arbitrary
JavaScript or an opaque vendor dependency graph. Unknown vendor edges still
refuse original admission. Trusted-host custody does not turn payloads into
trusted code.

## Inactive Linux combined adapter

`scripts/claude-agent-sdk-spawn-combined.py` now expresses the proposed host
orchestration. Its root modes are unavailable to ordinary rehearsals. Source
state-machine tests supply owned event/filesystem models and explicitly report
`kernelEnforced: false`. No changed filter, ptrace, PID namespace, cgroup or
canonical path has been activated by these tests. Earlier successful transport
and fixture runs remain exact historical evidence, not proof of this adapter.

The proposed monitor starts outside a new PID namespace, puts the trusted guest
inside its exclusive cgroup before releasing the fork barrier, then creates an
unnamed loopback-only network namespace and drops groups, saved credentials and
all capability sets. It requires already-enabled pids/memory/cpu controllers;
it never enables parent controllers or installs a host service. Controller
requirements follow the [cgroupv2 interface](https://docs.kernel.org/admin-guide/cgroup-v2.html). Limits are
pids.max32 including TIDs, memory256MiB with no swap, and one CPU core. Missing
delegation, controllers, kernel methods, exact runtime/libraries or custody
refuse. Payloads receive no cgroup-control write, namespace escape, raw ptrace,
CLONE_UNTRACED or inherited outside network socket.

A root `PTRACE_SEIZE` owner follows fork/vfork/clone/exec/exit with EXITKILL.
At an exec stop it checks the committed executable device/inode/hash, argv,
environment and cwd, actual namespace/cgroup/credentials and live input custody
before resuming user entry. Process pidfds are opened while stopped, before
release. Only the finite driver → trusted Python native entry → exact owned
native sequence is admitted; a missing, duplicate, early, revoked, changed or
late native approval refuses. Unknown descendants and nonleader exec refuse.
Thread events count toward the same task budget. An exec-exit stop is not a
terminal wait. Ordinary signals do not grant approval. These are proposed uses of
[ptrace exec/event semantics](https://man7.org/linux/man-pages/man2/ptrace.2.html),
not observations of this backend.

This choice avoids treating a seccomp CONTINUE over mutable pathname/argv
memory as verified execution. An FD-number/AT_EMPTY_PATH scalar check is also
insufficient: [an absolute execveat pathname ignores dirfd](https://man7.org/linux/man-pages/man2/execveat.2.html). Exec stops verify the
loaded image, while captured launch-graph analysis must also show that every
allowed pre-exec child path is trusted runtime bootstrap. The prototype is not
a hostile-driver sandbox. A reachable vendor fork/exec/dynamic-code bypass, or
an unknown pre-exec child path, keeps that original invocation refused.

The driver receives inherited Landlock/seccomp before its Node entry. Native
entry adds its narrower policy before native Node entry. The root authenticates
entry requests using actual TCP socket ownership in that namespace/cgroup,
not a caller-supplied PID/token alone. Only the trusted guest inherits the root
control pipe. The native attempts an owned outside-file read, canonical fixture
write and connection to an independent host-loopback canary; an unattempted or
timed-out canary is not denial evidence. The same run must show the selected
protocol, permission behavior, endpoint stream, native/driver software waits,
actual root parent/adoptee waits, pidfd HUP for every process and empty cgroup.
Cleanup rechecks these separate records; a summary flag or empty cgroup alone
fails. Killing owned remaining tasks during independent disposal cannot repair
missing reaping proof.

The canonical helper's exact payload/read/write/cleanup primitives are reused.
Before mkdir or any creation effect, the host durably records creation in
progress with identity unknown. It binds the opened directory's actual identity
as soon as fstat supplies it, before chmod or file writes. Only an explicit
not-started state means no creation was attempted. An interrupted mkdir/open/stat
or identity-persistence gap remains unresolved even if the path appears absent;
cleanup never adopts an object just because it occupies the expected path.
Only a proved owned inode and exact known helper files may be removed. Unknown
identity, unexpected files, changed bytes, symlinks or ownership refuse. Cgroup
and fixture cleanup remain separate observations. The journal is retained on
unresolved creation, interruption, partial cleanup or unproved tree settlement;
successful disposal cannot manufacture missing evidence.

## Exact inactive job and next gates

Tom decision3ca51d63 v2 (2026-11-14) authorizes preparation and ordinary reviewed
registration of the exact donor f9025d219 packet. The [workflow patch](claude-agent-sdk-combined-workflow.patch)
and [byte-bound routing patch](claude-agent-sdk-combined-routing.patch) are now
applied together in PR553, preserving workflow SHA2566dfe800d3e3e1c950e755af38a29e2728be343b941bdf3a7e6112ee7e5e0ce29
and prepared helper4cba13f2bc52d0cfa9165c70eb758cdf8d79d2827cd8981d590310e3c422c925.
Source tests reverse/reapply the exact routing delta only in an owned scratch
fixture, verify both identities and reject changed workflow bytes or an unrelated
exception. The earlier namespace and administrator workflows remain unchanged. The job
contains three fresh Linux jobs (absent, disabled, managed-deny), manual-only,
contents:read, five minutes each, fail-fast disabled and no retry. It uses an
**already cached** exact Node22.23.2; there is no setup-node, installation or
download fallback. Read-only preflight records actual Python/timeout/sudo/Node,
libraries, Landlock ABI, pidfd API and existing controller delegation. A missing
cached runtime or enabled controller is an environment prerequisite, not a
reason to mutate the host. Its source-checked root setup is bounded by35s plus
2s kill grace, with a30s inner wall and independent5s checked cleanup. New
combined control frames remain16KiB; the acknowledged layout is64KiB, the
complete result is128KiB and the root
receipt256KiB, each bounded and sequence/length/SHA checked. These are new
proposal bounds, never substitutions in the old consumed grants.

The packet's runnable commands are in the applied workflow and the plan JSON.
Task237/PR556 merged atf3a134d47598b074de3f80776453f814a7905313 and is
reconciled into this parent. Independent GitHub registry/content reads identify
workflow418195892 at`.github/workflows/sdk234-combined-fake.yml`, state active,
with the exact approved6dfe800d workflow bytes and zero recorded
`workflow_dispatch` attempts at registration reconciliation. The later consumed
attempt is recorded below. The registry state is availability, not execution
authority or proof of host capability. The routing helper is byte-identical to
the approved4cba13f2 prepared helper; no exception or process semantics changed.
After a fresh exact-source one-attempt grant, the proposed dispatch names the
exact pushed parent source and current detached spawn record:

```sh
gh workflow run sdk234-combined-fake.yml --repo inflatable-cookie/swallowtail \
  --ref ns-d7afab60-9a93-4414-95e7-61608a703ab7 \
  -f source_sha=EXACT_APPROVED_SOURCE -f record_sha=EXACT_APPROVED_SPAWN_RECORD
```

This workflow is registered for preparation only. A separate exact
resulting-source activation grant is still required; it is not authorized to run. It selects only
owned fakes. Root ptrace/PID-namespace capability, exclusive pre-enabled cgroup
custody and the changed combined enforcement/administrator path remain
unobserved. The existing runner is a concrete candidate, not an admitted host.
No privileged attempt is requested just to replay the old incompatible topology.

Original execution additionally needs the exact original SDK/native artifacts
and every reachable selected launch/code edge closed under their frozen
identities, combined native administrator discovery and platform/runtime
custody, then separate exact source/platform/artifact/attempt authority through
a declared Nucleus source-immutable proof route. The accepted identity ledger
and Research480/482 provide artifact lineage/source-flow evidence, not this
complete admission. There is no original mode or command that can bypass these
gates. Darwin's shared-cache custody and combined boundary remain unresolved.
Q-009 stays open; no ACP/CLI proof transfer, platform widening or qualification
increase follows from this source/fake milestone.

ACP227 reports that its actual sleep group leader and checked ACP are siblings
under the Rust host. The reviewed acp-actor-shape stand-in uses a parent-child
graph and does not prove that composition. Task227 owns its private sibling
adapter; no shared actor/kernel/schema change or integration claim is made by
this exact workflow preparation.


## Consumed combined attempt and actual host refusal

Tom decision78e36a73-4254-42a2-b872-71235fbdf4f9 v2, answered2026-11-14
13:15:23.553Z, granted exactly one new three-job owned-fake dispatch at source
2ea832c64269f4adfcaf1436dcfd93afee06c271, record
cedc3216b416764ecfc56b84581e08a6be8f1ad4b5861073a4d3e8f7a4437d9c and plan
8cf7b25a8640ca8f49f07628be4d84b9d90d605e5948adce256ec40355b5bf2d. Immediate
independent local/remote/ref, all26 captured input, workflow/default/execution
byte, plan/routing and prior-attempt checks agreed. The recorded exact command
returned exit0 and [run43599057785](https://github.com/inflatable-cookie/swallowtail/actions/runs/43599057785).
Its final metadata independently confirms that source, workflow418195892,
workflow_dispatch, attempt1 and failure. The source/ref stayed frozen through
settlement; workflow history contains exactly one combined dispatch. The grant
is consumed. No retry, rerun or changed-source substitution is authorized.

| State / actual job | Preflight | Privileged probe | Independent cleanup step |
| --- | --- | --- | --- |
| [absent132459694239](https://github.com/inflatable-cookie/swallowtail/actions/runs/43599057785/job/132459694239) | exit1 | skipped | exit0, no-setup branch |
| [disabled132459693979](https://github.com/inflatable-cookie/swallowtail/actions/runs/43599057785/job/132459693979) | exit1 | skipped | exit0, no-setup branch |
| [managed-deny132459694198](https://github.com/inflatable-cookie/swallowtail/actions/runs/43599057785/job/132459694198) | exit1 | skipped | exit0, no-setup branch |

All jobs checked out the frozen source and passed the exact input step. Runner
setup reported Ubuntu26.04.1 / ubuntu-26.04; absent and managed-deny image
20261111.196.1, disabled image20261111.82.6. These are runner image observations,
not an actual kernel/runtime custody inventory. All failed `Existing runtime
and read-only combined preflight` before any inspect JSON or Python traceback
was emitted. The untraced shell block contains source/clean-worktree checks,
private mktemp creation and `test -x` for the cached exact Node before Python.
The logs do not identify which shell operation failed. Missing cached Node is
therefore a candidate, not a proved diagnosis; no runtime, cgroup or kernel
capability was admitted from this failure.

Each cleanup step printed `Read-only preflight refused; privileged setup was
not reached`. It did not invoke the root cleanup helper. This is a successful
no-setup branch, not fixtureCleanup:true, independent tree settlement or
post-cleanup filesystem inspection. No root fixture/cgroup/PID/netns/ptrace
setup or owned driver/native probe ran. No actor/exec/filter/route records,
software waits, pidfd/cgroup observations, canonical policy discovery or runtime
library attestation were produced. Full raw job logs, final run/job/history
metadata, exact dispatch receipt and input checks are retained as author
review evidence; no success is inferred from their absence.

The next actual gate is identifying the failing existing-host preflight check
and establishing an admitted host with the unchanged required cached exact
Node22.23.2, complete tool/runtime/library custody and pre-enabled exclusive
cgroup controllers plus the required kernel methods. A bounded diagnostic must
report the individual read-only gate outcome before any privileged entry; any
new job/source/workflow bytes need applicable review/preparation and a fresh
exact attempt grant. No installation, parent-controller mutation, provisioning,
policy weakening or replay follows from this refusal. All planned time/task/
memory/CPU/FD/output/storage/transfer and cleanup limits remain unchanged but
were not exercised by the skipped combined probe. Original vendor/launch-graph,
Darwin shared-cache and actual ACP sibling composition gates remain separate;
Q-009 and the qualification merge hold stay open.
