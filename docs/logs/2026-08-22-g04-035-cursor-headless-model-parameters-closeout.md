# g04.035 Cursor Headless Model Parameters Closeout

Date: 2026-08-22
Milestone: g04.035
Cards: 095, 096, 097
Status: complete

## Result

`cursor-agent.headless` is the first per-route feature milestone:

- Card 095 promoted Research 183 with exact secret-free evidence across all
  four qualified builds, a deliver-now tuple allowlist, and evidence-gated rows.
  The CLI catalogue exposes plain model ids only; SDK parameter descriptors
  stay a sibling surface.
- Card 096 added typed adapter-local APIs (`CursorHeadlessFast`,
  `CursorHeadlessContext`, `with_fast`, `with_context`, `with_effort`) on
  `CursorHeadlessModelSelection`, preserved the plain-model path, rejected raw
  bracket grammar, and bound qualified effort to portable `ReasoningSelection`
  for high only on `claude-opus-4-8` and `claude-opus-5`.
- Card 097 proved canonical single-argument `--model` dispatch, unchanged
  plain-model behavior, and fail-closed rejection of unqualified tuples before
  process work.

Deliver-now tuples:

- `claude-opus-4-8` — `context=1m`, `effort=high`, `fast=false`
- `claude-opus-5` — `context=300k`, `effort=high`
- `composer-2.5` — `fast=false`

Fast and context remain Cursor-local selected-model parameters. The route
claims qualified dispatch only; provider acceptance and effective application
remain separate states under Contract 040. Cursor ACP and catalogue behavior
are unchanged.

## Validation

- Cards 095-097 ran `effigy validate:focused swallowtail-adapter-cursor`,
  `effigy package:verify-affected swallowtail-adapter-cursor`, and
  `effigy check:examples` over the affected adapter surface.
- Deterministic preparation, driver, and corpus tests cover every deliver-now
  tuple, combined selections, plain-model compatibility, and failure
  boundaries without provider calls or account inspection.

## Next

Compile Ollama attached `num_ctx` as the next numbered per-route feature
milestone. Contract 029 currentness remains standing.
