# 050 Provider-Reachable Panic Closure

Status: planned
Owner: Tom
Created: 2026-08-08
Generation: g03
Depends on: g03.049
Vision tags: safe diagnostics, fail-closed, compatibility maintenance
Contract refs: 029, 037, 051
Planning state: cards 148-151

## Problem

A verified deep audit found provider-reachable panic traps and one panic class
that is only latent today:

- `ollama_runtime_binding` panics on a blank provider version string; it is
  reached from `parse_version` on a malformed `/api/version` response, while
  the 13 sibling adapters return `Option` and fail closed
  (`adapter-ollama/src/selection.rs:23`, `protocol/catalog.rs:70`)
- `codex_cli_binding` is the same trap, currently test-only callers
  (`adapter-codex/src/selection.rs:80`)
- `kimi-platform` uses `unreachable!()` on the shared two-variant `Payload`
  enum, so adding any third variant to
  `swallowtail-protocol-openai-chat` converts it into a provider-triggered
  panic without an adapter-side change
  (`adapter-kimi-platform/src/protocol/events.rs:53-55`)
- `anthropic` turn handling keeps a dead `Ok(AttemptOutcome::Tool(_)) =>
  unreachable!()` arm behind a prior conversion that must stay in sync by hand
  (`adapter-anthropic/src/driver/session/turn.rs:202-243`)
- 1,816 `.expect()` sites across adapters with no tooling preventing a future
  provider-reachable one; the blank-version panic is the proof it happens

## Goals

- [ ] make version-binding helpers total: no adapter panics on observed
      provider text
- [ ] fail closed wherever a shared enum can grow
- [ ] remove dead unreachable arms that must be kept in sync by hand
- [ ] add a CI rule that keeps version-parse expects on literals only

## Execution Plan

- [ ] Execute card 148 (total version-binding helpers and blank-version
      regression).
- [ ] Execute card 149 (fail-closed exhaustiveness and dead-match removal).
- [ ] Execute card 150 (provider-reachable expect sweep).
- [ ] Execute card 151 (literal-only version-parse expect rule).

## Boundaries

- no public API, diagnostic-code, classification, or version-range change
- no provider, transport, or consumer behavior change
- no tag, release, registry publication, or live provider work
- panic-free guarantee applies only to provider-observable paths

## Acceptance Criteria

- [ ] blank and whitespace-only provider versions fail closed with a version
      diagnostic across every adapter with an observed version axis
- [ ] shared-enum growth cannot reach a provider-triggered panic
- [ ] no dead unreachable arm remains behind an upstream conversion
- [ ] the CI rule fails any non-literal `InterfaceVersion::new(...).expect`
- [ ] focused and workspace test rounds pass

## Next Planning Checkpoint

The suite planning checkpoint after g03.051: reassess evidence-gate posture
before the scaffolding extraction tranches.
