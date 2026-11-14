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
record, state/attempt and minimal privileged setup/cleanup authority. Keep its
patch unapplied. No local canonical write, cgroup provisioning or host update is
a rehearsal. Darwin needs separately admitted shared-cache/runtime custody and
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
scratch settings are not canonical administrator-path proof. Kernel, canonical
setup and original entrypoints remain unavailable in this prototype.

Planner design answer to questionc84fed65: no blanket USER_NOTIF requirement,
and no new trust ruling. Custody does not make admitted SDK/native code trusted.
Before original admission, every reachable spawn/exec/worker/dynamic-code/addon
edge must be mediated, prohibited or source-proven unreachable for the exact
invocation. An executable bypass of the wrapper keeps that invocation refused;
detecting it after target effects is failure. Cgroup task limits include threads
and must apply before untrusted code, with migration/controller/monitor escape
blocked. Empty membership is not reaping. The current graph and changed kernel
adapter are still incomplete; this software checkpoint is not a ready original
execution request. A seccomp-notification CONTINUE over mutable argv/path memory
would not itself close that gap.

Coordination: task227 supplies exact captured facade/loader evidence at
`c0d591438590f77f502650a47b44e96295cf23f3`, including
`crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-acp-guard-loader.mjs`
(digest512a2b607f94b29ff215ed2eb6028ab09bc6a06ee30e14b6d6c9f798d5f0c71a).
It is ACP-specific evidence, not SDK qualification or a complete native exec
mediator. Reuse its private invocation/event seam without changing public APIs,
borrowing authority or duplicating the cgroup/namespace implementation. ACP's
installed-compatible Node axis remains separate from this SDK target.
