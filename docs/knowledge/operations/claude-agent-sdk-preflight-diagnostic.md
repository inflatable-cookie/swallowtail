# Read-only combined-host preflight diagnostic

Private source/fake preparation after consumed run43599057785. The observed
three exit1 preflights do not identify their failing shell check. No new
attempt or host admission is authorized. The exact workflow preparation ruling
is recorded below.

## Testable contract

Emit bounded, sanitized JSON lines for source identity, clean worktree,
private scratch creation, required cached executable, inventory production
and read-only host checks. Flush each completed observation before advancing.
On refusal, preserve those observations, label later phases skipped and exit
nonzero. Never print environment values, dirty filenames, command output or
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
