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
Normal close can race that signal path. Every private driver exit request now
synchronously quiesces future SDK spawns and awaits the same actual child before
calling the retained immediate-exit primitive. A sidecar survivor response stays
a failure; the later supervisor stop/wait supplies cleanup evidence only. A
failed wait leaves the parent under the existing host deadline and retains
unknown custody. No deadline or admission bound is enlarged.
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
and safe aliases through entry and restore. Ordinary custody cannot repair an
unsafe ancestor or library. The separate five-object administrative exception
below must finish before this ordinary reader opens the candidate. Graph inputs total at most512MiB/128 ELF objects;
each file is at most256MiB. The manifest is at most1MiB.

## GitHub host-package producer

Tom said "Go for it" on2026-11-16 and chose the GitHub Linux VM route in
decision0dfce67e-1c14-427e-be51-365db853eb87 v2. Tom owns the resource; task240
owns the repository provisioning stage. The old manual-manifest/resource-choice
request is superseded. Do not ask Tom to assemble file hashes or choose a route
again. This ruling authorizes source/owned-fixture preparation, not allocation,
workflow application/registration/dispatch, privileged setup, permission mutation,
candidate execution, installation or provider activity.

The trusted bootstrap principal will be the exact reviewed GitHub workflow/job
running its repository-owned administrative stage. Its operational identity must
be supplied by the later activation grant; no job or host is already admitted.
The initial GitHub image, system loader, root administrative tools and initial
system Python form the explicit finite trusted-host anchor. The producer binds
those exact image/package/source inputs before producing the ordinary-uid
manifest; candidate reports supply none of that authority. Complete loader/search,
Python/supervisor closure, safe ancestry and live pre-entry custody remain required.

The selected profile remains Ubuntu26.04.1/20261111.196.1 x86_64 and the exact
already-installed22.23.4 candidate below. There is no runtime install/copy/staging
fallback. The image's published build recipe sets `/opt/hostedtoolcache`0777;
that is a source-discovered safe-ancestry prerequisite, not a fresh host-mode
observation. The producer must refuse unsafe actual ancestry before opening the
candidate or executing any candidate command. Leaf preparation cannot repair it.
A narrowly scoped setup alternative must be reviewed before any repair attempt;
no broad recursive cache chmod or weakened ancestry rule is authorized.

### Five-ancestor administrative transaction

Planner disposition030bc11f, recorded in version8, approves the exact five-object
design for source/owned-fixture implementation and independent review. This
supersedes the gate-only stop at a3941b5e. It grants no host mutation or activation.
The administrative exception covers only the literal cache root, node,22.23.4,
x64 and bin directories; `/` and `/opt` must already be safe. Only0777→0755 is
prepared;0755 is held unchanged. Every other mode/type/owner/alias refuses.
Ordinary Inputs/Lease checks remain strict.

The same administrative process holds noninheritable directory descriptors and
a durable journal through package production and the later ordinary invocation.
It persists identity and intent before each mutation, prepares outer-to-inner,
and completes safe ancestry before opening candidate bytes. Known no-entry
failures may conditionally restore checked identities. An attempted/possibly
completed spawn creates an unresolved obligation before calling Popen; missing
actual waits, drift and ambiguous effects retain owner, descriptors and evidence.
After exact settlement and leaf restore, ancestors restore in reverse order.
No root directory handle passes to the ordinary supervisor or its payload.

The [pinned image profile](claude-agent-sdk-host-package.json) binds the official
Ubuntu26.04.1/20261111.196.1 build source. Its0777 tool-cache recipe is the reason
for this narrow administrative preparation, not a waiver of safe ancestry or a
fresh host observation. The historical unapplied contract patch remains retained;
the implemented transaction follows this refined safe-chain-before-candidate rule.
Task240 owns complete package production and exact unapplied setup/activation
source. Tom supplies no manually assembled hashes. Root bootstrap trusts the
selected GitHub image, loader and administrative tooling; candidate output never
establishes pre-entry authority. Both qualification holds remain unchanged.

