# 043 Soundcheck Secondary Application Proof

Status: paused
Owner: Tom
Created: 2026-07-25
Milestone: `../014-consumer-scale-application-proof-and-hardening.md`

## Objective

Prove the hardened candidate through Soundcheck's normal catalogue and bounded
structured-run product path.

## Entry Gates

- [x] card 042 complete
- [x] current Soundcheck roadmap permits AI-runtime proof work
- [ ] exact transitive consumer source and proof tuple frozen
- [ ] deterministic isolated data opens through the native product path;
  deadline and safe attempt-evidence
  mechanisms exist
- [ ] exact 20-attempt provider ceiling plus screenshot, search, and test-data
  authority approved

## Gate Audit

The 2026-07-26 read-only audit stopped this card before consumer mutation or a
provider call.

Soundcheck is at commit
`656555e817782483a66be5566e759d1a789fea87` with an active uncommitted M11
tranche. Its sole next task is card 088. That card changes
`src-tauri/src/assistant_tagging.rs`, the same evidence, prompt, screenshot,
validation, and proposal path this proof must exercise. Cards 089 and 090 then
project and natively accept the combined REAPER inventory. The current
Soundcheck roadmap therefore does not permit a separate AI-runtime proof, and
the card's active-work stop condition is met.

The audit also found three proof-readiness gaps:

1. Every primary Soundcheck research turn sets external network and search to
   host-approved. The previous split of only 2 search-enabled workflows did
   not describe the product path honestly.
2. Product deadlines are fixed at 15 minutes for research and 3 minutes for
   repair, ranking, and companion turns. No bounded proof-only deadline control
   exists, so one controlled deadline cannot be reproduced inside the
   2-hour envelope.
3. `SOUNDCHECK_LIBRARY_DB_PATH` isolates the library and adjacent screenshots,
   but no deterministic assistant fixture seed or bounded safe attempt ledger
   exists. A workflow may add repair, ranking, or companion attempts after its
   primary research attempt, so UI workflow counts alone cannot prove the
   20-attempt ceiling.

The existing integration boundary is otherwise suitable: prepared Codex
catalogue and structured exec are separate; default selection is exact
`gpt-5.4-mini`, low reasoning; cancellation is request-scoped; schemas,
screenshots, search policy, deadlines, and cleanup remain explicit; taxonomy
validation and application stay in Soundcheck.

## Readiness Handoff

After Soundcheck reaches a clean M11 checkpoint, its own authority must decide
and implement one bounded proof-readiness batch:

1. freeze the exact Soundcheck, Swallowtail, Codex executable, host, access,
   model, reasoning, and no-fallback tuple
2. seed a fresh isolated database with an approved public/synthetic product
   corpus and 4 non-sensitive fixture screenshots
3. add a proof-only bounded deadline control without changing production
   defaults
4. expose a sanitized attempt ledger containing only request correlation,
   operation kind, terminal class, elapsed time, search observation, attachment
   presence, and cleanup outcome
5. prove fixture setup, evidence, and teardown deterministically before live
   approval

This is consumer test support, not a Swallowtail product-policy surface.

## Readiness Outcome

Soundcheck card 091 is complete from product baseline `7c135da` with
proof-support source `3566419a2e4abf7e83b629d4b7cd12ba33f8b84b`. Its
opt-in proof profile binds one marked absolute root and exact adjacent
database, preserves normal production deadlines, and permits only a bounded
proof override. The deterministic seed creates all 16 workload records and 4
non-sensitive screenshots.

The attempt ledger contains only request correlation, operation kind, terminal
class, elapsed milliseconds, search observation, attachment presence, and
cleanup outcome. It rejects unknown fields, duplicate correlation, invalid
classes, and more than 20 attempts. Guarded teardown passed.

Soundcheck health, docs QA, 24 frontend tests, and 176 Rust tests pass.
Swallowtail health and the complete offline QA suite pass. No provider call,
search, credential access, or subscription effect occurred.

The final entry gate remains open: operator approval for live effects. Start
with Soundcheck card 092's smaller ceiling of 5 provider attempts, 1 native
launch, and 30 minutes. Do not authorize the full envelope by implication.

## First Pilot Stop

The operator approved Soundcheck card 092's smaller envelope. Its first native
launch stopped before assistant execution because the isolated fixture was
schema v50 while the running app supported v48.

The candidate tuple had omitted Soundcheck's local path dependencies.
`soundcheck-library` had 51 worktree entries, including schema 49-50, and
Poodle had 16. Signal and Swallowtail were clean. The newly added proof binary
also made the normal Effigy development command ambiguous until the app binary
was passed explicitly.

No provider request, search, credential exchange, or subscription-backed model
call occurred. The ledger remained empty and guarded teardown removed the
proof root. Soundcheck card 091 is reopened. Do not resume live proof until
the dependency owners provide clean committed checkpoints, the full local
source graph is frozen, the normal selector launches the intended app, the
fixture opens natively offline, and the operator approves a fresh envelope.

## Scope

1. Use one approved fresh `SOUNDCHECK_LIBRARY_DB_PATH` and adjacent fixture
   screenshot root across 4 native launches.
2. Start 16 primary research workflows: 8 text-only ordinary, 4
   screenshot-backed ordinary, 2 ordinary workflows that must expose search
   progress, 1 cancellation, and 1 controlled deadline.
3. Authorize external search for all 16 primary research workflows. The 2
   explicit search cases require observable search progress; the other
   workflows may use their normal bounded research policy without making
   search occurrence an acceptance condition.
4. Permit no more than 20 provider run attempts: 16 primary research attempts
   plus at most 4 Soundcheck-owned repair, ranking, or companion attempts.
   Stop after 2 hours.
5. Run one structured operation at a time.
6. Keep taxonomy, prompts, repair, review, and proposal application in
   Soundcheck.
7. Confine every product mutation and screenshot to the approved isolated
   proof root.
8. Reduce Swallowtail failures to deterministic fixtures and replay.

## Acceptance Criteria

- [ ] the normal Soundcheck path uses the hardened candidate
- [ ] structured output and progress survive the accepted repeated workload
- [ ] 16 workflows stay within 20 provider attempts and 2 hours
- [ ] cancellation, deadline, failure, and cleanup remain distinct
- [ ] product validation and application remain downstream
- [ ] no live user library or DAW state changes without separate authority
- [ ] the sanitized ledger reconciles every provider attempt and cleanup
      outcome across all 4 launches

## Stop Conditions

- active Soundcheck work overlaps the proof path
- proof would mutate unapproved product or DAW state
- provider spend or search authority exceeds the accepted envelope
- the structured-run path needs consumer policy inside Swallowtail
