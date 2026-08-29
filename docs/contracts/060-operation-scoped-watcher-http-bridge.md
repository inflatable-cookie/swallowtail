# 060 Operation-Scoped Watcher HTTP Bridge

Status: active
Owner: Tom
Created: 2026-08-30
Updated: 2026-08-30
Research: 257, 260

## Purpose

Carry one opted-in harness route's reserved watcher calls and completion check
into the exact turn-owned Contract 059 watcher service. The bridge is a closed,
host-owned operation resource. It is not a generic MCP server, consumer tool
transport, provider endpoint, sign-in callback, or durable daemon.

## Independent Authority

Contract 010 gains one optional stable `WatcherBridge` host service kind. It is
separate from:

- `Watcher`, which owns watcher identity, work, status, control, and join;
- `Process`, which owns provider and watcher child processes;
- `Network`, which approves external destinations;
- `ServingEndpoint`, which publishes an already running owned service; and
- the one-sign-in loopback callback port.

Registration binds no listener and starts no work. A route requires the bridge
only when its immutable prepared plan opts into a qualified watcher mechanism.
Missing service, host mismatch, or unsupported topology fails before provider
work.

The bridge is provider-neutral. Provider-specific MCP configuration, hook
settings, instruction assets, flags, and version gates remain adapter-owned.

## Operation Lease

Opening the bridge creates one lease bound to the exact:

- execution host;
- runtime operation scope;
- owning turn; and
- registered `WatcherHostService` instance.

The listener is bound and ready before the provider process starts. For a local
provider, it binds an ephemeral loopback endpoint on the provider's execution
host. Loopback is relative to that execution host, never assumed to be the
consumer or orchestrator machine. A remote execution host needs a separately
qualified equivalent transport; it cannot reuse a local endpoint by inference.

Endpoint and authentication material are driver-only lease values. They are
non-serializable and redacted from default `Debug`, `Display`, diagnostics,
events, plans, receipts, and durable records. Public lease state may expose
only safe lifecycle and cleanup truth.

## Authentication And Correlation

Each opened bridge creates fresh, cryptographically unguessable,
operation-private bearer capability material. The bearer may authenticate the
bounded calls within that lease; it cannot authorize another lease or survive
close. Every request must resolve to the lease's exact host, operation, turn,
and open generation before watcher dispatch. Foreign, stale, cross-lease,
post-terminal, missing, or malformed authority and duplicate request
correlation fail closed before watcher work.

Authentication material does not enter:

- provider command arguments or ambient environment;
- user, project, workspace, or durable harness configuration;
- public records, events, diagnostics, logs, errors, or formatting; or
- consumer-visible watcher status and summaries.

A file-oriented provider binding uses an operation-scoped temporary working
resource and bounded host I/O to materialize private configuration. The file
lease releases only after provider, bridge, hook, and watcher cleanup. Raw
paths remain driver-only and redacted. A route must not fall back to inline
command arguments, ambient settings, or shared project mutation when private
materialization is unavailable.

## Closed Protocol Surface

The first carrier is bounded HTTP carrying the minimum MCP lifecycle and the
reserved watcher family. The bridge admits only:

- protocol initialization and the exact tool-list response required by the
  qualified harness;
- Contract 059 start, inspect, list, wait, and stop operations; and
- one completion-gate query returning bounded active-watcher state for the
  owning turn.

It does not admit arbitrary MCP tools, consumer tool declarations, provider
tools, prompts, files, general network requests, shell commands, executable
paths, PIDs, provider task ids, raw output, or unrestricted process launch.
Unknown methods, paths, protocol revisions, tool names, content types, fields,
or correlation values fail closed.

Request and response bodies, headers, watcher counts, concurrent requests,
in-flight work, and wait duration are positively bounded. Decoding completes
before host work. Error bodies remain safe and never echo secret material,
raw provider input, commands, paths, or watcher output.

The bridge calls the same turn-owned watcher service used by operator controls.
It preserves model requester identity. It does not duplicate watcher state,
infer authority from a tool name, or weaken host approval of watcher starts.

## Completion Barrier

The completion query returns the latest bounded active or unjoined watcher
state for the exact turn. It does not silently wait, stop work, or convert a
provider-terminal response into success.

The route adapter remains responsible for an exact qualified pre-terminal
mechanism that calls this gate and returns active-watcher context to the same
model turn. A terminal-only failure, hidden automatic wait, prompt request, or
provider-native background-task check does not satisfy Contracts 059 or 060.

Successful provider completion is admitted only after the gate observes no
active or unjoined watcher and bridge admission has frozen. Races between the
gate, model calls, operator stop, watcher completion, cancellation, and bridge
close resolve under one monotonic turn lifecycle. No request accepted after
the terminal barrier may create or revive watcher work.

## Close And Join

Normal completion, cancellation, deadline, provider failure, hook failure,
transport failure, and explicit close use one joined sequence:

1. freeze new bridge admission;
2. finish or cancel bounded in-flight bridge calls;
3. stop when required and join all turn-owned watcher work under Contract 059;
4. close the listener and join its accept, connection, and dispatch tasks;
5. release private configuration, endpoint, and authentication material; and
6. only then report operation cleanup.

Close is idempotent. Drop is defensive cleanup, not success evidence. No
listener, connection, dispatch task, wait, provider helper, watcher, or private
file detaches past the operation.

## Topology And Security Non-Claims

The loopback listener reduces the carrier delta; it is not authentication by
itself. Port secrecy, process ancestry, provider PID, filesystem location, and
provider-supplied ids never replace the bearer capability and exact scope
checks.

This contract adds no container, virtual machine, sandbox, cgroup, Job Object,
privileged helper, firewall manager, public server, TLS termination, remote
exposure, or hostile-process containment claim. Contract 059's ordinary
host-process and detached-descendant boundaries remain unchanged.

## Conformance

Provider-neutral fixtures prove:

- registration without open creates no listener or task;
- open binds ready-before-spawn endpoint, fresh authority, exact host,
  operation, turn, and watcher service;
- default formatting and public records reveal no endpoint, token, path,
  header, request body, or raw watcher data;
- absent, wrong, stale, cross-lease, duplicate, foreign, malformed, oversized,
  unknown, and post-terminal requests fail before watcher work;
- only the reserved watcher family reaches the same registry as operator
  controls;
- concurrent wait, stop, completion, cancellation, and close races resolve
  deterministically without work after the terminal barrier;
- every terminal and failure path freezes admission, joins watcher and bridge
  work, releases private material, and reports cleanup exactly; and
- omission leaves provider payloads and host service requirements unchanged.

Route fixtures additionally prove exact private configuration, instruction and
hook delivery, version behavior, same-turn re-entry, unchanged omission, and
no support claim before the complete route seam passes.

Live provider turns and credentials remain separately authorized evidence.
They are not part of ordinary provider-neutral bridge conformance.

## Acceptance

- one opted-in provider route can reach only the exact turn's reserved watcher
  operations through a host-owned joined bridge
- model and operator controls continue to use one watcher registry
- endpoint, authentication, configuration, paths, and raw activity remain
  private
- successful turn completion cannot race with new, active, or unjoined watcher
  work
- omission creates no listener, private material, provider argument, hook,
  skill, event, or capability claim
- no generic MCP, network, process, container, daemon, or consumer-tool
  authority enters the portable surface
