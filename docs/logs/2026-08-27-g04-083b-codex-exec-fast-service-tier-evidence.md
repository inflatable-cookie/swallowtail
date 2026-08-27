# 2026-08-27 g04.083b Codex Exec Fast Service-Tier Evidence

Status: complete
Card: 233
Research: 234

## Boundary

Evidence only. The worker updated this file, card 233, Research 234, and new
Codex-local frozen evidence. Shared planning and production code stayed unchanged.

## Outcome

Research 234 promotes an honest empty deliver-now set after orchestrator review.
Gate, config, catalog membership, billing split, and omission research remain
frozen. `features.fast_mode` gates tier dispatch; `/fast` is TUI-only; legacy
config `fast` normalizes to wire `priority`; five bundled-catalog slugs at
`0.149.1` advertise tier id `priority` as evidence-gated membership only.

Empty-set rationale: live ChatGPT catalog can replace the bundled catalog before
request construction; `get_service_tier` silently drops unsupported tiers to
`None`; current exec decoding does not observe returned `service_tier` before
provider work. Static slug checks plus `--config service_tier="priority"` cannot
prove `priority` survives catalog resolution on the exec path.

Omission preserves current exec argv, including the delivered verbosity surface.

Frozen evidence:
`crates/swallowtail-adapter-codex/tests/fixtures/evidence/exec-fast-service-tier-range.json`.

## Validation

- `effigy validate:focused swallowtail-adapter-codex` — pass
- `effigy qa:northstar` — pass
- `git diff --check` — pass

## Unresolved

Production binding not authorized. A future lane needs pre-prompt confirmation
that effective tier matches caller selection, or exact tagged proof that
`priority` survives live-catalog resolution before provider work.