### Image-specific producer source

[The administrative owner](../../../scripts/claude-agent-sdk-host-transaction.py)
and [package producer](../../../scripts/claude-agent-sdk-host-package.py) implement
the refined contract. The root owner journals each operation, retains exact
post-effect identities and requires actual owned waits plus a live scope's
settlement before restore. Directory effects are fsynced before their receipts;
a failed directory sync leaves an ambiguous operation and retains custody.
Construction requires an independent live input checker before any journal or
mutation. The future root entry must supply its exact image/boot/source/tool
checker; a serialized binding is not that checker. Serialized wait fields cannot clear an unknown launch.
A new recovery process refuses all entry attempts and ambiguous operations; known
no-entry recovery independently rebinds inputs and directory identities. A live
owner may complete a receipt only for an effect it actually observed. Root
handles are noninheritable and remain subject to owner-process identity checks.

The producer uses the existing ELF reader. It parses the selected little-endian
glibc1.1 cache, accepts only x86_64 entries and unambiguous cache/default targets,
and obtains built-in search paths from the trusted system loader's `--help`
without naming a candidate program. All glibc-hwcaps directories must be absent;
preload, legacy search, unsupported dynamic inputs and ambiguous targets refuse.
Actual cache bytes, paths/aliases, search inventories/absence and fresh host/auxv
inputs are held. The initial loader/dpkg tooling belongs to the explicit GitHub
image trust anchor; it is not candidate-generated authority.

