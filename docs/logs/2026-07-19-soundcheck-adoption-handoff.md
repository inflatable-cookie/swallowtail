# Soundcheck Adoption Handoff

Date: 2026-07-19
Status: recorded

## Result

g01 card 025 and roadmap 007 are complete.

- current Soundcheck sources were inspected read-only; no consumer file changed
- a generic adapter integration fixture proves the full reusable seam through
  public Swallowtail APIs without product types
- current Codex evidence added the controlled-automation flags required for a
  host-approved temporary non-Git resource and non-interactive execution
- model catalog translation now preserves safe description and
  provider-default evidence alongside reasoning metadata
- the downstream plan targets model discovery and `run_codex_turn` mechanics
  only
- the plan specifies an exact Git revision, temporary compile feature, host
  adapters, per-turn policies, consumer acceptance, rollback, and prompt legacy
  removal

Product evidence, prompts, schemas, validation, repair, ranking, progress
wording, review, persistence, and application remain Soundcheck-owned.

## Evidence

- `crates/swallowtail-adapter-codex/tests/structured_run_parity.rs`
- `docs/roadmaps/g01/soundcheck-adoption-handoff.md`
- current `codex-cli 0.144.6` command help
- current Codex manual automation guidance
- full Effigy QA: 78 tests pass

## Next Lane

Roadmap 008 starts with card 026: promote concrete provider-neutral session
options, tool declarations, callback requests/responses, wait states,
cancellation, and deadline semantics before changing the Codex app-server
driver.
