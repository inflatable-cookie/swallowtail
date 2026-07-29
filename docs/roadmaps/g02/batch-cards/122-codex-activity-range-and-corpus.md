# 122 Codex Activity Range And Corpus

Status: planned
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

- [ ] every guaranteed version segment has exact provenance
- [ ] app-server and exec remain separate interface shapes
- [ ] deprecated and replacement events have explicit milestones
- [ ] readable summaries and raw reasoning remain distinguishable
- [ ] unknown newer semantic items have a safe fixture
- [ ] fixture validation runs offline

## Validation

- focused Codex protocol and fixture tests
- `effigy qa:docs`
- `effigy check:rust`

## Stop Conditions

- Stop on undocumented identity or contradictory tagged-source behavior.
- Keep a thinner older-version profile instead of backfilling newer fields.

## Auto-Continuation

Continue to card 123 only when the full maintained range has exact activity
segments.

