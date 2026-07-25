# 010 Codex Prepared Facade Conformance And Guidance

Status: completed
Owner: Tom
Created: 2026-07-24
Milestone: `../003-codex-prepared-integration-facade.md`

## Objective

Close the prepared Codex facade with deterministic conformance, safe
diagnostics, public guidance, and a consumer migration handoff.

## Governing Refs

- Contract 037 (active)
- Contracts 011-013, 023, 029, 032-034
- completed cards 008-009

## Scope

1. Exercise all prepared profiles under local and remote-authoritative hosts.
2. Cover qualified, deprecated, unverified-newer, missing, malformed,
   incompatible, drifted, cancelled, timed-out, and cleanup-failed paths.
3. Prove tools, callbacks, interruption, schemas, attachments, search, and
   bounded writes retain existing semantics.
4. Audit diagnostics and public formatting for secret and payload leakage.
5. Add concise getting-started and low-level escape-hatch documentation.
6. Prepare exact Nucleus and Soundcheck migration inputs without editing them.

## Acceptance Criteria

- [x] every prepared profile passes deterministic conformance
- [x] current low-level Codex conformance remains green
- [x] local and remote-authoritative host identities remain exact
- [x] safe diagnostic stages survive consumer projection
- [x] examples compile at the declared MSRV
- [x] normal integration no longer requires manual configured-instance or
      requirements construction
- [x] cards 011 and 013 have exact migration inputs

## Validation

- focused Codex, runtime, host-local, and testkit suites
- all Codex conformance and compatibility corpora
- docs, MSRV, API diff, formatting, and warnings-denied clippy
- `effigy qa`
- `effigy doctor` delta review
- `git diff --check`

## Evidence Required

- complete deterministic matrix
- public example and migration guide
- facade versus low-level parity record
- roadmap g02.003 closeout log
- consumer migration card readiness assessment

## Stop Conditions

- a prepared path loses an existing lifecycle or capability guarantee
- diagnostics leak raw target, environment, credential, or provider data
- public guidance needs consumer-specific policy
- deterministic fixtures cannot prove the normal path

## Auto-Continuation

No. Consumer repository mutation requires the exact downstream authority gate.
Rebaseline card 011 and card 013, then follow the authorized adoption order.

## Closeout

Completed 2026-07-24.

- All four prepared profiles execute under local and remote-authoritative host
  identities.
- Qualified, deprecated, excluded, malformed, missing, drifted, cancelled,
  timed-out, cleanup-failed, and unverified-newer paths remain distinct.
- The 89-test Codex suite retains callback, interruption, schema, attachment,
  search, bounded-write, cancellation, deadline, and joined-cleanup semantics.
- Prepared debug and failure projection exclude raw target, environment,
  operation, credential, and provider material.
- Public getting-started, explicit-limit, failure-stage, and low-level escape-
  hatch guidance is complete.
- Card 011 is ready with exact Nucleus migration inputs. Card 013 has exact
  Soundcheck inputs but remains planned behind the authorized Nucleus-first
  order.
- No Nucleus or Soundcheck file changed.
