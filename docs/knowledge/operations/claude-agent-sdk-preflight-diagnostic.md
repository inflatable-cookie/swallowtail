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

Canonical registration is solely task238 /02bb26b2-517c-427e-a0e5-4b49b0be44c7.
After it settles, reconcile canonical main normally, verify the registered
workflow bytes/identity and zero prior diagnostic attempts, then name the exact
resulting source/record/command for a separate one-attempt activation decision.
No dispatch is authorized by preparation. Existing combined/namespace/fixture
workflow and runtime admission bytes remain unchanged; all historical execution
grants are spent. Q-009 and the qualification merge hold remain open. The exact
failed combined shell gate and all actual diagnostic host facts are unobserved.
