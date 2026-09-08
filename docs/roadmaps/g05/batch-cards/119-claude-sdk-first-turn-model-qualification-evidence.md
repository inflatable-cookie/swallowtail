# 119 Claude SDK First-Turn Model Qualification Evidence

Status: complete; PR 268 merged at `cfb0b106`
Owner: Tom
Created: 2026-09-07
Updated: 2026-09-07
Milestone: `../029-claude-sdk-interactive-parity.md`
Depends on: `v0.4.3` at `cbd4ddc8`; the producer diagnosis of 2026-09-07 (read-only, at `dbbcd192`); Contract 053 amendment promoted with this card

## Goal

Make the first-turn `supported_model_rejected` diagnosable. Today the sidecar rejects when `catalogueAvailable && !supportedModels.includes(system.model)`, never compares the requested model to the effective one, and the evidence that would show why (which ids were in the catalogue, which id `system/init` reported) is dropped: the sidecar emits diagnostic records the pump discards (`sdk/connection/pump.rs`, `Diagnostic(_) => Ok(())`). Likeliest cause (H1): catalogue rows carry the alias value only while `system/init` reports a dated canonical id, and the real row lacks the defensively read but unfrozen `resolvedModel`.

## Scope

1. Sidecar: an optional bounded evidence body on the existing diagnostic record, emitted at stage `first-turn-model-qualification` and correlated with the exact code (`query_rejected` / `supported_model_rejected`) about to be raised. Fields: requested model id and effective (`system.model`) id, each at most 128 bytes; catalogue size at most 64; a fixed-length truncated digest of the sorted, de-duplicated catalogue ids; requested-membership and effective-membership booleans; the SDK query source; phase; declared and loaded SDK version strings; native version. Never the full catalogue, prompt or content, paths, credentials, or raw error text.
2. Pump: forward sidecar diagnostic records to the existing `DebugObservation` / `DiagnosticObserver` path under Contract 053's interface-version-or-qualification kind. Failure response shape, predicate, code, and order stay byte-identical; observer absence changes nothing.
3. Provider-free fixtures on the fake SDK: `AliasOnly` (rows carry alias, init reports canonical), `CanonicalOnly`, `BothIds`, `NeitherIds`, each with a recording observer asserting the evidence body; the existing `EmptySupportedModels` and `CanonicalModel` fixtures unchanged.
4. Guide section "Diagnosing a first-turn model rejection"; changelog `[Unreleased]`; additive baseline.

## Out Of Scope

Changing the predicate or admitting a rejected model; persisting observations; any live turn.

## Acceptance Criteria

- [ ] the evidence body reaches a registered observer on every first-turn rejection with the fields above and nothing more
- [ ] predicate, code, order, and failure shape unchanged (fixture-proven)
- [ ] the four new fixtures plus the two unchanged ones green
- [ ] Contract 053 amendment cross-referenced; guide, changelog, baseline; one PR

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check`
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy package:api`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: a first-turn model rejection can be explained from the observer record alone, without the catalogue, the prompt, or a path. Smallest counterexample: a rejection whose observation lacks the effective-membership boolean.

## Stop Conditions

Explaining a rejection needs the full catalogue or raw error text (return to Chatterbox for a redaction ruling).

## Auto-Continuation

No. Stop for exact-head review. No live tier; a later live gate needs fixtures first and separate operator authority; never read or replay the Desktop candidate.

## Result

Implemented on [PR 268](https://github.com/inflatable-cookie/swallowtail/pull/268).
The sidecar now emits bounded model-qualification evidence best-effort and the
pump forwards valid evidence to the existing `InterfaceVersion` observer path.
Invalid evidence is omitted or rejected privately; the existing
`supported_model_rejected` predicate, code, order, and failure response remain
unchanged. Provider-free alias-only, canonical-only, both-id, neither-id, and
diagnostic-write-failure proofs cover the accepted and fail-soft paths. Decoder
negative coverage rejects bad digests, oversized ids/counts, shape drift,
wrong source/phase, and evidence paired with the wrong diagnostic code.

Focused, affected-package, API, Northstar, formatting, diff, and docs checks
pass. No provider/native launch, credentials or auth state, live turn, Desktop
candidate, Card120/121 work, release, or tag action was taken. Exact-head review
remains the stop condition.
