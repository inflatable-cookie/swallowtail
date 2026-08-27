# 2026-08-27 g04.083b Codex Exec Fast Service-Tier Evidence

Status: complete
Card: 233
Research: 234

## Boundary

Evidence only. The worker updated this file, card 233, Research 234, and new
Codex-local frozen evidence. Shared planning and production code stayed unchanged.

## Outcome

Research 234 promotes a non-empty deliver-now table: twenty explicit Fast
dispatch rows on exact published `0.147.0`, `0.148.0`, `0.149.0`, and `0.149.1`
maintained suppressed exec for `gpt-5.6-sol`, `gpt-5.6-terra`, `gpt-5.6-luna`,
`gpt-5.5`, and `gpt-5.4`. Wire tier is `priority`; catalog tier id is
`priority`; legacy config `fast` normalizes to `priority`. `features.fast_mode`
gates dispatch and defaults enabled; `/fast` is TUI-only. ChatGPT-subscription
and API-key profiles are separate dispatch authorizations with documented
billing split. Provider acceptance, returned effective tier, billing
realization, and observed latency stay withheld.

Introduction points frozen from tags: `service_tier` config at `0.115.0`,
`fast_mode` gate at `0.118.0`, request wiring at `0.125.0`, bundled Fast
catalog rows at `0.131.0`, `gpt-5.6-*` Fast rows at `0.147.0`. Omission
preserves current exec argv, including the delivered verbosity surface.

Frozen evidence:
`crates/swallowtail-adapter-codex/tests/fixtures/evidence/exec-fast-service-tier-range.json`.

## Validation

- `effigy validate:focused swallowtail-adapter-codex` — pass
- `effigy qa:northstar` — pass
- `git diff --check` — pass

## Unresolved

Production binding not authorized. Live catalog replacement and provider
returned tier remain withheld admission authorities.
