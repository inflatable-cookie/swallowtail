# 079 Claude Managed Agent Conformance And Closeout

Status: completed
Owner: Tom
Updated: 2026-07-21
Milestone: `../025-claude-managed-agent-remote-harness-proof.md`

## Objective

Add provider-neutral provider-managed-harness conformance, prove the Claude
driver under both host topologies, and close roadmap 025.

## Governing References

- Contracts 009-012, 014, and 022
- roadmap 025
- cards 077-078

## Scope

- tenth provider-neutral conformance profile for a provider-managed remote
  harness
- local and remote-authoritative host fixtures through the same public seam
- success, callback wait, rescheduling, disconnect reconciliation,
  interruption, cancellation, deadline, provider failure, deletion ambiguity,
  redaction, and joined cleanup
- no preview-as-output, implicit retry, second session, repository authority,
  live credential, external request, or paid inference
- architecture, roadmap, front-door, and log closeout after realized behavior

## Ordered Steps

1. Add the provider-neutral profile without Anthropic identity or HTTP details.
2. Run the production driver through local and remote-authoritative fixtures.
3. Prove resource ownership, event authority, callback correlation, recovery,
   deletion truth, cleanup ordering, and lease release.
4. Run focused and full repository validation.
5. Promote realized architecture and close roadmap, card, log, and front doors.

## Acceptance Criteria

- [x] the profile adds only provider-managed remote-harness assertions
- [x] local and remote-authoritative hosts pass the same public seam
- [x] ordinary harness, direct, SDK, ACP, and serving profiles remain unchanged
- [x] every terminal path joins local work before lease release
- [x] unconfirmed remote deletion remains degraded cleanup
- [x] full QA passes or failures are recorded honestly

## Completion Evidence

- added the tenth public provider-neutral profile for a provider-managed remote
  harness; it composes the common Contract 011 assertions without changing the
  prior nine profiles
- the profile binds resource-free structured harness execution, an exact
  provider agent, durable retention, managed recovery, one reattachment,
  run-scoped callbacks, per-resource deletion truth, and no process or working
  resource authority
- synthetic local and remote-authoritative hosts prove blocking and child work
  finish before credential release
- the production Anthropic driver passes the same public profile and completes
  one exact run under both opaque host, instance, and endpoint identities
- existing production fixtures cover callback wait, rescheduling, one history
  reconciliation, cancellation, deadline, provider failure, deletion
  ambiguity, redaction, remote cleanup, and joined handle close

## Validation Evidence

- 10 provider-neutral profile tests pass
- 2 managed-harness preflight tests pass
- 10 Anthropic managed-driver tests pass
- focused warnings-denied clippy passes
- full `effigy qa` passes with 330 tests; three installed/live probes remain
  gated
- `effigy doctor` reports only the inherited 19 oversized-file findings: 12
  warnings and 7 errors
- `git diff --check` passes
- no live credential, provider account, external request, remote resource, or
  paid inference was used

## Continuation Record

Roadmap 025 is complete. Return to the provider-coverage evidence checkpoint.
No implementation card is ready: Cursor still needs operator intent on
repository and remote-mutation authority, while remote ACP still needs stable
transport and reconnect authority.

## Evidence Required

- public conformance report for both host topologies
- focused adapter and profile test counts
- full repository QA result
- architecture and roadmap closeout links

## Validation

- focused adapter and provider-managed-harness conformance tests
- warnings-denied focused clippy
- `effigy qa`
- `effigy doctor`
- `git diff --check`

## Stop Conditions

- card 078 is incomplete
- the conformance profile would flatten provider recovery, event authority, or
  remote deletion into an existing lifecycle shape
- live provider access becomes required for default QA

## Auto-Continuation

No. Return to the roadmap 025 planning checkpoint after closeout.
