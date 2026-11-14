# Read-only combined-host preflight diagnostic

Private source/fake preparation after consumed run43599057785. The observed
three exit1 preflights do not identify their failing shell check. No new
attempt or host admission is authorized. The exact workflow preparation ruling
is recorded below.

## Testable contract

Emit bounded, sanitized JSON lines for source identity, clean worktree,
private scratch creation, required cached executable, inventory production
and read-only host checks. Flush each completed observation before advancing.
On refusal, preserve those observations and exit nonzero. The v2 dependency
rules below determine which later phases are skipped. Never print environment values, dirty filenames, command output or
arbitrary exception text. No blanket tracing. Broken output is failure.

The inventory reads exact executable bytes and existing controller/namespace
identities; it never executes the cached Node, invokes sudo, installs tools,
creates a cgroup/namespace, attaches ptrace or writes canonical settings.
Report production and read-only check success are distinct from admission:
every result states hostAdmission:false, kernelEnforced:false and
originalActivation:false. Runtime/library custody, privileged capability,
actual fixture discovery and process-tree settlement remain unproved.

Use a20s outer timeout with2s kill grace, a10s internal deadline, bounded Git
output8KiB, bounded file reads/hashing and total JSON output32KiB. Only a fresh
private scratch leaf under the supplied run TMPDIR may be created and removed.
An absent cached executable or failed inventory/report must preserve a nonzero
refusal. Owned fixtures test these paths without needing actual hosted tools.

## Execution route

Existing CI runs Rust/contract/package checks, not this diagnostic. Its Node
setup jobs install the selected runtime before tests, so they cannot establish
the untouched cache availability that the failed combined job required. Do not
hide diagnostic activation in ordinary tests or change those workflow bytes.

Tom decision77371f79-2764-458c-90ac-e2787207421a v2, answered2026-11-14
16:07:34.530Z, authorizes exact preparation and separate ordinary reviewed
registration. The reviewed donor31cdc97a97b867e2d93c3fee18b964de389e718d and both
patch identities were verified before application in PR553: workflow patch
71f1cc92016f2cbd5ca25873aaf0ee5d69bbb17f0a7fb51bf606f8b63c3f9f09 and routing patch
4210c04ea1ccf7ef4f25e25a17f34233d92541b18af99488a56e25b4ae2b1bc3. Applied workflow
bytes remain9e5b668fc2e2d1f8760fb92e161a37fc4092364bca9d9c85029521159757490c;
prepared helper1b050616a676aedc7c75903218ff50349b68928fe50a143db9752ac03ad7d1dc.
Manual-only contents:read, one Ubuntu job, two minutes,20s+2s/10s/32KiB and
read-only/no-candidate-execution policy are unchanged.

The packet validator checks exact applied bytes and reverses/reapplies only in
owned fixtures. The combined source validator removes only this exact approved
routing delta in an owned fixture before its retained exact earlier-packet
checks; no process/filter/admission rule changes. Actual diagnostic workflow and
routing inputs join the separate captured diagnostic record; the parent uses
an acyclic reference to that record. No diagnostic is executed by validation.

Task238 /PR557 merged ateea290a5c2a151934027aa82c9718e3aff1bb2eb and is
reconciled normally into the parent. Independent registry/content reads confirm
active workflow418437987 at`.github/workflows/sdk234-preflight-diagnostic.yml`,
exact approved9e5b668f bytes and zero prior diagnostic workflow_dispatch
attempts. Main/parent routing bytes match the approved1b050616 helper exactly;
reconciliation added only the registration changelog. Registration is
availability, not host admission, diagnostic results or activation authority.

Return the exact resulting source/current diagnostic record and command for a
separate one-attempt activation decision. No dispatch is authorized by this
preparation/registration ruling. Existing combined/namespace/fixture workflows
and all runtime/admission rules remain unchanged; all historical execution
grants are spent. Q-009 and the qualification merge hold remain open. The exact
failed combined shell gate and all actual diagnostic host facts are unobserved.


## Consumed diagnostic attempt and current host gate

