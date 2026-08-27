# 2026-08-27 g04.082a Codex App-Server Model Verbosity Evidence

Status: complete
Card: 228
Research: 229

## Boundary

Evidence only. The worker updated this file, card 228, Research 229, and new
Codex-local frozen evidence. Shared planning and production code stay unchanged.

## Outcome

Research 229 promotes an honest empty deliver-now set for app-server
`model_verbosity` on exact `0.147.0`, `0.148.0`, `0.149.0`, and `0.149.1`.

Decisive gaps:

- no typed verbosity on `thread/start`, `turn/start`, or `thread/settings/update`
- no verbosity metadata on `model/list`
- no verbosity in `ThreadSettings` or persisted `ThreadMetadata`
- only generic `config.model_verbosity` map and `config/read` exposure
- unsupported models warn and omit Responses `text.verbosity` after session bind
- current Swallowtail app-server bytes omit verbosity on open and turn start

Exec Research 213 remains exec-only. No app-server verbosity binding is
authorized.

## Changed Files

- `docs/research/229-codex-app-server-model-verbosity-evidence.md`
- `docs/roadmaps/g04/batch-cards/228-codex-app-server-model-verbosity-evidence.md`
- `crates/swallowtail-adapter-codex/tests/fixtures/evidence/app-server-model-verbosity-range.json`
- this log

## Validation

- `effigy validate:focused swallowtail-adapter-codex` — passed
- `effigy qa:northstar` — passed
- `git diff --check` — passed

## Next Move

Orchestrator review and serial merge per g04.082 lane A ordering.
