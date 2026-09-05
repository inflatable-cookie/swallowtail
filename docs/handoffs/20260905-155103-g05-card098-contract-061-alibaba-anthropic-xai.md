---
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
---

# Card 098 — Contract 061 Alibaba, Anthropic, and xAI package completion

## Dispatch

- Launch record: dispatched after the g05.009 manifest promotion commit `dc0cc671d325186cbfd0172d1af606b99e9c0d19` at `2026-09-05T15:51:03+01:00`
- Planning base: `b874df63e4802ed3400f3bf162b488c2e27d0046`
- Card: `docs/roadmaps/g05/batch-cards/098-contract-061-alibaba-anthropic-xai-package-completion.md`
- Manifest: `docs/roadmaps/g05/009-contract-061-consumer-projection-realization.md`, section `Cards 097-099 Manifest`
- Worker branch: `worker/g05-card098-alibaba-anthropic-xai`
- Dedicated workspace: `g05-card098-alibaba-anthropic-xai`
- Coordinator: `af21d886-4053-4156-ae6a-e878dfb99985`

This is an implementation worker-pr-loop. Keep the worker and exact workspace
alive through independent review, merge, and closeout. The reviewer must run in
this same workspace and receive the exact PR head.

## Owned paths

- `crates/swallowtail-adapter-alibaba-model-studio/**`
- `crates/swallowtail-adapter-anthropic/**`
- `crates/swallowtail-adapter-xai/**`
- Owned additive API baselines under `release-baselines/public-api-0.4.1/`
- `CHANGELOG.md` `[Unreleased]` entry
- Card 098 `## Result`
- Append-only `PAPERCUTS.md` evidence if required by the card

## Forbidden paths and review oracle

Shared runtime, testkit, core, contracts, census, audit note, other adapters,
and reserved closeout surfaces are forbidden. Per-turn rows require retained
plan evidence and `ConsumerMediatedPerTurn`. Matrix-only rows are withheld at
construction. Ledgers must bind to real prepared-facade contributions in both
directions: every claimed emitted row is published by the named facade and
every withheld row is absent. Do not widen a production plan solely to reach a
ledger target; any such capability change needs separate disclosed evidence.

## Validation and closeout

Run the card-named focused/package/API/docs/routes/Northstar/god-file/diff
checks, push one reviewable PR, and stop for an independent cross-model review
in this workspace. Do not merge from the worker. After acceptance and green
checks, the coordinator merges, records the exact coverage, performs reserved
closeout, and notifies Chatterbox.

No credentials, live provider calls, tags, publications, or consumer
mutations are authorized.
