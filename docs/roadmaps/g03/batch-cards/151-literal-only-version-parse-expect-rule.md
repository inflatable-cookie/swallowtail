# 151 Literal-Only Version-Parse Expect Rule

Status: completed
Owner: Tom
Created: 2026-08-08
Milestone: `../050-provider-reachable-panic-closure.md`
Depends on: card 150

## Goal

Add a CI rule that keeps `InterfaceVersion::new(...).expect(...)` and its
axis sibling on string literals only, so a provider-reachable panic cannot
regress.

## Scope

1. Add a small source-scan check (extend an existing script or add one next
   to the other check scripts) that rejects `InterfaceVersion::new(` or
   `InterfaceVersionAxis::new(` followed by a non-literal argument within the
   same statement ending in `.expect(`.
2. Wire the check into an existing gate task in `effigy.toml` (for example
   `qa:docs` or a code-scan task) and into CI.
3. Verify the check passes the current tree and fails on a synthetic
   non-literal regression.

## Out Of Scope

- runtime behavior changes
- other panic classes outside version parsing

## Acceptance

- [x] the check runs in CI and fails a non-literal version-parse expect
- [x] the current tree passes the check

## Stop Conditions

- stop if the scan produces false positives on macros or generated code

## Auto-Continuation

Yes, to card 152 after acceptance.

## Validation

- run the new gate task; confirm the synthetic negative fails and the tree
  passes
- `effigy qa:docs`

## Completion Evidence

- new `scripts/check-literal-version-expects.py` source scan rejects
  `InterfaceVersion::new(...)` or `InterfaceVersionAxis::new(...)` with a
  non-literal, non-constant argument in any statement ending in `.expect(`,
  including the `.ok().expect` variant; comments, `#[cfg(test)]` modules,
  tests, examples, and the test-support testkit crate are excluded; string
  and raw-string contents are skipped so braces inside literals cannot
  corrupt parsing
- wired as `qa:code:version-expects` in `effigy.toml`, added to the `qa:docs`
  sequence, and to the CI `stable` route-and-guide-contracts step
- synthetic negatives fail (direct expect and `.ok().expect`), a comment
  containing the pattern does not false-positive, and the current tree
  passes
- the scan exposed sixteen latent binding-helper traps the earlier sweep
  had classified as static construction: private `version(value)` claim
  helpers across antigravity, claude-agent (both routes), codex, cursor,
  gemini, grok, kimi (both routes), oh-my-pi, opencode, pi, and qwen, plus
  the kimi local-server segment helpers and the muse release binding
- those sixteen helpers now return `Option` (or callers expect on literal
  calls), matching the card-148 total-helper pattern; test-module shadow
  helpers stay test-only
- focused rounds for the twelve touched adapters, workspace nextest (1,495
  passed), examples, format, and warnings-denied clippy all pass
