# 013 Interactive Session Access Policy

Status: active
Owner: Tom
Updated: 2026-07-21

## Purpose

Let an interactive harness session represent ambient or bounded execution
without turning `writable`, a working directory, or a sandbox preference into
a hidden bundle of filesystem, approval, network, or host-placement authority.

## Core Rule

Every interactive session carries one explicit, preflight-bound session access
policy. Its dimensions are independent:

- working-resource access through host services: `Read` or `ReadWrite`
- filesystem boundary: absent, or the one host-resolved working-resource lease
- harness isolation: `AmbientHost`, `ProviderEnforced`, or `HostEnforced`
- provider approval posture: `Never` in the first implementation
- provider-side external network: ambient, denied, or separately host-approved
- external search: disabled or separately enabled under Contract 009
- unexpected provider-request handling: reject, or observe-and-stop for an
  explicitly declared provider extension

No boolean or preset may be the durable public contract. Named profiles may be
convenience constructors only when their expanded policy remains inspectable
and is checked exactly.

Harness isolation applies only when a local harness process exists. Direct
hosted inference and remote services with no local harness process bind no
harness-isolation posture. Isolation is opt-in: `AmbientHost` is the generic
default, while either enforced posture requires exact driver or host support.
No posture may silently substitute for another.

## Initial Profiles

### Ambient Harness

The generic local-harness profile expands to:

- `ResourceAccess::Read` or `ReadWrite` for host-mediated working-resource
  services
- no filesystem boundary claim
- `HarnessIsolation::AmbientHost`
- ambient execution-host process and network authority
- explicit approval, search, and provider-request policy

The working resource may select a project location and scope ACP or other host
callbacks. It does not limit direct filesystem, shell, child-process, or
network access by the harness. The consumer owns whether this route is offered
or enabled and how its ambient authority is disclosed.

### Read-Only Interactive

The provider-enforced Agent Chat profile expands to:

- `ResourceAccess::Read`
- working-resource-only filesystem visibility
- `HarnessIsolation::ProviderEnforced`
- approval posture `Never`
- provider-side network denied
- external search disabled
- only declared tool callbacks accepted
- all other provider requests rejected safely

This remains the explicit Codex Agent Chat profile. Adding ambient or bounded
write support must not change its preflight plan or Codex request shape.

### Bounded Workspace Task

The first task-execution profile expands to:

- `ResourceAccess::ReadWrite`
- exactly one working-resource lease as the writable root
- `HarnessIsolation::ProviderEnforced` in the first Codex mapping
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

For a bounded path-oriented driver, the one materialized working resource may
become the provider working directory and sole writable root. A client path,
ambient current directory, secondary root, symlink escape, or provider-
configured root does not gain authority.

For `AmbientHost`, the same materialized value may select the working
directory, but it is location only. The harness and descendants retain the
execution host user's ambient authority outside it. The working-resource lease
still scopes host callbacks and cleanup; it does not manufacture containment.

Working-directory selection and client-mediated filesystem callbacks are not
process containment. Under Contract 017, a harness that can bypass the
callback through local filesystem or child-process operations needs an exact
provider or execution-host mechanism only before it claims a bounded profile.
The same harness may use an explicit ambient profile without that claim.

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
its delegated authentication. A bounded task profile may still set provider-
side network access to denied and external search to disabled.

Proxy, certificate, credential, or provider configuration is host-owned and
does not enlarge the session policy. A driver must not inherit ambient settings
that weaken an enforced network or filesystem posture. An `AmbientHost` route
instead records ambient network authority explicitly; it cannot claim denied
provider-tool network access merely because search or web tools were omitted.

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
| harness isolation | `ProviderEnforced` | `ProviderEnforced` |
| thread sandbox | `read-only` | `workspace-write` |
| turn sandbox type | `readOnly` | `workspaceWrite` |
| writable roots | none | resolved working-resource lease only |
| provider network | denied | denied |
| approval policy | `never` | `never` |
| undeclared requests | reject | reject except declared observe-and-stop extensions |

For the first attached OpenCode HTTP driver, only read-only interactive access
is supported:

- harness isolation is `AmbientHost`; Swallowtail does not own or sandbox the
  attached OpenCode server
- the host-resolved working-resource filesystem value becomes the explicit
  OpenCode `directory` query for session and event operations
- session creation sends ordered permission rules that deny `*`, then allow
  only `read`, `glob`, and `grep` for `*`
- shell, edit, task, skill, question, external-directory, web-fetch, web-search,
  and future permissions remain denied by the catch-all rule
- no consumer tool declarations or provider-request extensions are supported
- an unexpected permission or question event triggers native session abort and
  terminal failure; the driver never approves or answers it
- bounded workspace access and resume remain unsupported until their exact
  provider mappings are separately promoted

The first Gemini ACP driver is also `AmbientHost`. Plan Mode, denied permission
requests, and bounded read callbacks remain exact provider and host policies,
but they do not constrain every direct filesystem or child-process path in the
Gemini CLI process.

Attached session close owns local HTTP/SSE work only. It does not call OpenCode
instance dispose, authentication, configuration, share, or server lifecycle
routes.

Provider-specific fields stay inside the adapter. Common policy remains
provider-neutral.

## Identity And Redaction

Access policy, harness-isolation posture, request, session, turn, callback,
provider session, provider turn, execution-host, and resource-lease identities
remain distinct. Consumer task, work-item, mandate, review, and receipt ids do
not enter Swallowtail contracts.

Public events and default diagnostics never contain raw paths, prompts,
provider envelopes, tool bodies, approval bodies, user-input bodies, stdout,
stderr, or credentials.

## Acceptance

- read-only sessions retain their existing exact request shape
- ambient harness execution is valid, explicit, and never described as a
  bounded filesystem profile
- provider- and host-enforced isolation are opt-in exact capabilities
- write access is explicit in policy, capability requirements, and resource
  resolution
- one authorized resource is the only writable root for a bounded profile
- approval, isolation, filesystem, provider network, search, callback, and
  deadline dimensions cannot imply one another
- unsupported or mismatched dimensions fail before provider work
- observed provider requests stop safely without consumer authority
- every outcome exposes joined cleanup separately
- local services cannot execute a remote-authoritative resource
