# Nucleus Task Execution Handoff

Status: prepared
Owner: Nucleus downstream
Source milestone: 010 Bounded Workspace Session Access
Updated: 2026-07-20

## Shared Runtime Seam

Swallowtail now exposes the provider-neutral records and Codex helpers needed
for one bounded writable task session. Nucleus remains the owner of task and
Goal admission, prompts, ordering, durable linkage, review evidence, receipts,
waiting state, recovery, and UI projection.

The first downstream slice uses:

- `codex_bounded_workspace_access_policy()` for the expanded session policy
- `codex_bounded_workspace_capability()` in the configured instance, model
  route, and operation requirements
- `HostServiceKind::WorkingResource` in the preflight requirements
- both namespaces from `codex_provider_request_extensions()` in the operation
  requirements
- `OpenSessionRequest::with_access_policy` with no product tool declarations
- one host-monotonic deadline on the task turn

The policy keeps read/write access, filesystem representation, approval,
provider network, external search, and provider-request handling independently
inspectable. The normal read-only session constructor and Codex JSON shape are
unchanged.

## Host And Resource Mapping

Nucleus selects one authoritative execution host and one opaque working
resource before Swallowtail preflight. The matching host service resolves that
reference as `ReadWrite` plus `Filesystem`. The returned lease supplies the
only provider writable root and retains consumer cleanup authority.

A missing resource, mismatched host id, missing write capability, missing
working-resource service, different lease reference, read-only lease, or
non-filesystem lease fails before Codex process start. A local service set
cannot execute a remote-authoritative plan merely by receiving its host label.

## Codex Mapping

The adapter maps the bounded policy exactly:

| Boundary | Mapping |
| --- | --- |
| thread sandbox | `workspace-write` |
| thread working root | host-authorized root as `cwd` and the sole `runtimeWorkspaceRoots` entry |
| turn sandbox | `workspaceWrite` |
| turn writable roots | exactly one host-authorized root |
| provider network | `networkAccess: false` |
| ambient temporary roots | excluded from the writable sandbox |
| approval | `never` |
| search | disabled by policy and unsupported for this profile |
| common tools | none |

No downstream request accepts a raw path or a secondary writable root.

## Provider Requests And Outcomes

Codex command, file-change, and permission approvals normalize to the declared
approval extension. Codex tool user-input requests normalize to the declared
user-input extension. Each observation retains distinct runtime callback,
runtime turn, provider request, extension namespace, event sequence, and
deadline correlation.

An observed request is not answered or authorized. The adapter returns an
explicit provider error, interrupts the turn, emits the bounded callback
observation, and terminates with `ProviderRequestObserved`. Unknown,
undeclared, malformed, or mismatched requests remain runtime failures.

The downstream projection should preserve these meanings:

| Swallowtail result | Consumer meaning |
| --- | --- |
| completed plus clean/not-applicable cleanup | task execution completed |
| approval observation plus joined cleanup | waiting for approval |
| user-input observation plus joined cleanup | waiting for user input |
| cancelled plus joined cleanup | operator cancellation |
| provider, host, or runtime failure with certain cleanup | failed execution |
| timeout or uncertain/degraded/failed cleanup | recovery required |

The waiting projections describe durable consumer state after provider
cleanup. They do not claim the Codex process or turn remains resumable.

## Identity Rule

Consumer task, Goal, mandate, work-item, review, and receipt ids stay outside
Swallowtail. Runtime request, session, turn, callback, provider session,
provider turn, provider request, configured instance, model route, execution
host, resource reference, and resource lease identities remain distinct.

Started consumer linkage must be durable before waiting for the provider
terminal outcome. Swallowtail exposes provider references and cleanup facts;
it does not decide how the consumer persists or reviews them.

## Downstream Validation

- one local bounded task reaches review-ready with durable diff evidence
- one ordered two-task Goal reaches review-ready
- approval and user-input observations produce distinct durable waiting states
- cancellation, timeout, provider failure, host mismatch, and cleanup failure
  retain separate outcomes
- resource-free and remote-authoritative execution fail closed until Nucleus
  supplies the required host route
- Agent Chat remains read-only and retains declared chat tools
- no direct Codex JSON-RPC remains in the product task executor after the
  Swallowtail path passes parity
