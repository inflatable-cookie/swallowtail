# 2026-08-27 g04.082d Ollama Attached Think Max Evidence

Status: complete
Card: 231
Research: 232

## Boundary

Evidence only. Updated card 231, Research 232, this log, and new Ollama-local
frozen evidence. Shared planning and production code unchanged.

## Worktree

- path: `/Users/tom/.t3/worktrees/swallowtail/t3code-1db36e7c`
- branch: `t3code/review-ollama-think-evidence`
- base: `origin/main` at `5ffb16153ecde00fb79ee23654fb60e4899be48f`

## Outcome

Promoted Research 232 with an honest empty deliver-now set for native
`think: "max"` on `ollama.attached`.

Key findings:

- `"max"` entered the tagged `ThinkValue` JSON parser at `v0.22.0`; the
  immediate stable predecessor `v0.21.2` remains byte-identical to `v0.20.0`
  for `api/types.go` and rejects it at unmarshaling.
- From `0.22.0` through `0.32.15`, the wire parser accepts `"max"`, but
  `/api/show` exposes only generic `capabilities: ["thinking"]`.
- The prepared route binds that generic capability only; it does not bind
  parser, template, or level lists for reasoning admission.
- Tagged `server/routes.go` silently maps `"max"` to `"high"` on harmony/gpt-oss
  paths before dispatch.
- Production preparation still admits only `off|low|medium|high` and rejects
  `max` before chat dispatch; interactive sessions expose no reasoning selector.

Frozen corpus:
`crates/swallowtail-adapter-ollama/tests/fixtures/ollama-think-max-v0.14.0-v0.32.15/`.

## Validation

- `effigy validate:focused swallowtail-adapter-ollama` — passed (67 tests)
- `effigy qa:northstar` — passed
- `git diff --check` — passed

## Next

Orchestrator review and serial integration after lanes A–C merge.
