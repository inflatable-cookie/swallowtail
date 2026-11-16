# SDK trusted-host bootstrap transaction

Status: source/security proposal; no host activation.

Tom's 2026-11-16 preserved T3-to-Paseo handoff keeps the SDK qualification
obligation, inherited242f repair and original PR553 as history. The continuation
uses task240 and its own branch/PR. All old execution grants remain consumed.
This file owns the new private transaction; the
[permission-only packet](claude-agent-sdk-runtime-permissions.md) is historical
and must not be registered or dispatched to demonstrate permission preparation.

## Private contract

One ordinary-uid invocation must perform this sequence:

1. Bind the independently provisioned host manifest, source record, supervisor,
   candidate descriptor, safe ancestry, loader/cache, aliases and transitive ELF
   graph. The candidate has not executed. A candidate report cannot provide any
   of these prerequisites.
2. Persist and fsync permission intent before the sole exact-leaf0777→0755
   transition. Hold the original descriptor and the prerequisite descriptors.
3. In that invocation, retain a live bootstrap through runtime observation and
   each prepared-probe/shipped-sidecar fake case. Persist entry intent before
   creating an observer; keep the actual Popen handle and wait it. Reported
   shared objects must equal the independently bound set after exact alias
   resolution. The three fixed cases are prepared probe, sidecar callback denial,
   and sidecar pre-callback managed denial/continued turn.
4. Return from the source-bound rehearsal only after its owned cleanup/waits.
   Require driver and native waits. These software observations do not establish
   kernel tree settlement, canonical settings discovery or ACP actor evidence.
5. Verify identity, bytes, prerequisites and known child settlement before
   restoring0777. Persist the checked restoration. Keep every receipt, including
   intent, pending writes and failures; never delete evidence to turn failure
   into success.

A crash between launch intent and a returned Popen handle is unknown, even if
no PID was recorded. An unknown wait/fake settlement, replacement, changed bytes,
unexpected mode, unsafe ancestry or changed loader input retains failure and
refuses rollback. PID disappearance, pidfd HUP, cgroup emptiness and host disposal
cannot clear that gate. Recovery reacquires independently verified inputs; it
never inherits a dead supervisor's custody. The independent recover-only command
refuses any journal with pending children/entry intent and never launches Node.

The rehearsal has an explicit in-memory resource owner, without a temporary
directory finalizer. If stopping or waiting raises, it revokes further launches
and retains the actual process handle, open capture, pidfd, endpoint and staged
fixture. The refusal carries that owner; the outer transaction retains it too.

Tom's convergence direction separates admission from settlement. The private
contract now reserves one native launch obligation before the driver spawn, and
activates it before releasing the driver's payload. At the existing approved
one-shot spawn edge, source-bound tracking records intent before calling spawn,
the actual returned child handle, and that handle's terminal wait. A synchronous
parent-exit record can prove that the edge was never attempted; an attempted
spawn with no returned handle or missing notification remains unknown. Neither
record grants permission to execute payload. The driver Popen call has its own
prior intent; an exception without its handle retains unknown launch custody.
Captured peer imports cannot reach the tracker; only the bootstrap and private
generated spawn facade hold that edge. A forged-ledger import refuses before effect.

The obligation progresses through pending, identity-bound, admitted, refused and
settled states independently of the successful admission list. Identity is held
before native source/custody/cancellation admission checks. A failed identity
check keeps the pending obligation; a reported PID never authorizes a signal.
When entry notification is absent, only the already-bound parent's exact one-shot
child handle and actual wait can settle it. No adopter path is implemented here.
The same predicate covers success, refusal, cancellation and outer transaction
errors. Ordinary refusal still precedes the owned payload's first effect marker.
Only a later cleanup attempt using those same handles may release the capture
and identity-bound fixture after the owned stop, driver wait and endpoint join
succeed, and every active launch obligation has its exact parent's `ChildProcess.exit`
observation. Cleanup checks the retained owned stderr bytes, including at most
64KiB of tail after the driver wait within the fixed1MiB output ceiling. A smaller
protocol fault threshold still refuses that protocol; it cannot discard the
cleanup wait observation. Matching requires native PID, registered parent,
source, one-shot launch binding and terminal exit status. A missing or conflicting wait retains custody
even when the driver exited successfully. This route implements no adopter wait
and never accepts a caller-supplied receipt as one. The cleanup retry cannot
launch another rehearsal or clear the outer
transaction's unknown-settlement receipt. Process exit never substitutes for an
actual wait, and this retention does not establish kernel tree reaping.
On a protocol refusal, stop the identity-bound native before asking its driver
to close, so that the actual parent remains alive to receive the exit status.
The private owned driver also retains its returned ChildProcess handle and, on
termination, stops and awaits that handle before requesting sidecar close. This
covers a child still checking its Python entry before host identity notification.
If that parent still exits without the wait, retention remains mandatory.

Read-only input batches check every held identity before and after acquisition,
and retain each member's byte, owner, mode and ancestry checks. A partial failure
revokes that capture. Batches are bounded to512 members and refuse after command
configuration or while an observer is active. Entry/permit checks remain complete;
no cross-invocation cache or deadline extension is used. This removes repeated
metadata scans during preparation while preserving the original10s capture start.

## Non-circular trust anchor

The first-party ELF reader parses held bytes without executing Node, `ldd`,
`readelf`, a loader or an external interpreter. It verifies ELF64 little-endian
x86_64 headers, PT_INTERP and every transitive DT_NEEDED edge, including the
loader's own dependencies. It refuses unsupported audit/filter/preload-style
dynamic inputs, NODEFLIB, ambiguous mappings, missing/unused graph objects and
nonempty RPATH/RUNPATH. Such a refusal needs a separately reviewed exact policy;
the reader never guesses a search path.

