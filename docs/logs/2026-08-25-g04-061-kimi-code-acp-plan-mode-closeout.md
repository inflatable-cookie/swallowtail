# 2026-08-25 g04.061 Kimi Code ACP Plan Mode Closeout

Status: complete
Owner: Tom
Milestone: g04.061
Cards: 170-172
Research: 208

## Outcome

Delivered portable `HarnessMode::Plan` on route `kimi-code.acp` for exact
`0.28.1` and exact `0.29.0..=0.38.0` under the existing ACP reasoning
revisions. No new behavior revision. Selection remains new-session-only and
requires current `mode` snapshot membership plus response `currentValue=plan`.
Provider `default|auto|yolo` rows may coexist; they are not public selections
and do not widen permission. Isolation stays `AmbientHost`. Plan mode is
prompt-and-tool policy, not containment. Load, resume, import, and recovery
gain no harness-mode mutation.
PR 60 fast-forwarded the exact reviewed green head `f21220cd` to `main`.

## Evidence

- Research 208 promoted with first qualified complete milestone at `0.28.1`,
  preceding published boundary `0.28.0` outside the qualified ACP window, and
  byte-identical ACP `modes.ts` through `0.38.0`
- focused fixtures cover omission, plan-only dispatch, reasoning composition,
  missing/ambiguous/malformed/unknown-row snapshots, confirmation drift,
  provider rejection, joined cleanup, load/resume rejection before host
  effects, visible `UnverifiedNewer`, and composed plan membership against
  the reasoning confirmation snapshot rather than the stale `session/new`
  list
- Kimi prepared guide and feature-matrix notes distinguish plan mode,
  permission posture, and ambient isolation

## Validation

Hosted Stable, documentation/API, pinned MSRV, dependency, and source-consumer
jobs passed on exact head `f21220cd`. Independent review reran 121 focused Kimi
tests and affected-package verification successfully. `git diff --check`
passed. Doctor remained at the inherited 378 findings: 332 warnings and 46
errors, plus the inherited stale graph and generated-source warnings.

## Generation Boundary

g04.061 closes only this route-local family. g04 remains open for the next
per-route inventory reassessment unless the operator supplies a different
direction. Contract 029 currentness stays standing.
