---
title: g05.009 Card 034 Kimi package completion worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-04
updated: 2026-09-04
handoff_path: /Users/tom/Dev/projects/swallowtail/docs/handoffs/20260904-210500-g05-card034-kimi-package-completion.md
base_required: pushed-main
tags: [coordination, handoff, worker, pr, g05, contract-061, kimi]
---

# g05.009 Card 034 Contract 061 Kimi Package Completion

## Objective

Implement the ready Card 034 Candidate F tranche in
`swallowtail-adapter-kimi` and `swallowtail-adapter-kimi-platform`, using the
accepted Card 076 compound-acknowledgement design and the landed Card 073
provider-operation observation view.

## Scope

Own only the Card 034 manifest paths: the two Kimi adapter packages and their
tests, the named prepared guide and matrix cells, Card 034's result, the two
additive public API baseline files, and append-only `PAPERCUTS.md`. Shared
runtime, testkit, core, contracts, other adapters, the Kimi gate note, census,
Kimi version claims, and coordinator-reserved closeout surfaces are forbidden.

## Execution

Fetch origin before preflight. Deliver the card's exact 89-row, 75-emitted /
14-withheld result; distinct prepared and active source IDs; outcome-backed
catalogue observation; compound acknowledgement with terminally undispatched
Plan truth; the adapter-local 128-byte token bound; and the specified case 2
and case 4 failures. Regenerate the two owned API baselines through
`scripts/generate-public-api-baseline.sh` and confirm additive-only changes.
Use provider-free fixtures and no credentials.

## Validation and review

Run the manifest-named focused, affected-package, API, docs, Northstar,
god-file, formatting, and diff gates. Push one reviewable PR against `main`
and request an independent cross-model exact-head review in this same worker
workspace. Do not merge.

## Handoff

Report the exact PR head, owned-path diff, baseline additivity, validation, and
any deviation from the card. Coordinator owns the merge gate, reserved
closeout, and frontier recomputation.
