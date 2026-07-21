# 017 Provider-Owned Session Load, Replay, And Host Containment

Status: active
Owner: Tom
Updated: 2026-07-21

## Purpose

Allow a harness driver to load or resume provider-owned persistent sessions
and service ambient or bounded filesystem work without confusing provider
persistence, consumer state, callback authority, permission approval, or
process isolation.

## Persistent Session Identity

A provider-owned persistent session reference is opaque. It is distinct from:

- the runtime session handle created for one attachment
- the process or protocol connection
- a provider turn, replay item, callback, request, or model id
- consumer conversation, task, memory, or persistence identity

Every new attachment gets a new runtime session id. Loading or resuming a
provider reference does not revive an old runtime handle.

A durable session binding fixes:

- provider session reference
- configured instance and execution host
- model route and model
- working-resource reference and access
- expanded interactive session access policy
- harness-isolation posture when a local harness process exists

Load or resume must match every dimension before provider, process, resource,
or credential work. A provider session discovered through list, copied from a
diagnostic, or supplied as an unbound string has no authority to attach.
Consumers may persist an opaque binding, but Swallowtail does not provide a
session database or infer consumer persistence policy.

## New, Load, And Resume

New, load, and resume remain separate lifecycle operations:

- **new** creates provider state and returns the initial durable binding
- **load** attaches to bound provider state and transports historical replay
  before returning a ready session
- **resume** attaches to the same kind of bound provider state without
  historical replay

One generic resume boolean or provider-specific option cannot represent all
three. A driver may implement only the operations it advertises. A proxy may
build load from resume plus consumer-owned history only through a later
explicit contract; the first implementation does not.

The driver derives the provider working directory or equivalent project
location only from the host lease resolved for the binding. Provider-stored
roots, ambient current directory, request strings, list metadata, and prior
process state cannot replace it. Under an enforced isolation posture, that
lease also supplies the exact boundary root. Under `AmbientHost`, it remains a
location and callback scope only. If the provider cannot verify or safely re-
establish the bound resource and posture, load and resume fail closed.

## Replay Phase

Provider replay belongs to the load operation. It is not live turn output.

- replay items carry the bound provider session and their own monotonic replay
  sequence
- historical user, agent, reasoning, tool, plan, and configuration updates
  keep distinct semantic kinds when represented
- replay ordering is preserved exactly
- the load response is a phase boundary and arrives only after replay ends
- no turn may start before the load phase completes
- replay is bounded; overflow fails load and returns no usable session handle
- a protocol or provider failure during replay fails load even if some replay
  items were already observed
- resume exposes no replay phase; a historical replay item before its response
  is a protocol failure

Replay transport does not assert that history is complete, accepted, or
persisted by the consumer. Consumers decide whether and how to merge it with
their own transcript. Replay items do not increment live-turn usage or trigger
product effects.

## Filesystem Write Callbacks

`WorkingResourceIo` may expose bounded text replacement in addition to bounded
text read. A write request contains a redacted provider locator and bounded
UTF-8 content. It is authorized only when:

- the immutable plan declares the exact write callback capability and bounds
- the session policy selects `ResourceAccess::ReadWrite`
- one filesystem working-resource lease matches scope, session, execution
  host, reference, and access
- the selected host provides the write-capable callback service

The host resolves the target canonically under the one leased root. It rejects
absolute-root mismatch, traversal, symlink or junction escape, wrong scope,
wrong session, wrong host, wrong representation, non-text content, and excess
bytes before mutation. Creating a missing regular file and replacing one
regular file are the first common operations. Success is returned only after
the host completes the whole requested replacement.

Append, partial patch, binary fidelity, chmod, rename, delete, directory
creation, multiple roots, and atomic-replace guarantees are not implied.
Provider locators and content never enter public events or default diagnostics.

## Callback, Approval, And Isolation Independence

Filesystem callback authority is not provider-tool approval. Permission
requests remain distinct provider callbacks under Contracts 012 and 015. A
driver cannot approve Write, Edit, shell, or another provider action merely
because a `ReadWrite` lease exists.

Filesystem callbacks are also not a process sandbox. A harness may have local
filesystem or child-process paths that bypass its text callback. Such a
harness may run under an explicit `AmbientHost` posture without claiming a
filesystem boundary. A bounded interactive profile is valid only when every
filesystem path is either:

- mediated through the matching host callback service, or
- contained by a provider or execution-host mechanism whose exact root,
  access, child-process inheritance, and cleanup behavior are preflight-bound
  and tested

Setting a working directory, passing a resource path, denying an approval,
advertising ACP filesystem capabilities, or trusting provider convention does
not prove containment. A driver without complete mediation or tested
containment cannot claim read-only or bounded-write filesystem isolation. It
may still advertise ambient communication when that posture is immutable,
visible to the consumer, and never substituted for an enforced route.

Process containment is host or provider authority, not consumer authorization
policy. The consumer still owns whether a provider tool should run and whether
an ambient route is offered. A host containment service must apply on the
selected execution host and cover child processes; a local sandbox cannot
represent remote-authoritative execution.

