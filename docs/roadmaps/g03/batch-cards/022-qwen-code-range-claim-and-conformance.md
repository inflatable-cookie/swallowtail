# 022 Qwen Code Range Claim And Conformance

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../009-qwen-code-installed-range-closure.md`
Depends on: card 021

## Goal

Extend Qwen Code's maintained compatibility claim through `0.21.2` using the
frozen interval and behavior revisions from card 021.

## Scope

1. Extend the Qwen package-axis claim without moving baseline `0.19.11`.
2. Dispatch adapter-private behavior only where a frozen milestone requires it.
3. Align discovery, configured instance, preflight, activity, catalogue,
   structured run, restarted continuity, and facade evidence.
4. Cover boundaries, milestone neighbors, representative interiors,
   exclusions, prereleases, and one later stable unverified point.
5. Preserve exact observed version and no-fallback behavior.

## Acceptance Criteria

- [x] `0.19.11..=0.21.2` membership matches the frozen corpus exactly
- [x] every segment carries the correct maintained behavior revision
- [x] installed `0.21.2` is qualified, not silently relabelled
- [x] later stable versions remain permitted but unverified
- [x] unsupported and malformed points fail before provider work
- [x] focused Qwen tests pass
- [x] card 023 becomes the sole ready and next task

## Validation

- `effigy validate:focused swallowtail-adapter-qwen`
- `effigy package:verify-affected swallowtail-adapter-qwen`
- `git diff --check`
- no broad workspace suite

## Auto-Continuation

Yes. Continue to card 023 after focused and extracted-package proof.

## Evidence

- the claim contains maintained `0.19.11..=0.20.1` baseline behavior and
  `0.21.0..=0.21.2` catalogue-filter behavior
- exact preflight version now drives stream `session_start` validation
- latest-boundary structured execution completes only with matching `0.21.2`
  stream evidence
- 35 focused Qwen tests passed with warnings denied
- the extracted 60-file package compiled
- no live provider or workspace effect ran
