# 231 Codex 0.147.0 Range Corpus

Status: completed
Owner: Tom
Created: 2026-08-17
Milestone: `../072-recurring-version-currentness-and-codex-0-147.md`
Depends on: card 230; Research 120; Research 127

## Goal

Freeze exact Codex CLI `0.147.0` identity and protocol evidence against the
current `0.146.0` qualified bound, and name whether `0.147.0` is a compatible
extension or a behavior milestone.

## Scope

1. Record npm `@openai/codex@0.147.0`, local `codex-cli 0.147.0`, and the
   `codex.cli` claim through `0.146.0` with existing gaps.
2. Compare app-server and exec protocol evidence to the `0.146.0` corpus,
   using Research 120 schema notes and the g03.047/048 live repairs.
3. Name the segment shape for card 232: same behavior revision through
   `0.147.0`, or a new private milestone, or stop with the bound unchanged.
4. Keep production claims at `0.146.0`.

## Out Of Scope

- editing `selection.rs` claims or public matrices
- Grok, Claude, or other 127 families
- provider prompts, install, update, or publication
- new portable operations

## Acceptance Criteria

- [x] exact `0.147.0` package and CLI identity is recorded
- [x] exec and app-server surfaces are classified separately if their
      evidence diverges
- [x] card 232 has an explicit segment decision
- [x] no claim membership changes in this card

## Validation

- fixture or schema comparison named in the card evidence
- `effigy qa:northstar`

## Stop Conditions

- stop if `0.147.0` is no longer the official stable point
- stop if qualification would require a provider prompt
- stop if a new public operation or contract beyond 029 is required

## Auto-Continuation

Continue to card 232 once the segment shape is named.

## Evidence

- Research 128
- `crates/swallowtail-adapter-codex/tests/fixtures/compatibility/codex-0-147-range.json`
- Segment decision: compatible extension of existing exec, app-server,
  lifecycle, and thread-catalogue revisions through exact `0.147.0`. Not a
  new milestone. After qualification, synthetic later-stable is `0.148.0`.
