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
and safe aliases through entry and restore. No permission repair is allowed on
an unsafe ancestor or library. Graph inputs total at most512MiB/128 ELF objects;
each file is at most256MiB. The manifest is at most1MiB.

## Provisioner and external resource handoff

The proposed accountable bootstrap principal is **Tom as host custodian**, or a
human delegate Tom names, acting through host root before the ordinary-uid
transaction starts. The SDK worker and candidate are not that principal.
GitHub Actions is the proposed infrastructure provider: its
[Linux VM runners support passwordless sudo](https://docs.github.com/en/actions/reference/runners/github-hosted-runners#administrative-privileges).
The past Ubuntu image observation demonstrates that host family existed, not
that an exact future image is reserved. No live host or independent manifest
producer has been assigned. GitHub's image label/software inventory is not a
complete loader/Python manifest.

The finite trust anchor is the custodian's explicit acceptance of the provider
image, its bootstrap tools and root provisioning process. Before ordinary
Python entry, the custodian must independently approve the exact initial Python,
loader, standard-library, extension, shared-library and supervisor closure.
That authority arrives through the authenticated operator/plan channel. Python's
later module/map inventory checks containment; it cannot supply pre-entry
authority. The custodian's authority is the root of trust, not another executable
whose report needs an endless chain of self-attestation. This proposes trusted
host administration within decision89db0f25, not hostile-same-user verification.
It preserves every reviewed completeness, search and live-custody check.

Recommend **A: one custodian-audited GitHub Linux VM** with a separately approved
root preparation phase and the already-installed exact candidate below. This
uses a real provider, but requires an actual job/VM allocation and audited policy.
Abort on image/candidate mismatch without install, runtime copying or other
permission repairs. **B: an existing Tom-administered Linux x86_64 host** can
supply the same package, but its actual image/kernel needs a separate exact host
proposal. Neither option is activated here. If neither owner can supply the
package, request a named provisioner first; another worker-generated inventory
is not a runnable third option.

The named custodian must supply this complete package for one host/boot/uid:

| Resource | Exact obligation |
| --- | --- |
| Host/principal record | Tom names the root custodian, provider job/VM, image/build source, boot ID, kernel, ordinary uid and CPU/auxv. Approve initial bootstrap-tool identities before ordinary Python starts. No same-user adversary claim. |
| Existing candidate and ancestry | Exact path/bytes/hash below, ordinary-uid owner,0777 leaf and already-safe ancestors/aliases. No Node invocation, including `--version`, establishes this input. |
| Independent loader policy | Audit the exact loader/libc package/build sources and configuration; complete cache/default/hwcaps search for the actual CPU/auxv; every directory inventory, absent alternative and preload path; aliases and justified selected targets for every transitive DT_NEEDED. Bind cache/config bytes. Refuse unsupported dynamic inputs and nonempty RPATH/RUNPATH. `ldd`, a directory snapshot or a completeness flag supplies no authority. |
| Initial supervisor policy | Canonical Python target, interpreter/DSOs, isolated/no-site stdlib/extensions, bytecode that may be read despite `-B`, import/search/absence policy, lazy imports and all first-party supervisor/timeout inputs. Post-startup modules/maps alone are insufficient. |
| Published artifacts | Existing0600 `host-manifest.json` with the schema above; custodian-authored loader/Python completeness/provenance record; exact source commit/record; detached manifest, policy and source-record digests delivered independently through Nucleus before ordinary verifier entry. Keep the inputs quiescent through settlement. |
| Existing private allocation | Safe canonical checkout, ordinary-uid0700 receipt/scratch root with initially absent `session.json`, and failure-retention space. Supply allocation identity; never adopt an unknown fixture. No runtime staging. |
| Outer boundary and wait owner | Already-enabled exclusive cgroupv2 delegation, PID namespace/init, loopback-only network, filesystem/filter/ptrace monitor and actual parent/adoptee wait owner, with exact source/config/readbacks and receipts. The transaction neither provisions nor proves these. |

There is **no approved executable provisioner** here that derives this image's
loader/search or initial Python authority. `HostInputs` consumes it and the ELF
reader checks its graph. The smallest missing component is the named custodian's
finite policy/manifest for this one image/candidate, backed by independent
package/build audit. No general ELF packager or sandbox is needed. If an exact
selected input cannot be justified, refuse and request that image/package record
from the custodian. Source/fake work cannot manufacture this trust input. Until
it is supplied and reviewed, this is an external-resource request, not a runnable
host packet.

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
until the named custodian supplies the independent package. It is not an active
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

## Pending ACP integration input

Version5 retains PR558 candidate source
`5923c113c9e4ee4be42c3345148b212b1d8733cf`, submitted to its own independent
review, not yet an accepted host/migration input. At that exact commit,
`docs/knowledge/operations/claude-agent-acp-startup-integration.json` is3541 bytes,
SHA-256 `38eac917e03e8b0888767e5edc9005e44f0647188d8deba0c9d56ce426411634`;
its `.md` explanation is5646 bytes, SHA-256
`96faa926e2a1ca8f02494816132a16e975a1d012439b55c6a94a367dfe6de4c5`.
Read exact Rust prefix/recipe, ACP loader/shim and FD/runtime identities from
that receipt after independent acceptance; short labels and historical
fingerprints are not bindings. The real Rust parent owns the sleep group leader
and checked ACP as siblings; ACP joins sleep's group and parents native entry.
Software waits/descriptor EOF remain separate from kernel/adoptee/tree waits.
Original option tails/callsite provenance, selected Python/non-system runtime/
opaque-native closure and original-compatible combined fixtures still refuse.
No counterpart worktree change, consumption, activation or qualification credit
is granted. The resource-owner question and all host/kernel/workflow holds remain.

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
