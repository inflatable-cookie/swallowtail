# 059 Operation-Scoped Process Watchers

Status: active
Owner: Tom
Updated: 2026-08-29
Research: 255, 259

## Purpose

Give a model and operator one dependable operation-scoped way to start, wait
for, inspect, and stop host-owned background work while exposing bounded state
to consuming applications. A watcher is concurrent work inside one agent turn,
not a detached process, provider task, daemon, or arbitrary PID.

## Opt-In Boundary

Watchers are an explicit prepared route option. Absence means:

- no watcher skill or instructions are injected
- no reserved watcher tools are declared
- no watcher host service is required
- no watcher activity or completion gate runs

A route advertises watcher support only when its exact model-facing mechanism,
host service, completion interception, version segment, and cleanup behavior
are qualified together. Prompt text alone never advertises the capability.

## Identity And Ownership

Each watcher has:

- one stable id scoped to the owning runtime turn
- one owning turn, session, configured instance, and execution host
- one host-owned launch or task binding
- one corresponding operation-local activity identity
- one monotonic lifecycle revision

Watcher ids are opaque and never equal a PID, process group, provider task id,
callback id, activity id, or consumer record id. Model and operator operations
accept only ids owned by the active turn. Foreign, stale, unknown, reused, and
post-terminal ids fail closed.

The host service owns watcher work. A provider-native child, command, task,
subagent, session, or activity becomes a watcher only through a separately
qualified mechanism that supplies the same ownership, control, completion, and
join guarantees. Observation alone never promotes it.

## Host Watcher Service

Contract 010 gains one optional capability-scoped watcher service. The service
provides bounded operations:

- start one host-authorized watcher request
- inspect one watcher or list the active turn's bounded watcher set
- wait for one watcher to reach terminal state
- request idempotent stop
- stop and join every watcher owned by a cancelled or timed-out turn

The model-facing start payload is bounded operation data interpreted under host
policy. It is not an executable path, shell command contract, arbitrary
argument vector, PID, or permission grant. The host may reject any request
before work. Registration does not authorize a start.

Every accepted start binds process-tree or task ownership before returning the
watcher id. The host owns output capture, backpressure, cancellation,
deadlines, graceful stop, force-stop where authorized, descendant cleanup, and
join. No watcher may outlive its owning turn.

## Containment Admission

The watcher registry coordinates lifecycle; it does not manufacture process
containment. A process-backed watcher is admitted only when the selected host
composition supplies an exact containment backend with an owned lease that:

- contains descendants by construction, including concurrent forks;
- does not permit ordinary child APIs to leave the containment scope;
- targets stop and force-stop through the lease rather than a numeric PID;
- reports terminal only after the contained workload is terminal;
- proves the containment scope empty before joined cleanup; and
- joins its own supervision work before releasing turn resources.

A root-process handle, process group, inherited output pipe, process-table
poll, or observed parent chain is not a containment backend. Detection of an
escaped descendant is useful failure evidence but does not satisfy ownership
or cleanup. Default macOS process groups and `launchd` group cleanup therefore
do not qualify.

Hosts without a qualified containment backend must omit process-backed watcher
support or reject the start before work. Registration of the portable watcher
service does not imply that a process executor is present. A task-backed
watcher qualifies only when the host proves the task cannot create unmanaged
child work, or binds all such work to the same containment lease.

Containment is capability-gated, not platform-inferred. Windows Job Objects,
Linux cgroup v2, a consumer-supplied supervisor, container, or VM remains
unavailable until its exact authority, breakaway behavior, termination, empty
scope, join, and failure semantics are implemented and tested. No route may
advertise watcher support from an operating-system name alone.

## Model And Operator Controls

The model and operator use separate typed control paths against the same
registry:

- model operations arrive through one exact reserved watcher tool family
- operator operations arrive through the consumer-facing turn control surface

The paths retain requester identity and authorization separately. They do not
create separate watcher state. Repeated stop is idempotent. Completion racing
with stop resolves to the first terminal transition and reports the exact
result; it never stops unrelated work.

The reserved watcher tools are not generic consumer tools under Contracts 012
and 041. Swallowtail transports and services only this closed lifecycle through
the registered watcher host port. All other tool names and execution remain
consumer- or provider-owned.

## Injected Watcher Skill

