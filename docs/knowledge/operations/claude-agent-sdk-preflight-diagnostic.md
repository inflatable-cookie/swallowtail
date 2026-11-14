# Read-only combined-host preflight diagnostic

Private source/fake preparation after consumed run43599057785. The observed
three exit1 preflights do not identify their failing shell check. No new
attempt, workflow application or host admission is authorized.

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

The exact unapplied manual diagnostic workflow and companion narrow routing
patch are retained here for separate source/security disposition and exact
preparation/activation choices. They propose one read-only Linux job with no
privileged/original branch or installation. No workflow is applied, registered
or dispatched by this packet. Existing combined/namespace/fixture workflow and
admission bytes remain unchanged; all their execution grants are spent.
