# 146 Installed Range Currentness And Corpus

Status: completed
Owner: Tom
Created: 2026-07-30
Milestone: `../044-installed-harness-range-maintenance.md`

## Goal

Select the next batched installed-harness range maintenance work from current
exact evidence.

## Scope

1. Probe installed versions without provider work.
2. Check current stable package records.
3. Reuse exact Codex `0.146.0` tagged-source evidence.
4. Compare OpenCode `1.18.4..=1.18.10` selected source and artifacts.
5. Separate client-protocol changes from downstream provider behavior.
6. Confirm contract fit and sequence implementation cards.

## Acceptance Criteria

- [x] installed and current stable points are recorded separately
- [x] Grok, Kimi, and Claude no-op range decisions are explicit
- [x] Codex `0.146.0` additive drift is frozen
- [x] every OpenCode stable point through `1.18.10` is inspected
- [x] OpenCode `1.18.8` unrelated OpenAPI drift remains visible
- [x] no common contract or API change is required
- [x] implementation scope and stop conditions are explicit

## Validation

- exact executable version probes
- npm package metadata
- tagged-source comparison
- docs QA
- `git diff --check`

## Stop Conditions

- Stop if a selected interface or authority boundary changed.
- Do not install, update, authenticate, prompt, or mutate provider state.
- Do not infer unpublished or prerelease compatibility.

## Auto-Continuation

Yes. Continue to card 147 after focused docs validation.

## Evidence

- [Research 071](../../../research/071-installed-harness-range-maintenance-selection.md)
- local Codex `0.146.0` and OpenCode `1.18.9` observations
- current stable Codex `0.146.0` and OpenCode `1.18.10` package records
- exact OpenCode tag commits and OpenAPI hashes through `1.18.10`
- unchanged selected OpenCode route, lifecycle, deletion, authorization, and
  test source
