# Subagent Topology, Observation, And Control

Status: active
Owner: Tom
Updated: 2026-07-31

## Purpose

Expose provider-owned child work as a bounded portable graph so consumers can
browse and inspect side jobs without parsing tool prose, inventing parentage,
or receiving hidden provider state.

Keep observation, provider-owned collaboration, and direct operator control
separate.

## Separate Concepts

The following are independent:

- main operation
- provider thread or session
- child work identity
- child parent relation
- activity actor
- child lifecycle snapshot
- transient operation-local directory projection
- provider-owned collaboration action
- operator-issued child control
- whole-turn cancellation
- callback or question exchange
- consumer tree projection and persistence

A provider tool named `Task`, `agent`, or `delegate` is not a child graph. A
child id is not control authority. Whole-turn interruption is not child
interruption.

## Operation-Local Graph

One activity observation names its actor:

- primary operation
- one known child

A subagent or collaboration observation may also carry a bounded set of child
snapshots. Each snapshot contains:

- one non-empty bounded operation-local child id
- known operation parent, known child parent, or unknown parent
- one current portable status
- optional bounded label
- optional bounded task description
- optional exact model and reasoning mode
- optional foreground or background posture
- optional opaque originating provider activity reference

Portable status is:

- unknown
- pending
- running
- waiting
- completed
- failed
- interrupted
- shutdown

The activity lifecycle and child status remain separate. A completed `wait`
action may observe a still-running child. A completed `spawn` action does not
mean the child completed.

Repeated snapshots replace prior metadata for the same operation-local child.
Omission carries no deletion or completion meaning. Consumers may retain the
latest snapshot until terminal operation truth or a later snapshot replaces
it.

The runtime may project those ordered observations into one transient
operation-local directory. The projection:

- takes one explicit positive child bound before observation
- rejects activity from another operation without changing its state
- retains children in first-observed order
- replaces one child's full snapshot when later truth differs
- emits added or replaced changes only
- returns the exact activity actor for downstream transcript routing
- retains terminal children until the projection is dropped

An attributed child or referenced parent with no snapshot may create an
identity-only placeholder with unknown parent and status. This preserves known
topology without inventing metadata. Later provider truth replaces the
placeholder.

Capacity exhaustion is a local projection failure. It does not invalidate the
provider event, fail the operation, or authorize silent eviction.

## Activity Attribution

An adapter marks an activity as child-authored only when the selected wire
supplies a stable child identity. It does not infer attribution from labels,
tool names, text prefixes, timing, or nesting in provider display content.

Parentage is `Unknown` when the provider identifies a child but does not expose
its parent. Unknown parentage is not silently promoted to the main operation.

The same provider child identity maps to the same operation-local child id
within one run or turn. That id is correlation evidence. It cannot be passed
to another route, configured instance, operation, or host as authority.

A route whose native activity envelope is child-owned may keep one bounded
operation-local admission set. The root thread remains admitted. A child enters
the set only after an earlier qualified topology observation establishes its
exact id for that operation. A child-owned envelope is then attributed to that
child even when the activity item carries no separate child field.

Admission is activity-only. It does not authorize callbacks, provider
requests, terminal events, root-turn output, provider-session operations, or
direct child control. Unknown, stale, and cross-operation ids fail closed. The
set is cleared at operation termination and is never imported by a later run,
turn, resumed session, or consumer. Missing parent evidence remains
`Unknown`; admission does not invent parentage.

## Provider-Owned Collaboration Actions

Observable provider-owned actions use an exact typed vocabulary:

- spawn
- send input
- resume
- wait
- close

These values describe what the harness or main agent did. They do not expose a
consumer command method.

A route profile declares each observable action separately. A route may expose
child lifecycle without exposing collaboration actions.

## Operator Inspection And Control

Live activity snapshots and the transient directory projection are the current
portable inspection surfaces. Consumer persistence may make the graph
browseable after the operation ends.

A future provider-native directory or history reader is a separate bound
inspection role. It must define:

- root operation or session authority
- pagination and item bounds
- child and parent identity mapping
- visible-content disclosure
- retained versus live truth
- version and experimental-interface qualification

Direct operator control is a separate bound role. Each supported action needs:

- exact route and interface qualification
- one authorized child target
- supported action and payload bounds
- pre-dispatch rejection
- provider acknowledgement or explicit uncertainty
- deadline and cancellation truth
- terminal interaction with parent and child work
- joined cleanup before authority release

No common fallback is permitted. An unsupported child cancel does not become
whole-turn cancel. Unsupported steering does not become a message to the main
agent. Unsupported close does not become provider-session deletion.

This contract does not authorize arbitrary provider-thread access.

## Bounds And Disclosure

Child ids, labels, descriptions, model identifiers, reasoning modes, and
originating references are bounded. Child snapshots are bounded per
observation and reject duplicate child ids.

Descriptions are operation content, not safe diagnostics. Child ids, labels,
descriptions, model values, reasoning values, and provider references are
redacted in default formatting.

No child snapshot may expose hidden reasoning, raw tool arguments, raw tool
results, provider envelopes, credential material, filesystem state, or
unqualified internal status payloads.

## Prepared Fidelity

The observable-activity profile may declare:

- identity and lifecycle
- parent and metadata
- attributed child activity

Richer fidelity satisfies thinner requirements. The profile may separately
declare observable provider-owned collaboration actions.

These constraints describe observation only. They do not imply provider-native
directory, history, steering, interruption, or close authority.

## Realized Routes

| Route | Realized portable detail |
| --- | --- |
| Codex app-server | parent and child thread identity, bounded spawn metadata and child state, visible collaboration action, child activity attribution |
| Codex exec | parent and child thread identity, bounded spawn metadata and child state, visible collaboration action |
| Kimi local-server | child identity, main-operation parent, name, background posture, originating tool correlation, and full lifecycle including waiting |

Other routes make no child-topology claim. Generic tool activity remains
available where already qualified.

No selected route currently exposes direct operator child control through a
Swallowtail handle.

## Consumer Ownership

Consumers own:

- durable graph and transcript persistence
- sidebar, tree, tabs, badges, grouping, and collapse state
- labels that replace provider presentation
- selection and navigation
- operator policy and confirmation UI
- retry or recovery workflows
- consumer record retention and deletion

Swallowtail owns exact portable identity, topology evidence, lifecycle,
attribution, bounds, redaction, ordered delivery, the bounded transient
directory reducer, route fidelity, and any future bound provider control role.

## Conformance

Deterministic fixtures prove:

- stable child identity across lifecycle observations
- operation and nested parent mapping where claimed
- primary and child actor attribution
- independent activity lifecycle and child status
- bounded label, description, model, reasoning, background, and origin detail
- exact observable collaboration actions
- unknown parent and status without invented truth
- first-observed directory order and exact snapshot replacement
- identity-only actor and referenced-parent placeholders
- operation mismatch and capacity rejection without partial mutation
- exact activity-actor routing in each directory delta
- profile constraints for each fidelity tier and action
- duplicate and oversized snapshot rejection
- redacted formatting
- no direct-control claim from observed provider actions
- root and previously established child-owned activity envelope admission
- unknown, stale, and cross-operation child-envelope rejection
- operation-terminal child-admission cleanup without widening callback, turn,
  terminal, or provider-session ownership

No live authentication, paid inference, installed harness, or provider effect
runs in default conformance.