Parsing an ELF graph does **not** establish which objects glibc will select.
An independent host provisioner must supply the exact complete loader-resolution
manifest and bind its SHA-256 outside the candidate and this reader. This is the
remaining concrete host prerequisite. It must cover glibc cache/default search,
CPU/hwcaps search variants and the initial Python/interpreter/stdlib/extension/
shared-library/supervisor closure. Observing the already-running Python's modules
and maps checks containment within that provisioned set; it cannot attest its own
pre-entry authority. No `complete:true`, serialized runtime report or caller flag
substitutes for the independent binding.

The manifest's exact fields are:

| Field | Required binding |
| --- | --- |
| `path`, `sha256`, `bytes`, `version` | Exact candidate below; version is a hypothesis checked only after admission |
| `originalActivation` | `false` |
| `host` | Exact `system`, `machine`, kernel `release`, ordinary `uid`, canonical `python` executable, fresh Linux `bootId` and `loaderAuxv` tags16/23/26/29/30 when present; secure mode must be0 |
| `bootstrapInputs` | Four maps: canonical absolute path→SHA-256 for `libraries`, `loaderInputs`, `interpreters`, `supervisorInputs` |
| `resolutions` | Every candidate/library/interpreter object→exact DT_NEEDED name→canonical object path; empty maps required |
| `aliases` | Exact PT_INTERP and reported-library path→canonical held object; each symlink and directory identity is retained |
| `interpreterAliases` | Exact subset of `aliases` actually named by PT_INTERP |
| `lookup.directories` | Every independently identified loader/default/hwcaps search directory→sorted immediate entry names; at most32 directories/8192 entries each |
| `lookup.absent` | Exact absent search alternatives and preload paths, with safe existing parents; at most128 |
| `lookup.environment` | Exactly `PATH=/usr/bin:/bin`, `LANG=C`, `LC_ALL=C` |

The provisioner must independently prove search completeness, including absent
hwcaps alternatives; a self-authored snapshot of arbitrary directories is not
that proof. The reader retains their identities, contents/absence, cache bytes
and safe aliases through entry and restore. No permission repair is allowed on
an unsafe ancestor or library. Graph inputs total at most512MiB/128 ELF objects;
each file is at most256MiB. The manifest is at most1MiB.

## Exact proposed host and command

The [machine-readable proposal](claude-agent-sdk-bootstrap-session.json) fixes
Ubuntu26.04.1 image20261111.196.1, Linux x86_64, existing
`/opt/hostedtoolcache/node/22.23.4/x64/bin/node`,124902584 bytes,
SHA-256 `94efd062ed066c1a0b332e18249859c905e920eeada5df55d24c99b919629a90`.
The past kernel observation was7.0.0-1019-azure; the independent manifest must
bind the actual future host, never reuse the past observation as custody.
This candidate remains fake-only outside qualified Node22.23.2..22.23.3.

After independent source/security review and an exact new operator grant, the
provisioner supplies an already-existing private0700 receipt/scratch directory
and a0600 independently bound host manifest. No download, copying a runtime,
installation, chmod of other objects or workflow registration is part of this
command. In the exact approved checkout, use the proposal's command with
`receipt_root`, `host_manifest`, `host_manifest_sha256` and `source_record_sha256`
bound by that grant. These are explicit provisioner inputs, not values taken
from candidate output. All source dependencies are pinned by the detached
[source record](claude-agent-sdk-bootstrap-session.sha256); the grant must name
its digest and the resulting source commit.

One attempt is one90s primary invocation plus2s termination grace. Internal
permission/custody/restore deadline is80s; each of the three observations is5s
and each fake case10s. Receipt64KiB; observer output64KiB; each rehearsal has
1MiB output/storage bounds. Independent recovery is at most10s+2s. Timeout or
unknown cleanup fails and retains the journal. The host must separately provide
the existing finite filesystem/network/task/thread/CPU/memory/storage/FD kernel
boundary before candidate entry; this transaction builds no alternate sandbox
and attests none of those gates. No combined, canonical, namespace/controller,
original/vendor/provider or release authority is bundled here.

The command is executable source with a concrete input contract, not an active
job or a permission-only demonstration. Until the independent host manifest and
outer boundary are provisioned and exactly authorized, only the owned source/fake
tests may run. Darwin shared-cache closure remains outside this Linux proposal.
ACP239 must supply its actual Rust sibling topology; SDK source/fake credit does
not transfer. Qualification claims and the merge hold stay unchanged.

The [selected SDK startup absence probe](claude-agent-sdk-startup-lookup.md)
owns the source-discovered optional-builtin incompatibility and the exact shared
ACP migration candidate. It supplies no host or original-runtime authority.

## Covering proof

Run `check:claude-agent-sdk-bootstrap-session`,
`validate:claude-agent-sdk-bootstrap-session`,
`validate:claude-agent-sdk-source-packet`,
`check:format:claude-agent-sdk-bootstrap-session`, the inherited
`check:claude-agent-sdk-runtime-custody`,
`validate:claude-agent-sdk-runtime-custody`,
`check:format:claude-agent-sdk-runtime-custody`, and the affected spawn,
namespace, combined and diagnostic source/fake selectors. Run `qa:docs`,
`qa:routes` and the retired-concept selector for changed operational truth.
These prove source and owned-fake behavior only. Current-source independent
review must cover inherited242f and these changes; older3fa6 approval is history.
