# Northstar Worker Handoff

- Role: worker
- Lane: g05.009 Card 068, Contract 061 llama.cpp and Ollama package completion
- Dispatch provenance: operator-confirmed direction relayed by Chatterbox
- Promoted planning commit: `c2145d8972f4f3eb5c90c132c3a5545f2b092c79`
- Manifest: `docs/roadmaps/g05/009-contract-061-consumer-projection-realization.md`, `Card 068 Manifest`
- Card: `docs/roadmaps/g05/batch-cards/068-contract-061-llama-cpp-ollama-package-completion.md`
- Base requirement: fetch `origin` before preflight; work from current `origin/main`

## Assignment

Implement the exact candidate J Contract 061 tranche: 35 rows across
`llama-cpp.attached` (10), `llama-cpp.owned` (6), and `ollama.attached` (19),
with 32 emitted and 3 construction-time withheld. Add the established
adapter-local contribution shape to the six named prepared facades, author
deterministic route and per-shape Ollama fixtures, and stop after one
reviewable PR.

## Boundaries

Owned paths:

- `crates/swallowtail-adapter-llama-cpp/**`
- `crates/swallowtail-adapter-ollama/**`
- `CHANGELOG.md` `[Unreleased]`
- this card's `## Result`
- `PAPERCUTS.md` append only

Forbidden: core, runtime, testkit, every other adapter, contracts,
architecture, census CSV, and the candidate J audit note. Do not add provider
credentials, live probes, currentness work, active observation, or public
vocabulary. Preserve attached/owned and Ollama operation-shape distinctions.

## Required closeout

Return a self-contained worker capsule naming the exact branch, worktree,
commit, changed paths, row ledgers, withheld reasons, validation results, and
PR URL. Notify on finish. The coordinator will perform exact-head independent
review and merge gating; do not merge or edit reserved shared closeout
surfaces.
