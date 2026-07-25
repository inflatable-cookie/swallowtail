# OpenCode Range Dispatch And Conformance Closeout

Date: 2026-07-23
Cards:
`../roadmaps/g01/batch-cards/122-opencode-range-dispatch.md`,
`../roadmaps/g01/batch-cards/123-opencode-range-conformance-and-closeout.md`

## Outcome

OpenCode HTTP now publishes a closed `opencode.server` claim from `1.14.48`
through `1.18.4`.

- configured instance, requirements, and immutable plan bind one exact release
- 18 private behavior revisions cover 45 stable releases in 20 published
  segments
- semantic gaps, prereleases, older, missing, and ambiguous versions reject
- `/global/health` must match before provider catalogue or session work
- created sessions must report the same exact release
- deny-first read-only policy, delegated authentication, external ownership,
  unknown-event stop, abort, deadline, redaction, and joined cleanup remain
  unchanged
- baseline and latest health checks pass under local and
  remote-authoritative host identities
- the provider-neutral attached-network profile remains the operation boundary

No Codex-specific record entered core, runtime, or testkit. Testkit only gained
an explicit delegated-authentication assertion for behavior it already
exercised.

## Validation

- focused OpenCode and testkit: 91 passed; one OpenCode installed probe gated
- full repository QA: 577 inventoried, 573 passed, four gated probes ignored
- workspace all-target check and warnings-denied clippy: passed
- documentation, formatting, and `git diff --check`: passed
- doctor: inherited 19 findings, 12 warnings and seven errors

The batch briefly raised doctor to 21 findings. Splitting health observation
and selection tests restored the inherited count before closeout.

## Continuation

- roadmap 040 and cards 120-123 are complete
- the original provider-coverage checkpoint was superseded on 2026-07-24 by
  the operator's forward-compatibility policy correction
- roadmap 041 and cards 124-127 now separate guaranteed qualified support from
  permitted unverified-newer execution before provider coverage resumes
- Kimi Code `0.29.0` is a candidate, not a selected policy

This later policy does not retroactively qualify newer OpenCode releases.
