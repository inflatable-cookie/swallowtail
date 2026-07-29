# 122 Codex Activity Range And Corpus

Status: completed
Owner: Tom
Created: 2026-07-29
Milestone: `../036-codex-observable-activity-fidelity.md`
Depends on: card 121

## Goal

Freeze exact Codex activity-schema milestones across the maintained exec and
app-server range before production mapping.

## Scope

1. Revalidate current official app-server and exec event documentation and
   tagged source.
2. Reuse the existing maintained executable range and identify every activity
   schema milestone inside it.
3. Freeze fixtures for:
   - item start and completion
   - assistant commentary and final answer
   - plans and plan replacement
   - reasoning-summary boundaries and deltas
   - command output and completion
   - file changes and turn diffs
   - MCP, dynamic, and collaborative tool calls
   - search, image, review, compaction, tasks, and hooks
   - approvals and request resolution
   - exec completion-only records
   - deprecated, additive, unknown, malformed, and foreign events
4. Record exact disclosure and version exclusions.
5. Make no production behavior change.

## Out Of Scope

- narrowing the maintained range
- installed or authenticated live probes
- production mapping
- raw provider payload exposure

## Acceptance Criteria

- [x] every guaranteed version segment has exact provenance
- [x] app-server and exec remain separate interface shapes
- [x] deprecated and replacement events have explicit milestones
- [x] readable summaries and raw reasoning remain distinguishable
- [x] unknown newer semantic items have a safe fixture
- [x] fixture validation runs offline

## Result

- Promoted Research 064 from current official docs, every relevant stable
  source tag through `0.145.0`, npm history, and the new stable `0.146.0`
  release.
- Kept the guaranteed upper bound at `0.145.0`; stable `0.146.0` remains a
  permitted unverified-newer attempt without activity-profile widening.
- Froze separate app-server and exec schema segments with exact tag commits.
- Froze rich app-server cases for messages, plans, readable reasoning
  summaries, commands, files, tools, collaboration, search, image, review,
  compaction, hooks, approvals, resolution, unknowns, malformed events, and
  foreign ownership.
- Froze exec's per-kind lifecycle truth: messages, reasoning, files, and
  warnings are completion-only; commands, MCP, search, collaboration, and
  todo lists retain the fuller lifecycle actually emitted.
- Recorded `thread/compacted` and legacy file-output replacements without
  backfilling newer events into older releases.
- Reconciled every older Codex compatibility corpus with the maintained-range
  policy: `0.146.0` is explicit unverified-newer evidence, not a rejection.
- Kept raw reasoning text, raw response events, credentials, and provider
  envelopes outside portable disclosure.

## Validation

- focused Codex activity, current compatibility, legacy app-server, and legacy
  exec corpus tests — 13 passed
- compatibility, lifecycle, and legacy version-policy regression tests — 15
  passed
- provider-neutral observable-activity conformance — 1 passed
- `effigy format:check`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy check:rust` — every workspace crate and target compiled
- `effigy doctor` — restored the unchanged 111 findings: 83 warnings and 28
  errors

## Stop Conditions

- Stop on undocumented identity or contradictory tagged-source behavior.
- Keep a thinner older-version profile instead of backfilling newer fields.

## Auto-Continuation

Continue to card 123. The full qualified range now has exact activity segments
and Contract 044 requires no change.