Tom decisiond600b741-16e6-42da-bd59-e1678161fd2a v2, answered2026-11-14
16:50:54.547Z, granted exactly one read-only diagnostic dispatch. Immediate
independent local/remote/source, all9 captured input, proposal/helper,
registration/default/execution workflow bytes and zero-attempt checks agreed.
The exact command returned exit0 and [run43618052014](https://github.com/inflatable-cookie/swallowtail/actions/runs/43618052014).
Final run/job/history reads confirm workflow418437987, workflow_dispatch,
attempt1 and exactly one diagnostic dispatch, at frozen source
0967f07d11950178b6265c3437d161728163119b /record
4fa4a28b09cfcc04ae757bd7b638d635a2070636cf1001a7521c983c6cd4d5b4 /proposal
33d18a171aaa5c5ed85d8038f22d5b196c4136750889445cefe97ba6925d441e. The ref stayed
immutable through settlement. Grantd600b741 v2 is consumed; no retry is granted.

[Job132517351658](https://github.com/inflatable-cookie/swallowtail/actions/runs/43618052014/job/132517351658)
exited1. Complete helper output is1286 bytes, seven records within the32KiB
bound, plus the input/launcher/exit envelope records:

| Phase | Actual observation |
| --- | --- |
| source_identity | observed exact source and diagnostic record |
| clean_worktree | observed clean:true |
| scratch_creation | observed createdPrivateLeaf:true and removedOwnLeaf:true |
| cached_executable | refused, reason cached_executable_unavailable |
| inventory_production | skipped |
| host_capabilities | skipped |

The summary names failedPhase:cached_executable and reportProduced:false
(inventory), readOnlyChecksPassed:false, hostAdmission:false,
kernelEnforced:false and originalActivation:false. The required path
`/opt/hostedtoolcache/node/22.23.2/x64/bin/node` failed the predicate that it is
both a file and executable. The diagnostic does not distinguish missing,
non-file or non-executable states. No cached Node was executed or hashed; no
inventory/current Python/kernel/runtime-library or controller/delegation/
privileged capability report was produced. Runner setup reported
Ubuntu26.04.1 /ubuntu-26.04 image20261111.196.1; this is image metadata only.
The successful private-leaf removal is not canonical fixture cleanup,
independent process-tree settlement or post-cleanup filesystem inspection.

This supplies an actionable current-host cache-availability gate, not an
admitted shared host or a retrospective explanation of the unlabelled
43599057785 failure. There is no substantive admitted-host milestone to release
ACP227. Require an authorized existing host/environment that provides the exact
cached Node22.23.2 executable and then proves the remaining tool/runtime/library
custody, pre-enabled controllers and required kernel capabilities. No runtime
substitution, installation/staging, provisioning, controller enablement or
weakened admission is authorized here. Any further diagnostic or combined job
requires fresh exact source/record/attempt authority; do not replay this grant.
All planned limits are unchanged, and no timeout or privileged branch ran.
The consumed combined grant, original/vendor/Darwin/ACP gates, Q-009 and the
qualification merge hold remain open or spent exactly as previously recorded.

## Independent read-only phases and no-install comparison

Private v2 contract, specified before implementation: source identity, clean
worktree and owned scratch are guards. A guard refusal skips all host reads.
After those guards, the required 22.23.2 predicate controls only its dependent
runtime inventory. Its refusal remains the first failure and forces exit 1.
Independent fixed-path cache metadata, host identity, controller, namespace and
kernel-method observations continue, including after another independent phase
refuses. The summary retains every failed phase, emitted report names and a
separate runtime-inventory production flag. Report production never supplies
host admission; all records keep admission, enforcement and original activation
false. Deadline and output failure still refuse rather than continuing work
beyond their bounds. Error strings and unrelated environment/user files are
never emitted. The registered workflow and CLI remain unchanged.

Only four fixed tool-cache paths are inspected: 22.23.2 (required pin), 22.23.3
(already-qualified axis, availability unobserved), 22.23.4 (owned-fake candidate)
and 24.22.2 (published image default, metadata only). Path labels are not observed
versions. Available executable files may be hashed, never run; missing paths,
symlinks and read failures are labelled without fallback. No directory scan,
runtime installation, staging or controller mutation is part of this contract.


The job's [published image document](https://github.com/actions/runner-images/blob/ubuntu26/20261111.196/images/ubuntu/Ubuntu2604-Readme.md)
lists cached22.23.4 and24.22.2, default24.22.2. This is an exact image-metadata
lead, not observed cache bytes, runtime version, library custody or a current
host. It cannot explain the exact old combined shell failure. The consumed
0967 diagnostic and its v1 output remain historical evidence. The exact v2
attempt and its independently retained observations are recorded below.

| No-install option | Current evidence | Next gate |
| --- | --- | --- |
| Existing22.23.2 host | fixed path not found on43625464586 | another existing exact host must establish executable bytes and complete custody; no host is currently admitted |
| Existing22.23.3 host | within qualified axis; fixed path not found on43625464586 | another existing host must establish exact runtime/startup/library and host admission proof |
| Cached22.23.4 | file hash/size/mode observed on43625464586; version/custody unproved |0777 mode fails current custody rule; exact permission/custody preparation before any separately authorized fake execution |
| Image default24.22.2 | file hash/size/mode observed; version/custody unproved | metadata only; no automatic selection or SDK/ACP authority transfer |

The v2 packet consumed below used registered workflow418437987, unchanged bytes
9e5b668f, one2min job/20s+2s outer/10s inner/32KiB. It bound its exact
source and detached diagnostic record through the existing source_sha/record_sha
inputs. The packet reads all four fixed paths and independent public host
facts even when the required22.23.2 predicate refuses. Both read-only grants are now spent;
there is no further dispatch authority. No combined retry or
candidate runtime execution is hidden in it.

If22.23.4 is present, the following is the finite **future owned-fake** option,
not an enabled command: freeze its exact absolute path, observed bytes/digest
and file identity, independently establish actual versionv22.23.4 under the
existing explicit STARTUP policy, and capture all loaded libraries/loader,
Python supervisor and source inputs. Host-owned custody must remain valid
through launch and refuse substitutions/revocation. Hashes from this diagnostic
alone cannot attest that custody or the runtime's actual version. The existing
private spawn rehearsal accepts explicit --node and --node-sha256 together
with its detached spawn record, --rehearse, --state and --mode; it can exercise
owned SDK/native peers through the prepared driver and unchanged sidecar after
separate exact execution authority. Bound each selected absent/disabled/
managed-deny × sidecar/probe case by its existing30s maximum, finite output/
storage and observed child joins. Do not execute the runtime merely to complete
this source comparison. Darwin26 first-party rehearsals supply context only.

Before this candidate could carry **original SDK/native evidence**, it needs a
separate22.23.4 Node-axis qualification: exact official artifact/platform
identity and provenance, selected startup/loader/library compatibility and
regressions against the retained22.23.2/.3 baseline, plus route-specific proof
and reviewed admission changes preserving older qualified points. The frozen
SDK/native package/platform identities, combined canonical administrator
fixture discovery, native-usable confined transport, finite actor/thread/
resource enforcement and full software/adoptee/pidfd/cgroup settlement still
need actual exact-host proof and fresh authority. The current combined packet
continues to require22.23.2; this proposal does not weaken or replace its pin.
Original activation and the public Node22.23.2..=22.23.3 claim remain unchanged.
Darwin shared-cache custody and actual ACP Rust sibling composition remain
separate gaps. A read-only result alone does not resume ACP227.


## Consumed revised diagnostic and concrete custody gate

Tom decision7f7c7a49-5d69-4b24-a5ab-88f84443e324 v2, answered2026-11-14
18:11:50.156Z, granted one revised read-only dispatch at immutable
3fa6b24127d2f67dfebc65ed41646957b4888747 /record
c240e5cb22f6137c384615c5fdec949eaf5911d99796e26e2e8e41a1444f8d3e /proposal
89ffddbf0c3bd73445602d79bbb45b1a1348c6ee31ae55c4e1d973d511579594.
Immediately rechecked local/remote source, all9 captured inputs, exact workflow/
helper/default/execution bytes, active418437987 and exactly one old0967 run.
The exact command returned0 and [run43625464586](https://github.com/inflatable-cookie/swallowtail/actions/runs/43625464586).
Independent terminal run/job/history reads confirm attempt1,
[job132540185267](https://github.com/inflatable-cookie/swallowtail/actions/runs/43625464586/job/132540185267),
exactly one new dispatch, two total diagnostic dispatches and the ref frozen
through settlement. This grant and the oldd600 grant are spent; no rerun.

The job exited1 at cached_executable, with dependent inventory skipped.
Complete helper output is4199 bytes/11 records, plus three envelope records.
Source/clean worktree/private scratch and all five independent metadata phases
were observed. readOnlyChecksPassed:false, reportProduced:false (runtime),
independentHostReportProduced:true and failedPhases:[cached_executable] preserve
the failed overall result. Every helper record keeps hostAdmission,
kernelEnforced and originalActivation false. No timeout or runtime execution.

| Observation | Exact scope |
| --- | --- |
| Cache22.23.2/.3 | path_not_found for each fixed x64/bin/node path on this fresh job; not a claim about other hosts or old43599057785 |
| Cache22.23.4 |124902584 bytes, SHA25694efd062ed066c1a0b332e18249859c905e920eeada5df55d24c99b919629a90, device2049/inode311247, owner1001, mode0777, executable access; path label only, versionVerified:false/runtimeCustody:false/executed:false |
| Cache24.22.2 |126881248 bytes, SHA256904a6d3484241d466989f9df9ba3aab822b1496191bdb28230eb491a55c32a52, device2049/inode325308, owner1001/mode0777; metadata only |
| Actual host | Linux x86_64, kernel7.0.0-1019-azure; job reports Ubuntu26.04.1/image20261111.196.1 |
| Cgroup parent | device30/inode1, root-owned/mode0555; cpu/memory/pids available and enabled; exclusiveDelegationAttested:false |
| Namespaces | device5; cgroup4026531835/net4026531833/pid4026531836/user4026531837; no creation tested |
| Public kernel metadata | ptrace_scope1, max_user_namespaces58121, eight reported seccomp actions and Python pidfd_open symbol; no kernel methods tested |

No runtime version, loaded-library/loader/Python custody, Landlock ABI,
committed-image ptrace gate, pidfd operation, exclusive delegated cgroup,
namespace enforcement, canonical fixture discovery or software/adoptee/tree
settlement was proved. Private scratch leaf removal is the only owned cleanup
observation. Metadata does not prove historical failure, original/native
behavior or actual ACP sibling composition.

The precise next no-install source/security packet should prepare custody of
`/opt/hostedtoolcache/node/22.23.4/x64/bin/node` at the observed94efd062 digest.
Its0777 mode currently fails `Lease.bind`'s existing0o6022 mask with
custody_unowned_or_writable_input. Do not run the current rehearsal to discover
that refusal: runtime_inventory executes Node before Lease creation. The next
source preparation must enforce eligibility and stable host-owned pre-entry
custody before any version/library observation, then specify narrowly bounded,
identity-checked **separately authorized** in-place permission preparation
(proposed0755, unchanged bytes, checked cleanup/retained failures). This run
changed no cache permissions and staged/installed/executed no runtime. Do not
weaken the settled custody contract or treat the byte hash as provenance,
verified version or an OS verified-exec guarantee.

Enabled parent controllers remove one unknown, not the exclusive delegation
or enforcement gates. A later exact host packet must still bind complete
runtime/tool/library/startup inputs and establish a bounded exclusive child
with32 TIDs/256MiB/no swap/one CPU before any separately authorized combined
fake run. No parent-controller enablement follows. Candidate execution,
permission repair and privileged setup each remain outside the spent grants.
Node22.23.4 original use still requires its separate Node-axis qualification;
the current combined22.23.2 pin and public22.23.2..=22.23.3 axis stay unchanged.
ACP227 remains parked pending actual shared actor/runtime/host admission;
Q-009 and the task-owner qualification merge hold remain open.
