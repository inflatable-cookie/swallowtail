# 2026-08-08 Control-Free History Wire Gate

Status: closed
Owner: Tom
Research: 115

## Decision

Claude Agent ACP, Kimi Code ACP, and Kimi local-server still lack a
control-free history or transcript wire. Contract 054 remains correctly
unsupported on those routes. No adapter mapping cards.

## Evidence

Local corpora and adapter surfaces only. ACP history still rides
`session/load`. Local-server has session lifecycle + WS activity catch-up,
not a messages/transcript API; reconciliation replay stays empty.

## Next Move

g03 evidence gate. Revisit when Claude/Kimi/stable ACP or local-server OpenAPI
corpora change.
