# 087 Claude SDK Qualified Ranges

Status: planned; ready on card 086's admitted ranges
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-06
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

- [ ] every range endpoint cites card 086 evidence
- [ ] gaps listed explicitly; nothing bulk-bumped from `latest`
- [ ] identity tests at each boundary
- [ ] guide, matrix, changelog, baseline; one PR

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
