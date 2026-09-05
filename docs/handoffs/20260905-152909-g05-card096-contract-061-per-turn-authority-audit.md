---
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
---

# Card 096 — Contract 061 per-turn authority audit

## Dispatch

- Planning base: `ba8275ebf7623724ec61398acd1117d9c5c08c9a`
- Card: `docs/roadmaps/g05/batch-cards/096-contract-061-per-turn-authority-audit.md`
- Manifest: `docs/roadmaps/g05/009-contract-061-consumer-projection-realization.md`, section `Card 096 Manifest`
- Worker branch: `worker/g05-card096-contract-061-per-turn-authority-audit`
- Dedicated workspace: `g05-card096-contract-061-per-turn-authority-audit`
- Coordinator: `af21d886-4053-4156-ae6a-e878dfb99985`

This is a planning-only implementation worker-pr-loop. Keep the worker and
its exact workspace alive through independent review, merge, and closeout.
The reviewer must run in this same workspace, be independently routed, and
receive the exact PR head. Do not compile or implement any runtime card from
the audit.

## Owned paths

- `docs/roadmaps/g05/batch-cards/096-contract-061-per-turn-authority-audit.md`
- Exactly one new `docs/triage/YYYYMMDD-HHMMSS-contract-061-per-turn-authority-audit.md`
- Append-only `PAPERCUTS.md`, only if the audit requires it

## Forbidden paths and surfaces

- Every crate and all Rust source/tests/build files
- Contracts, architecture, the Contract 061 census, and the Batch 9.4 note
- Any other triage note or shared closeout surface
- Provider credentials, live probes, or implementation work for candidates
  B, K, or L

## Required audit oracle

Classify every per-turn and attachment row across:

- Candidate B — Alibaba, Anthropic, xAI: 76 rows
- Candidate K — Mistral Vibe, Muse, Oh My Pi, Qwen: 52 rows
- Candidate L — OpenCode, Pi: 69 rows

Every row needs a code anchor and a reason. Rule whether the existing
`ConsumerMediatedPerTurn` posture and projection vocabulary are sufficient,
or draft one additive shared baseline verbatim with names, admission rules,
and assertions. Provide a rubric verdict for each candidate and recommend
the first promotion; Chatterbox expects L. Do not silently promote any
candidate or change shared vocabulary.

## Validation and closeout

Run the named `effigy qa:docs`, `effigy qa:northstar`, and `git diff --check`
gates. Keep the diff zero Rust and inside the three owned surfaces. Push one
reviewable PR and stop for the independent cross-model reviewer in this same
workspace; do not merge from the worker. After acceptance and green required
checks, the coordinator merges, performs reserved closeout, and notifies
Chatterbox. Card 096 is concurrent with Card 081 and the Card 094 remainder.

No credentials, tags, publications, or consumer mutations are authorized.
