# 043 Soundcheck Secondary Application Proof

Status: planned
Owner: Tom
Created: 2026-07-25
Milestone: `../014-consumer-scale-application-proof-and-hardening.md`

## Objective

Prove the hardened candidate through Soundcheck's normal catalogue and bounded
structured-run product path.

## Entry Gates

- card 042 complete
- current Soundcheck roadmap permits AI-runtime proof work
- exact 20-attempt provider ceiling plus screenshot, search, and test-data
  authority approved

## Scope

1. Use a fresh `SOUNDCHECK_LIBRARY_DB_PATH` across 4 native launches.
2. Start 16 normal product workflows: 8 baseline, 4 screenshot, 2
   search-enabled, 1 cancellation, and 1 controlled deadline.
3. Permit no more than 20 provider run attempts, including at most 4
   product-owned repair attempts, and stop after 2 hours.
4. Run one structured operation at a time.
5. Keep taxonomy, prompts, repair, review, and proposal application in
   Soundcheck.
6. Reduce Swallowtail failures to deterministic fixtures and replay.

## Acceptance Criteria

- [ ] the normal Soundcheck path uses the hardened candidate
- [ ] structured output and progress survive the accepted repeated workload
- [ ] 16 workflows stay within 20 provider attempts and 2 hours
- [ ] cancellation, deadline, failure, and cleanup remain distinct
- [ ] product validation and application remain downstream
- [ ] no live user library or DAW state changes without separate authority

## Stop Conditions

- active Soundcheck work overlaps the proof path
- proof would mutate unapproved product or DAW state
- provider spend or search authority exceeds the accepted envelope
- the structured-run path needs consumer policy inside Swallowtail