The private Python profile is isolated/no-site system3.14 with its exact three
system search locations. The producer walks selected first-party and transitive
stdlib literal imports, binds source and cache bytes, verifies selected bytecode
against source, and includes extension/interpreter ELF closures. The exact
cache tag is `cpython-314`, with no cache prefix or optimization override;
canonical cache lookups stay inside the selected image. Missing nested paths
bind the first absent component beneath a safe existing parent. Canonical
directory aliases use the same strict ordinary ancestry checks as file inputs. First-party
nonliteral import calls and sourceless modules refuse. The trusted initial
interpreter/import bootstrap and fixed UTF8/ASCII/latin1 codec profile remain
explicit; no original/vendor Python code or arbitrary third-party search is
admitted. Installed package/source versions and the dpkg database bind image
provenance, rather than a `complete:true` snapshot assertion. The cache layout
and x86_64 flag policy follow the glibc
[cache definitions](https://github.com/bminor/glibc/blob/glibc-2.42/sysdeps/generic/dl-cache.h)
and [architecture selection](https://github.com/bminor/glibc/blob/glibc-2.42/sysdeps/x86_64/dl-cache.h);
nonzero selected hwcaps and ambiguous targets still refuse.

All512-input,128-object,32-directory,128-absence,512MiB aggregate,256MiB/file and
1MiB manifest bounds remain unchanged. Tool output is bounded to16KiB and every
returned tool handle is stopped/waited on failure. Unknown tool or ordinary-child
waits retain both administrative and package inputs. Real image/tool behavior
and the ordinary invocation/activation bridge still need their exact job binding;
owned fixtures are not host admission or completed qualification. The current
owned-child scope has no descendants and supplies software proof only; it is
not an implementation or attestation of the required kernel/adoptee adapter.
The ordinary manifest-descriptor bridge, existing finite kernel monitor
composition and exact unapplied job packet remain separate source work before
this setup can be proposed for one activation.

### Verification commands for a later grant

These commands are **unapplied**. As root custodian, first verify the independently
accepted source-record and manifest digests from the exact approved checkout:

```sh
printf '%s  %s\n' "$source_record_sha256" docs/knowledge/operations/claude-agent-sdk-bootstrap-session.sha256 | /usr/bin/sha256sum --check --strict -
/usr/bin/sha256sum --check --strict docs/knowledge/operations/claude-agent-sdk-bootstrap-session.sha256
printf '%s  %s\n' "$host_manifest_sha256" "$host_manifest" | /usr/bin/sha256sum --check --strict -
```

The manifest must be authored and its separate policy/provenance digest accepted
before these checks. Hashing an unapproved file grants nothing. Then, as the
bound ordinary uid with its already-approved initial Python, use the existing
source packet and reader in this bounded read-only invocation:

```sh
(
set +e
set -C
umask 077
read_only_output="$receipt_root/host-verifier.log"
if [ -e "$read_only_output" ] || [ -L "$read_only_output" ]; then exit 1; fi
env -i PATH=/usr/bin:/bin LANG=C LC_ALL=C HOME="$receipt_root" TMPDIR="$receipt_root" TMP="$receipt_root" TEMP="$receipt_root" /usr/bin/prlimit --fsize=16384:16384 -- /usr/bin/timeout --signal=TERM --kill-after=2s 20s /usr/bin/python3 -I -B -S - "$host_manifest" "$host_manifest_sha256" "$source_record_sha256" <<'PY' >"$read_only_output" 2>&1
import hashlib, json, resource, runpy, sys
assert resource.getrlimit(resource.RLIMIT_FSIZE) == (16384, 16384)
s = runpy.run_path("scripts/claude-agent-sdk-bootstrap-session.py")
sources = s["source_packet"](s["RECORD"], sys.argv[3])
with open(sys.argv[1], "rb") as stream:
    raw = stream.read(1048577)
assert len(raw) <= 1048576 and hashlib.sha256(raw).hexdigest() == sys.argv[2]
m = json.loads(raw)
assert (m["path"], m["sha256"], m["bytes"], m["version"]) == (s["CANDIDATE"], s["CANDIDATE_SHA"], 124902584, "v22.23.4")
assert m["originalActivation"] is False
assert (m["host"]["system"], m["host"]["machine"]) == ("Linux", "x86_64")
c = s["load"]("scripts/claude-agent-sdk-probe-custody.py", sources["scripts/claude-agent-sdk-probe-custody.py"])
e = s["load"]("scripts/claude-agent-sdk-bootstrap-elf.py", sources["scripts/claude-agent-sdk-bootstrap-elf.py"])
h = s["HostInputs"](c, e, m)
try:
    for name, data in sources.items():
        h.capture.bind(str(s["ROOT"] / name), hashlib.sha256(data).hexdigest())
    h.capture.bind(str(s["RECORD"]), sys.argv[3])
    h.check()
    print('{"hostInputsVerified":true,"runtimeExecuted":false}')
finally:
    h.close()
PY
verifier_exit=$?
[ "$verifier_exit" -eq 0 ] && [ "$(/usr/bin/wc -c < "$read_only_output")" -lt 16384 ]
)
```

The already-installed, independently bound `prlimit`, `timeout`, shell and `wc`
are additional custodian bootstrap inputs. The [util-linux resource limiter](https://man7.org/linux/man-pages/man1/prlimit.1.html)
sets both soft and hard RLIMIT_FSIZE to16384 bytes before timeout and Python
entry. Stdout and stderr share one exclusively created regular output file, so
the kernel bounds their combined captured bytes, including interpreter failures.
The verifier reads back both limits before input inspection. A full16KiB file
refuses even if a partial write or caught EFBIG lets the child exit0; truncation
is never accepted as success. An explicit existing-path/symlink check refuses any prior output, including a
FIFO or dangling link; noclobber additionally guards regular-file creation. The
custodian keeps the private allocation quiescent, and umask077 creates0600
capture without starting the verifier against any existing output. The custodian records the actual command/wait status and
retains the file on every failure; it does not append diagnostics past its cap.

This performs no permission transition, candidate execution or rehearsal. One
read-only attempt is20s plus2s kill grace, at most1MiB manifest and16KiB combined
captured verifier output. Retain failures; no automatic retry/install/repair. Its output checks the
custodian's inputs, never their authority. The primary invocation must reacquire
and hold its own live descriptors; this check cannot stand in for custody.
Source-only checks must compile this embedded Python and validate the surrounding
shell syntax without executing either command on a host. Owned child fixtures
exercise the inherited file-size limit, combined stdout/stderr saturation and
existing-file refusal. They model the limiter with Python resource.setrlimit;
they do not prove the future Linux prlimit binary, timeout or host admission.

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

The command is a conditional source template, not an executable host packet
until the repository producer supplies the independently bound package. It is not an active
job or a permission-only demonstration. Until the independent host manifest and
outer boundary are provisioned and exactly authorized, only the owned source/fake
tests may run. Darwin shared-cache closure remains outside this Linux proposal.
The ordinary supervisor owns the journal, live capture and actual child handles
until checked settlement and restore. After supervisor loss, the named custodian
owns retained journals/fixtures and independent recover-only execution (10s+2s);
recovery never launches Node or accepts an unresolved entry/child intent as a
wait. No old fixture or missing historical wait is adopted.

The custodian must provide the reviewed32-task/thread,256MiB, one-CPU
`100000 100000` quota,10 CPU seconds/process,64 payload-FD and1MiB per-rehearsal
output/writable bounds before candidate entry. Kernel parent/adoptee waits remain
required; cgroup emptiness and VM disposal cannot supply them. Task239 owns the
actual Rust parent with LocalProcessHost group-leader/ACP siblings and native
child. Its exact software integration must later compose inside this same finite
host/kernel boundary; the SDK stand-in proves none of that topology. Do not
duplicate239 integration or activate an extra job. Full original permission,
continued-turn/lifecycle and Q-009 remain open under the qualification merge hold.

The next authority must name the real host/boot, custodian, source/record,
manifest/policy digests, private allocation, boundary readbacks and one exact
permission/runtime/owned-fake/restore attempt. No chmod-only demonstration,
workflow edit/application/registration/dispatch, new host, runtime install/staging,
controller/canonical setup, original/native/provider/spend or release action is
authorized here. `originalCommand` and original admission stay closed.

The [selected SDK startup absence probe](claude-agent-sdk-startup-lookup.md)
owns the source-discovered optional-builtin incompatibility and the exact shared
ACP migration candidate. It supplies no host or original-runtime authority.

## Accepted ACP integration input

ACP239 PR558 is independently accepted at
`3cd84aec7a4a1742d6a08966969af11a4b061086`, review5920577693. This supersedes the
old pending5923 status. At the accepted commit, the integration JSON remains3541
bytes, SHA-256 `38eac917e03e8b0888767e5edc9005e44f0647188d8deba0c9d56ce426411634`;
the explanation is7025 bytes, SHA-256
`7383cfb79c12a3f493c38a55da1e628a809be29f122d8a9b015f54fac81e848d`.
Read the exact Rust captured prefix/recipe and ACP loader/shim/FD/runtime bindings
from that receipt. Rust parents the sleep group owner and checked ACP as siblings;
ACP joins sleep's group and parents native entry. Reuse this accepted source for
future exact composition without repeating unchanged accepted software tests.
Software waits and descriptor EOF do not prove kernel/adoptee/tree settlement.
Original option tails/callsite provenance, Python/non-system runtime/opaque-native
closure and original-compatible combined fixtures remain gates. Both qualification
holds and every original/provider/host/kernel/canonical activation gate remain.

## Covering proof

For producer changes run `check:claude-agent-sdk-host-producer`,
`check:format:claude-agent-sdk-host-producer`,
`validate:claude-agent-sdk-host-producer` and the source-packet/docs/routes/routing
checks below. Accepted unchanged runtime/ACP suites are not a ritual.

For admission changes run `check:claude-agent-sdk-host-admission`,
`check:format:claude-agent-sdk-host-admission`,
`validate:claude-agent-sdk-host-admission` and
`validate:claude-agent-sdk-source-packet`, plus the changed docs/routes/retired and
selector-routing checks. Do not repeat accepted unchanged runtime/ACP suites.

For transaction changes run `check:claude-agent-sdk-bootstrap-session`,
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
