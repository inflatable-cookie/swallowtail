# 194 Idioms Route Opt-In Binding And Codex Proof

Status: completed
Owner: Tom
Updated: 2026-08-09

## Goal

Bind the idioms opt-in into prepared plans, gate it by route capability, and
prove folded delivery on Codex app-server.

## Scope

- immutable prepared-plan binding of source identity and maximum
- `idioms_session_option` capability advertisement and fail-closed
  preflight rejection on non-advertising routes and plan mismatches
- Codex app-server deterministic proof of the folded developer-instructions
  delivery

## Out Of Scope

- Nucleus adoption (card 195)
- other route proofs beyond the capability gate

## Acceptance Criteria

- [x] plan mismatch, missing source, and non-advertising route reject before
      provider work
- [x] Codex app-server fixture proves folded delivery without live provider
      work
- [x] default behavior unchanged on every other route

## Validation

- [x] `effigy validate:focused swallowtail-idioms swallowtail-runtime
      swallowtail-testkit swallowtail-adapter-codex` — 467 tests pass
- [x] `effigy package:verify-affected swallowtail-idioms swallowtail-runtime
      swallowtail-testkit swallowtail-adapter-codex` — extracted package
      proof passes
- [x] `cargo fmt --check` and warnings-denied clippy pass
- [x] end-to-end Codex proof: prepared plan binds `IdiomsSessionOption`, and
      the scripted `thread/start` payload carries consumer instructions
      followed by the labeled `[idioms]` folded block
- [x] runtime resolver fails closed with `IdiomSourceUnavailable` when the
      opt-in has no registered source
