# 077 Claude Managed Agent Records And Fixtures

Status: completed
Owner: Tom
Updated: 2026-07-21
Milestone: `../025-claude-managed-agent-remote-harness-proof.md`

## Objective

Realize Contract 022's minimum shared policy and cleanup records, then freeze
the exact Claude Managed Agents beta REST/SSE corpus behind deterministic
loopback fixtures.

## Governing References

- Research 016
- Contracts 005, 006, 008-012, 014, 016, and 022
- roadmap 025

## Scope

- extend provider retention with an explicit durable-allowed posture
- add explicit provider-managed-recovery capability and request acceptance
- represent confirmed versus unconfirmed deletion for each owned remote
  resource without exposing provider identifiers
- reject weaker retention, missing recovery acceptance, or ownership mismatch
  during pure preflight before endpoint or credential effects
- add a distinct Managed Agents protocol boundary inside
  `swallowtail-adapter-anthropic`; do not widen the existing Messages driver
- freeze the `managed-agents-2026-04-01` beta header, first-party endpoint
  audience, API-key boundary, agent/version/model validation, limited
  environment, session, message, event stream/history, callback result,
  interrupt, session delete, and environment delete shapes
- deterministic success, requires-action, rescheduling, disconnect/history,
  cancellation, deadline, provider failure, schema drift, deletion failure,
  redaction, and cleanup-order fixtures
- no live account, credential, remote resource, external request, or paid work

## Ordered Steps

1. Add the minimum common capability, policy, requirement, and cleanup records
   required by Contract 022; keep ordinary runs unchanged.
2. Extend pure preflight and unit fixtures for exact durable retention,
   provider recovery, resource ownership, and rejection before effects.
3. Add the dated Managed Agents protocol module and static corpus inside the
   Anthropic adapter crate.
4. Add loopback REST/SSE fixtures for authoritative event order, one bounded
   reconciliation, callback correlation, interrupt, ordered deletion, and
   safe failures.
5. Run focused core, runtime, testkit, and Anthropic fixture validation.

## Acceptance Criteria

- [x] durable retention and provider-managed recovery are independent opt-ins
- [x] ordinary Anthropic Messages and all existing drivers preserve their
      current retention and retry posture
- [x] agent, version, model, environment, session, sandbox, event, attachment,
      runtime, callback, and request identities cannot be confused
- [x] previews never establish output or terminal truth
- [x] disconnect reconciliation cannot create a second session or task
- [x] session and environment deletion truth remains separate and redacted
- [x] exact limited-network environment creation is proven without an external
      host allowlist
- [x] card 078 has no unresolved shared-contract, access, or protocol decision

## Completion Evidence

- core capabilities distinguish durable retention, managed recovery, and
  deletion authority for owned environment and session resources
- runtime policy makes durable retention and managed recovery independent
  opt-ins; terminal outcomes retain per-resource confirmed or unconfirmed
  deletion truth
- structured runs can carry existing bounded tool declarations and expose the
  existing callback exchange through run handles; drivers without that subset
  reject both tools and managed recovery before effects
- provider-neutral testkit fixtures reject missing retention, recovery, or
  owned-resource requirements before their recorded provider effect
- the dated `managed-agents-2026-04-01` corpus freezes exact first-party
  headers, pinned agent/session overrides, limited empty-host networking,
  messages, callbacks, interruption, authoritative SSE/history, failures, and
  deletion responses
- the Anthropic test-only protocol boundary rejects preview events, malformed
  disconnects, contradictory duplicate history, unsafe schemas, and deletion
  ambiguity; card 078 will promote it into production use
- the loopback transcript permits one session, one stream attachment, bounded
  history reconciliation, callback and interrupt events, and session-before-
  environment deletion without live access

## Evidence Required

- focused common-record and preflight tests
- dated static corpus with source and exclusion notes
- deterministic loopback transcript assertions
- proof that rejected policy creates no endpoint, credential, or provider call
- proof that default validation uses no provider account or live access

## Validation

- focused core, runtime, testkit, and Anthropic adapter tests
- focused warnings-denied clippy
- `git diff --check`

## Stop Conditions

- current first-party documentation does not define an exact limited
  environment with no required external host access
- the beta schema cannot distinguish authoritative persisted events from
  best-effort previews
- shared records would encode Anthropic identity or silently widen existing
  driver behavior
- deterministic fixtures require live provider access

## Auto-Continuation

No. Promote card 078 only after the corpus is stable and its production
lifecycle has no unresolved boundary.