### Mechanism Qualification

This section applies only when a configured route claims `ProviderEnforced` or
`HostEnforced` isolation. It does not gate `AmbientHost` communication.

A containment mechanism qualifies only when current authoritative platform
evidence and tests cover every filesystem path needed by the selected access
profile. The proof must include exact root and access binding, descendant
inheritance, symlink and link behavior, filesystem topology, special
filesystems, alternate mutation syscalls, inherited descriptors, runtime
availability, cancellation, and joined cleanup.

Runtime availability covers the exact deployed executable, not a synthetic
stand-in alone. Packaging or re-signing must preserve required runtime
behavior, including embedded or dynamically extracted native code, under the
documented entitlement and signing model. A provider artifact whose runtime
requirements conflict with descendant inheritance does not qualify. Alternate
runtime flags, repackaging, source builds, or module substitution require their
own provider or integration support authority and pinned evidence.

Partial enforcement cannot be promoted as bounded filesystem access. In
particular:

- best-effort kernel feature negotiation is insufficient
- working-directory, callback, DAC, permission-prompt, or provider convention
  evidence is insufficient
- a mechanism with uncovered read or mutation syscall families is
  defense-in-depth only
- a synthetic helper passing while the selected provider artifact crashes,
  hangs, or cannot load required code is insufficient
- a deprecated, private, experimental, or undocumented platform interface is
  not production support authority
- a sandbox supplied by an application, container, VM, or remote deployment
  remains that execution host's authority and cannot be fabricated locally

Unsupported mechanism, version, platform, configuration, host, root, access,
or child-inheritance evidence must fail before process start when an enforced
posture is selected. Do not add a portable runtime containment record until at
least one concrete mechanism passes this qualification boundary. Never fall
back from an enforced posture to `AmbientHost`.

## Delegated Authentication And Sign-In

Existing harness-owned authentication may satisfy a configured instance
through a scoped delegated credential lease. It exposes no token, state path,
or reusable provider credential.

An advertised login command, device flow, browser flow, terminal auth method,
or provider `_meta` record is a supported sign-in action, not permission to
execute. Starting it requires explicit host and operator authorization,
executable and environment validation, and an isolated provider-state scope.
Authentication success only refreshes evidence for the same configured
instance and endpoint audience.

One harness may accept several credential mechanisms or downstream providers.
Each combination remains a separate configured instance and access profile.
The driver cannot discover the active route by scanning configuration or
silently substitute OAuth, API keys, cloud identity, or another provider.

## Cancellation, Disconnect, And Cleanup

- active-turn cancellation uses the provider method when available
- cancelling load stops replay, closes the owned attachment, and returns no
  session handle
- session close uses native close only when advertised and proven
- otherwise close ends input and stops only the host-owned process or
  connection under existing lifecycle authority
- disconnect invalidates the runtime attachment but does not delete durable
  provider state
- pending replay, callbacks, turns, readers, process work, containment leases,
  resource leases, and delegated credentials are joined or released in owner
  order
- provider completion never hides cleanup degradation or failure

Deletion, retention, export, history compaction, and garbage collection of
provider sessions remain provider or consumer policy and are not implied by
close.

## Conformance

Deterministic persistent-session and write fixtures must prove:

- new creates a binding and arbitrary ids cannot load or resume
- every binding dimension rejects mismatch before side effects
- load and resume cannot change ambient, provider-enforced, or host-enforced
  isolation posture
- load replay is ordered and completes before readiness
- resume emits no replay
- replay overflow, disconnect, cancellation, and wrong-session correlation
  produce no usable session handle
- writes require exact `ReadWrite` callback authority and stay under one root
- read-only, traversal, symlink, size, scope, host, session, and resource
  mismatches reject before mutation
- provider permission observation does not grant write authority
- process containment is present when callback mediation is incomplete and a
  bounded filesystem claim is selected
- ambient harness routes expose no bounded filesystem or descendant claim
- the selected containment mechanism covers the complete profile rather than
  degrading to partial or best-effort enforcement
- existing delegated auth exposes no secret and sign-in is not launched
- cleanup joins process and callback work before releasing resource,
  containment, and credential leases

Provider-specific session fields, replay envelopes, auth methods, and sandbox
mechanisms stay inside adapters and hosts. No Kimi, Gemini, Codex, or ACP
identity branch enters the common contract.

## Acceptance

- load and resume cannot be flattened
- provider replay cannot masquerade as a live turn
- arbitrary provider session ids cannot reactivate stored filesystem scope
- write callbacks require exact host and `ReadWrite` authority
- callback authority, provider approval, and process containment remain
  independent
- ambient communication does not require containment
- unsupported or incomplete containment fails before provider work only when
  an enforced posture was selected
- enforced isolation never degrades silently to ambient execution
- delegated login cannot expose or mutate credential state implicitly
- disconnect and close preserve provider state while joining runtime work
