# 087 Claude SDK Qualified Ranges

Status: stopped; Research 287 admits no range; five exact QualifiedOnly pins unchanged
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-07
Milestone: `../029-claude-sdk-interactive-parity.md`
Depends on: card 086 research promoted; Contract 029 segment and unverified-newer rules; the Codex range precedent in `crates/swallowtail-adapter-codex/src/selection.rs`

## Goal

Move the five `claude-agent.sdk` axes from qualified-only exact pins to Codex-style qualified ranges (baseline, latest qualified, explicit unpublished gaps) with stable-newer allowed as `UnverifiedNewer`, exactly as card 086 proves.

## Scope

1. `sdk/selection.rs`: per axis, replace the one-point claim with baseline, latest-qualified, and gap constants on the Codex shape; keep claim ids stable where the behaviour revision is unchanged, and mint `-window-2` ids where card 086 shows a surface change.
2. Discovery reports `Qualified`/`Deprecated`/`UnverifiedNewer`/incompatible per Contract 029; open under `UnverifiedNewer` runs provisionally and is labelled in open evidence.
3. Identity tests per axis boundary; the Research 280 identity test updated to the range.
4. Guide, matrix version cells, changelog `[Unreleased]`, additive baseline.

## Out Of Scope

Widening beyond what card 086 proved; the wire and sidecar axes unless card 086 shows they moved.

## Acceptance Criteria

- [ ] every range endpoint cites card 086 evidence — **stopped**: Research 287 Decision table is empty; no endpoint is admitted
- [ ] gaps listed explicitly; nothing bulk-bumped from `latest` — **stopped**: no range to gap; pins unchanged; `latest` unused
- [x] identity tests lock the current exact-pin boundaries and Research 287 withheld points as unqualified
- [x] guide, matrix, changelog, and baseline unchanged; one PR for the stop

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check`
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy package:api`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: a version is qualified only where a probe proved it. Smallest counterexample: a range endpoint with no card 086 line.

## Stop Conditions

Card 086 found a surface change inside the intended range (return to Chatterbox for a segment ruling).

## Auto-Continuation

No. Stop for exact-head review.

## Result

Stop. Research 287 admits no candidate range, so Card 087 does not invent
Codex-style windows, gaps, or `AllowUnverified` stable-newer. The five
`claude-agent.sdk` axes stay qualified-only exact pins:

| Axis | Withheld claim | Evidence |
| --- | --- | --- |
| SDK wrapper range | no `0.3.259` window and no `UnverifiedNewer` for `0.3.260` | Research 287, line 131 |
| Bundled native range | no `2.1.259` window | Research 287, line 132 |
| Host native `2.1.258` | not interchangeable with bundled `2.1.259` | Research 287, lines 97-98, 133, 147 |
| Node `22.23.2` as a range | open-only initialize controls exist; `system/init` does not | Research 287, lines 134, 79-81 |
| Node `26.7.0` | refused-with-code/unresolved; not compatible | Research 287, lines 70, 82-84, 135 |
| Native `2.1.227..=2.1.258` | not installed; not probed | Research 287, lines 99-100 |
| Harness schema as a range axis | artifact metadata only | Research 287, lines 136, 87-88 |
| Wire and sidecar | unchanged; Card 086 shows no movement | Research 287; card Out Of Scope |

The precise blocker is evidence, not planning: Card 086 required
`system/init`, `supportedModels`, and runtime `harnessSchema` per triple,
and `system/init` is the first query message (Card 100, lines 191-197). No
new live turn was run. Production claims, pins, guide, matrix, changelog,
and API baseline are unchanged. Claim ids stay `*-window-1` / `wire-v1` /
`sidecar-v1`. Reopen when Chatterbox authorizes a live first-query capture
that fills Research 287's Decision table, or when an initialize-only
surface can supply those fields without a turn.