An opted-in route delivers one versioned, bounded Swallowtail-owned watcher
instruction asset through an exact qualified harness mechanism. The asset
teaches the model when and how to call start, status, wait, and stop. It cannot
grant process, tool, filesystem, network, permission, or completion authority.

The exact route may use a native skill, developer instructions, MCP, dynamic
tools, or another documented mechanism only when preparation binds and tests
that mechanism. These mechanisms are not interchangeable. Contract 058 may
observe the injected skill independently; watcher authority does not depend on
that observation.

## Lifecycle

The portable lifecycle is:

- accepted
- running
- completed, failed, cancelled, timed out, or stopped
- joined

Accepted precedes work. Running may be omitted only when start fails before
work. Exactly one terminal cause wins. Joined is cleanup truth, not another
process result. A successful process result with failed join is not clean
completion.

Status and wait return the latest monotonic snapshot. Wait pauses the agent's
tool call until the selected watcher is terminal and joined, or the owning turn
is cancelled or timed out. Polling remains available but is not required for
correctness.

## Turn Completion Gate

An ordinary turn cannot complete successfully while any owned watcher remains
non-terminal or unjoined.

The normal path is explicit model wait or stop. If the model attempts to finish
early, the exact route must intercept completion before provider terminal
becomes irreversible and return bounded active-watcher state to the same model
turn. The model then waits or stops. A terminal-only route that can merely turn
provider success into a Swallowtail failure does not satisfy this gate.

On consumer cancellation or host deadline, the runtime stops and joins every
owned watcher, then returns the exact cancelled or timed-out turn result. On
provider, transport, callback, hook, or watcher-channel failure, cleanup still
stops and joins owned watchers before returning failure. Prompt compliance is
never cleanup evidence.

## Output And Consumer Projection

Consumers receive watcher lifecycle, status, and bounded redacted output
summaries on the existing ordered turn event stream. Contract 044 gains one
host-watcher activity kind. Its activity id is presentation identity; watcher
control still requires the watcher id and owning turn.

Raw stdout, stderr, command text, arguments, environment, paths, secrets, and
unbounded logs do not enter portable events or diagnostics. A summary may
carry bounded progress, terminal status, safe failure classification, duration,
and truncated redacted output selected by the host. Consumers own UI,
persistence, retention, disclosure preferences, and operator policy.

## Session And Recovery Boundary

Watchers belong to one turn. Session load, resume, recovery attachment,
reconciliation, controlled detachment, or consumer restart never recovers or
reattaches them. A session cannot become ready with inherited active watchers.
Durable services and provider-owned background runs remain Contracts 018, 021,
022, 048, and 049 concerns.

## First Proof Disposition

Claude Code headless is the first evidence candidate because the selected
interface exposes strict MCP configuration, hook events, background-work
behavior, and documented skill loading. Current Swallowtail intentionally
passes an empty strict MCP configuration and has no operation-scoped stop-hook
injection. Research must prove a bounded private watcher MCP/tool path and a
pre-terminal hook that can return active-watcher state to the same `-p` turn.

Codex app-server dynamic tools are insufficient for the first proof: current
evidence has no pre-terminal completion interception. No production route
advertises watcher support until the complete Claude seam or another exact
route closes.

## Conformance

Provider-neutral fixtures must prove:

- turn-scoped identity and rejection of foreign, stale, PID, and reused ids
- bounded counts, inputs, status, summaries, and event delivery
- model and operator stop through distinct paths against one registry
- start rejection before work and exact accepted/running/terminal/join order
- completion-versus-stop races and repeated stop
- explicit wait, wait cancellation, and wait deadline
- early completion returned to the same model turn rather than converted into
  hidden automatic wait or terminal-only failure
- cancellation, timeout, provider failure, hook failure, and close stop and
  join all owned work
- absence or pre-work rejection when process containment is unavailable
- containment-lease empty truth under descendant fork and escape attempts
- no watcher work or provider payload change when the option is absent
- no raw process content in events, diagnostics, or default formatting

Route fixtures additionally prove exact instruction delivery, reserved tool
admission, completion interception, version behavior, and unchanged omission.

## Acceptance

- model and operator can start, inspect, wait for, and stop owned watchers
- consumer applications receive bounded truthful process activity
- successful turn completion is impossible while owned work remains active
- every failure path stops and joins operation-scoped work
- no arbitrary process, detached daemon, native-task inference, or raw-log
  authority enters the portable surface
- no production route advertises support before the same-turn completion gate
  is proved
