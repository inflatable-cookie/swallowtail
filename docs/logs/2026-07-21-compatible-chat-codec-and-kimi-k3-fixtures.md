# 2026-07-21 Compatible Chat Codec And Kimi K3 Fixtures

## Changed

- added `swallowtail-protocol-openai-chat` with bounded structural request JSON,
  SSE fragmentation, comments, data records, `[DONE]`, common envelopes, and
  explicit unknown fields
- migrated llama.cpp build-9910 request and stream parsing onto the common codec
  without changing its provider, deployment, cancellation, usage, or serving
  semantics
- froze a synthetic Kimi Platform K3 manifest, catalogue, exact request,
  reasoning/output/usage stream, safe errors, unknown, model mismatch, and
  disconnect corpus
- recorded the current Kimi documentation discrepancy: the dedicated guide and
  API schema support `low`, `high`, and `max`; one stale embedded paragraph says
  only `max`
- completed card 084 and advanced card 085 to ready

## Boundary

The common crate owns wire structure only. K3 reasoning, cached-token usage,
fixed-parameter omission, returned-model agreement, catalogue flags, errors,
access, and exclusions stay in the dated Kimi fixture layer. llama.cpp keeps
its exact text-only and serving-ownership decisions.

## Validation

- 37 focused protocol and llama.cpp tests pass
- independent llama.cpp and Kimi success streams pass one fragmented decoder
- warnings-denied focused clippy and all-target workspace compile pass
- doctor remains at the inherited 19 findings: 12 warnings and 7 errors
- no credential, external request, paid inference, or live catalogue was used

## Continuation

Card 085 is the sole ready task. Implement the separately registered Kimi
Platform catalogue and K3 direct driver. Card 086 remains in bounds for hosted-
direct conformance and roadmap closeout.
