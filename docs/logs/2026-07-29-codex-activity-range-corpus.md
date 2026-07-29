# 2026-07-29 Codex Activity Range Corpus

## Changed

- Revalidated current Codex app-server and exec activity evidence against
  official docs, tagged source, npm history, and releases.
- Promoted Research 064.
- Froze separate offline app-server and exec activity corpora.
- Removed the stale `0.146.0` rejection from current and legacy compatibility
  corpora; each now records the release as permitted unverified-newer.
- Split the card 121 testkit assertion module before it became new structural
  debt.

## Current State

- The guaranteed Codex upper bound remains `0.145.0`.
- Stable `0.146.0` is permitted unverified newer and cannot widen the
  guaranteed activity profile.
- App-server core activity exists from `0.80.0`; later source milestones add
  richer message, plan, tool, hook, patch, timestamp, and subagent truth.
- Exec exposes mixed lifecycle fidelity by item kind. It is not one
  app-server-equivalent stream and not uniformly completion-only.
- Raw reasoning, raw response events, credentials, and provider envelopes
  remain excluded.
- Contract 044 is sufficient. Production mapping has not changed.

## Evidence

- `crates/swallowtail-adapter-codex/tests/fixtures/activity/range.json`
- `crates/swallowtail-adapter-codex/tests/fixtures/activity/app-server.jsonl`
- `crates/swallowtail-adapter-codex/tests/fixtures/activity/exec.jsonl`
- 13 focused Codex corpus tests
- 15 compatibility, lifecycle, and version-policy regression tests
- one provider-neutral observable-activity conformance test
- workspace format, compile, docs, and Northstar checks
- doctor restored to the known 111 findings after the testkit module split

## Next

Card 123 maps the qualified app-server corpus into portable observable
activity and exact prepared route profiles. Card 124 remains in bounds for
the separate exec projection and roadmap closeout.
