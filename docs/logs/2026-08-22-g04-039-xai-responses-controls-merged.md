# 2026-08-22 g04.039 xAI Responses Controls Merged

## Change

- accepted PR 38 at exact head `e9ae1a49a90a32c9242eaec0b64d80c3050d2e40`
  after all five CI jobs passed
- fast-forwarded `main` to that head without a merge commit
- reconciled the deferred xAI architecture, route matrix, feature matrix,
  changelog, programme, indexes, and roadmap state

## Result

`xai.responses-websocket` now exposes exact model-qualified reasoning on
structured and serial connection-local profiles: `low`, `medium`, and `high`
for `grok-4.5`; those values plus `xhigh` for `grok-4.6`. Both exact models
also expose positive `max_output_tokens` through `2_147_483_647`.

The controls remain fixed through continuation and fresh context-losing
replacement. The claim stops at dispatched request shape; it does not claim
provider acceptance, effective reasoning depth, or exact generated length.

The initial five-family per-route feature sequence is complete. The remaining
advanced-route inventory stays promoted for one-family-at-a-time selection.

## Next

Reassess the remaining inventory against current production-matrix and
contract truth. Select one route and one coherent control family, then compile
g04.040 and its cards before starting implementation.
