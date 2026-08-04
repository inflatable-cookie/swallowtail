# Anthropic Managed Agent Prepared Integration

Use this facade for one resource-free Claude Managed Agents run. It binds the
first-party Anthropic API, exact beta facade, public API-key access, one
operator-owned agent version and model route, one driver-owned environment,
one driver-owned session, durable provider retention, managed recovery,
authoritative event reconciliation, callbacks, interruption, and ordered
deletion.

This is provider-hosted harness execution. It is not the Anthropic Messages
direct-inference facade, Claude Agent SDK, Claude Code, a Claude subscription,
Bedrock, or another cloud marketplace.

## Managed Preparation

`prepare_anthropic_managed_agent` requires:

- configured-instance identity and revision
- one execution host and opaque host-approved Anthropic endpoint target
- the exact Managed Agents public API-key pay-as-you-go access profile
- observed or caller-asserted access evidence
- one operator-owned provider agent identity and exact numeric version

`anthropic_managed_access_profile` constructs the exact access shape from a
consumer-selected credential reference. Preparation performs no endpoint,
credential, or provider work. It never creates, updates, archives, or deletes
the operator's agent definition.

The prepared integration exposes its safe instance, access provenance,
available host services, target-drift check, and managed low-level driver. Its
driver identity and `HarnessInteraction` layer cannot substitute for the
separate direct Messages preparation.

## Explicit Run Authority

`prepare_managed_run` requires:

- request identity and text task
- exact route identity, revision, and model identity
- one operation deadline
- zero to eight consumer-declared custom tools
- `DurableAllowed` provider retention
- `ManagedAllowed` provider recovery
- exactly one authoritative-history reattachment

The full constructor keeps the three provider-state policies visible.
`durable_with_managed_recovery_and_one_reattachment` is the named shortcut for
that fixed profile.

That profile is still an ordinary attached run: closing its handle interrupts
active work when supported and deletes its session and environment. Call
`with_cross_process_recovery` only when the consumer will persist the emitted
reconciliation checkpoint and recovered-resource cleanup binding. This opt-in
adds recovery authority without changing the task, model, callback, retry,
reattachment, or ordinary close policy.

The current subset always creates a cloud environment with limited networking,
an empty host allowlist, and package-manager and MCP allowances disabled. It
grants no repository, provider filesystem, external sandbox network, built-in
tool, MCP, skill, vault, memory, multiagent, schedule, webhook, or file
authority.

## Lifecycle

`start_run` delegates unchanged to the low-level managed-agent driver:

1. validate the pinned operator agent version
2. create one limited driver-owned environment
3. create one driver-owned session pinned to that agent version and route
4. send one user task and consume authoritative persisted events
5. reconcile history once and reattach after one stream loss
6. relay correlated custom-tool callbacks without executing them
7. send `user.interrupt` after cancellation or deadline when work is active
8. delete the session, then delete the environment
9. join local work, then release the credential

Provider `rescheduling` is managed recovery inside the same provider session.
It is not a Swallowtail retry and does not authorize another session, route,
model, endpoint, or credential. Connection preview deltas never establish
terminal truth.

Deletion results remain separate for session and environment. Clean local
cleanup requires confirmed deletion of both. Failed or ambiguous deletion is
degraded cleanup, not a claim that provider state disappeared.

The returned run handle exposes the existing event, callback, cancellation,
terminal-outcome, remote-deletion, and cleanup APIs. `plan`, `request`,
`evidence`, `low_level_driver`, and `into_parts` remain available for
inspection and advanced use.

## Cross-Process Recovery

After the exact environment and session exist, a recoverable run emits one
semantic runtime event carrying two separate opaque records before it submits
the user task:

- `ProviderRunCheckpoint` admits read-only observation
- `ProviderRecoveredResourceCleanupBinding` admits later explicit cleanup

Persist each record with `export_persisted(run.plan())`. Do not extract or
store the environment id, session id, or adapter-private cursor. A fresh
process must prepare the same configured instance, host, endpoint, access
profile, agent version, facade, and model route before either record restores.

For observation:

1. construct `AnthropicManagedRunReconciliationInput` with a new request id,
   the exact model selection, persisted checkpoint, positive recovered-output
   byte bound, and optional deadline
2. call `prepare_run_reconciliation`
3. call `reconcile` with the current host services

The driver retrieves the exact session and at most eight ordered event pages,
with a total ceiling of 2,048 events. It can report `Active`,
`WaitingForProviderInput`, `Completed`, `Failed`, `Cancelled`,
`InactiveUnresolved`, or `Unknown`. Only exact terminal attribution may carry
bounded output and usage.

Reconciliation sends no message, retry, stream attachment, interrupt, callback
answer, archive, or delete request. `WaitingForProviderInput` is observation,
not a restored callback responder; the original callback id grants no authority
to the new process.

For cleanup, construct `AnthropicManagedRecoveredCleanupInput` from the
separately persisted cleanup binding, then call `prepare_recovered_cleanup`
and `cleanup`. The driver rechecks the exact session first. Active, waiting, or
ambiguous work is preserved. Once inactive, it attempts the exact session
deletion and requires confirmation before attempting the exact environment
deletion. Cancellation, deadline, or uncertain effects are returned without
retry.

Discard each persisted record only according to the consumer's durable state
policy after reconciliation or cleanup truth no longer needs to survive a
restart. Neither record is a reusable session, prompt, callback, or management
binding.

See the compile-tested
[`prepared_managed_agent` example](../../crates/swallowtail-adapter-anthropic/examples/prepared_managed_agent.rs).
