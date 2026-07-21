# 078 Claude Managed Agent Driver

Status: completed
Owner: Tom
Updated: 2026-07-21
Milestone: `../025-claude-managed-agent-remote-harness-proof.md`

## Objective

Implement the bounded Claude Managed Agents remote-harness driver against card
077's frozen corpus and Contract 022.

## Governing References

- Research 016
- Contracts 005, 006, 008-012, 014, 016, and 022
- roadmap 025
- card 077 fixture and shared-record evidence

## Scope

- separately registered structured-run harness driver inside
  `swallowtail-adapter-anthropic`
- exact first-party endpoint, API-key lease, beta header, configured agent,
  agent version, model route, deadline, durable retention, and recovery policy
- one driver-owned limited-network environment and one driver-owned session
- text task, bounded declared custom tools, authoritative persisted events,
  one bounded history reconciliation, cumulative usage, request and rate
  evidence, and safe diagnostics
- native interrupt on cancellation or deadline
- session-before-environment deletion, joined local work, then endpoint and
  credential release
- no repository, files, built-in tools, external sandbox network, MCP, skills,
  memory, multiagent, GitHub, schedules, webhooks, resume, retry, or fallback

## Ordered Steps

1. Register the distinct remote-harness descriptor and exact preflight binding.
2. Implement environment and session creation without mutating the operator-
   owned agent definition.
3. Implement authoritative event streaming, callback exchange, bounded history
   reconciliation, interruption, terminal projection, and usage evidence.
4. Implement ordered deletion and joined cleanup on every terminal path.
5. Validate against card 077's loopback corpus without live access.

## Acceptance Criteria

- [x] no Anthropic branch enters provider-neutral core or runtime behavior
- [x] no provider effect occurs before exact binding and policy validation
- [x] rescheduling remains provider-managed recovery, not Swallowtail retry
- [x] callback transport never executes consumer tools
- [x] disconnect does not create another session or task
- [x] cleanup truth distinguishes interrupt, session deletion, environment
      deletion, local join, and lease release
- [x] default QA uses no provider account, credential, remote sandbox, or paid
      inference

## Completion Evidence

- added a distinct `swallowtail.anthropic.managed-agent` harness driver and
  host-approved REST/SSE transport with the exact API version and beta headers
- added provider-neutral provider-agent/version preflight binding and callback
  operation identity so structured runs do not invent interactive turns
- production provisioning validates the operator-owned agent, creates one
  limited environment and one session, sends one task, and never mutates the
  agent definition
- authoritative event handling covers success, custom callbacks,
  rescheduling, provider failure, one history reconciliation, and one
  reattachment without a second session
- cancellation and deadline close active input, send `user.interrupt`, delete
  session then environment, release the credential, and join the scoped task
- confirmed and unconfirmed resource deletion remain separate terminal facts;
  provider bodies, remote ids, callback payloads, and secrets stay out of
  stable diagnostics
- Codex rejects a callback that races an already-started deadline stop without
  replacing the timeout with a connection failure

## Validation Evidence

- nine production-driver loopback tests and 181 affected-crate tests pass
- the Codex callback/deadline race test passes five repeated isolated runs and
  the complete 14-test app-server suite
- focused warnings-denied clippy and `git diff --check` pass
- `effigy doctor` reports only the inherited 19 oversized-file findings: 12
  warnings and 7 errors; this batch adds none
- no live account, external request, remote sandbox, or paid inference used

## Continuation Record

Card 079 is the sole ready task. Add the provider-neutral managed-harness
profile, run the same public seam under local and remote-authoritative hosts,
then close roadmap 025. No other continuation card is active.

## Evidence Required

- descriptor, preflight, success, callback, reconnect, cancel, deadline,
  failure, deletion, and cleanup assertions
- no-effect-before-preflight evidence
- exact request order and lease-release evidence
- safe diagnostic and redaction assertions

## Validation

- focused Anthropic driver and fixture tests
- focused warnings-denied clippy
- `git diff --check`

## Stop Conditions

- card 077 is incomplete
- production mapping exposes a missing shared contract or provider drift
- the driver cannot delete owned resources before releasing credentials

## Auto-Continuation

No. Promote card 079 only after the production lifecycle passes the frozen
corpus without widening the subset.
