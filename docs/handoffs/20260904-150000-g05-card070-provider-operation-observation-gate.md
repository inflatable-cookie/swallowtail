# Northstar Worker Handoff

- Role: worker
- Lane: g05.009 Card 070, Contract 061 provider-operation observation public-baseline gate
- Dispatch provenance: operator-confirmed direction relayed by Chatterbox
- Promoted planning commit: `62d22f5d`
- Manifest: `docs/roadmaps/g05/009-contract-061-consumer-projection-realization.md`, `Card 070 Manifest`
- Card: `docs/roadmaps/g05/batch-cards/070-contract-061-provider-operation-observation-gate.md`
- Base requirement: fetch `origin` before preflight; work from current `origin/main`

## Assignment

Produce exactly one planning-only triage gate note and fill Card 070's `##
Result`. Define additive provider-operation observation vocabulary, admission,
composition, failure, fixed maxima, two proving-consumer anchors, a drafted
Contract 061 amendment, drafted testkit assertions, and the readiness-rubric
verdict. Cover DeepSeek Harness local-server rows 44/45 and the Kimi session
catalogue row; record OpenCode HTTP only as a third carrier anchor without
auditing candidate L. Zero Rust.

## Boundaries

Owned paths:

- this card's file
- exactly one new `docs/triage/YYYYMMDD-HHMMSS-contract-061-provider-operation-observation-gate.md`
- `PAPERCUTS.md` append only

Forbidden: every `crates/**` path, contracts, architecture, census, Kimi gate
note, candidate I note, other triage notes, provider contact, and fixture
edits. Do not promote the amendment or compile a runtime card. Existing
session-scoped vocabulary must remain unchanged in meaning.

## Required closeout

Return a self-contained capsule naming exact branch/worktree/head, the single
triage note, every proposed name and evidence anchor, all open decisions, and
`effigy qa:docs`, `effigy qa:northstar`, and `git diff --check` results. Notify
on finish. The coordinator will review and reconcile shared closeout surfaces;
Chatterbox owns later promotion.
