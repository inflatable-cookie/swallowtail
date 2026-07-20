# 013 Interactive Session Access Policy

Status: active
Owner: Tom
Updated: 2026-07-20

## Purpose

Let an interactive harness session request bounded workspace mutation without
turning `writable` into a hidden bundle of filesystem, approval, network, or
host-placement authority.

## Core Rule

Every interactive session carries one explicit, preflight-bound session access
policy. Its dimensions are independent:

- working-resource access: `Read` or `ReadWrite`
- filesystem boundary: the one host-resolved working-resource lease
- provider approval posture: `Never` in the first implementation
- provider-side external network: `Denied` or separately host-approved
- external search: disabled or separately enabled under Contract 009
- unexpected provider-request handling: reject, or observe-and-stop for an
  explicitly declared provider extension

No boolean or preset may be the durable public contract. Named profiles may be
convenience constructors only when their expanded policy remains inspectable
and is checked exactly.

## Initial Profiles

### Read-Only Interactive

The existing Agent Chat profile expands to:

- `ResourceAccess::Read`
- working-resource-only filesystem visibility
- approval posture `Never`
- provider-side network denied
- external search disabled
- only declared tool callbacks accepted
- all other provider requests rejected safely

This remains the default. Adding bounded write support must not change its
preflight plan or Codex request shape.

### Bounded Workspace Task

The first task-execution profile expands to:

- `ResourceAccess::ReadWrite`
- exactly one working-resource lease as the writable root
- approval posture `Never`
- provider-side network denied
- external search disabled
- no consumer product-tool declarations
- Codex approval and user-input requests declared only as provider extensions
  that may be observed and stop the turn
- one explicit host-monotonic turn deadline

Observing an approval or user-input request does not grant authority, answer
the provider, or keep a provider session resumable. The consumer may persist a
waiting record after Swallowtail closes and joins the provider scope.

## Resource And Host Binding

The consumer supplies an opaque `WorkingResourceRef`. The execution host
resolves it with the exact access requested by the preflight plan. The driver
receives only the host-authorized lease representation.

For a path-oriented driver, the one materialized working resource may become
the provider working directory and sole writable root. A client path, ambient
current directory, secondary root, symlink escape, or provider-configured root
does not gain authority.

`ReadWrite` requires explicit driver support, a matching parameterized
capability in the immutable plan, and a host service able to resolve that
resource with `ResourceAccess::ReadWrite`. Any mismatch fails before process
start. Multiple writable roots require a later contract.

The host-service execution id, configured-instance execution host, preflight
plan, resource lease, session, and turn must agree. A local process service
labelled with a remote host id is not remote execution.

## Approval And Provider Requests

Approval posture and callback handling are separate:

- posture `Never` tells the harness not to request approval
- a provider request that still arrives is an observation, not authorization
- `observe-and-stop` is allowed only for declared, bounded, redacted provider
  extensions such as Codex approval or user input
- Swallowtail preserves provider request, session, and turn correlation
- Swallowtail does not execute an approval, prompt the operator, fabricate a
  response, or translate the request into a common tool call

Tool declarations and tool responses remain governed by Contract 012. A
task-session access policy cannot implicitly add tools.

## Network Boundary

Provider transport access and provider-side workspace network access are
different. Starting a local harness may use the configured provider route or
its delegated authentication. The task profile still sets provider-side
network access to denied and external search to disabled.

Proxy, certificate, credential, or provider configuration is host-owned and
does not enlarge the session policy. A driver must not inherit ambient settings
that weaken the selected network or filesystem posture.

## Deadline, Outcome, And Cleanup

- session open and turn execution may carry separate host-monotonic deadlines
- only a host deadline observation produces a timed-out terminal result
- observed approval or user-input requests remain distinct from completion,
  cancellation, timeout, and failure
- every terminal or observed-stop path abandons pending callbacks, closes the
  provider session, joins reader/process work, and releases operation-scoped
  leases
- provider completion plus failed cleanup is not clean completion
- drop remains best-effort only; awaited close owns the result

## Provider Mapping

For Codex app-server, the initial exact mapping is:

| Policy dimension | Read-only interactive | Bounded workspace task |
| --- | --- | --- |
| thread sandbox | `read-only` | `workspace-write` |
| turn sandbox type | `readOnly` | `workspaceWrite` |
| writable roots | none | resolved working-resource lease only |
| provider network | denied | denied |
| approval policy | `never` | `never` |
| undeclared requests | reject | reject except declared observe-and-stop extensions |

Provider-specific fields stay inside the adapter. Common policy remains
provider-neutral.

## Identity And Redaction

Access policy, request, session, turn, callback, provider session, provider
turn, execution-host, and resource-lease identities remain distinct. Consumer
task, work-item, mandate, review, and receipt ids do not enter Swallowtail
contracts.

Public events and default diagnostics never contain raw paths, prompts,
provider envelopes, tool bodies, approval bodies, user-input bodies, stdout,
stderr, or credentials.

## Acceptance

- read-only sessions retain their existing exact request shape
- write access is explicit in policy, capability requirements, and resource
  resolution
- one authorized resource is the only writable root
- approval, filesystem, provider network, search, callback, and deadline
  dimensions cannot imply one another
- unsupported or mismatched dimensions fail before provider work
- observed provider requests stop safely without consumer authority
- every outcome exposes joined cleanup separately
- local services cannot execute a remote-authoritative resource
