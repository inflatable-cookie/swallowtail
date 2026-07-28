# Anthropic Input, Tool, And Search Exchange

Date: 2026-07-28
Status: completed

## Changed

- prepared structured inference accepts one operation-scoped `image/png`
  attachment up to 1 MiB and emits Anthropic's exact base64 image source
- explicit provider search uses `web_search_20250305`, at most two uses, and
  optional host-approved domain restrictions
- a separate resource-free interactive role provides direct client-tool
  continuation without widening the one-attempt structured role
- a client-tool request closes its HTTP/SSE attempt before consumer wait; one
  exact correlated result authorizes the next attempt
- bounded private message history permits one later user turn and zeroizes on
  close
- attachment, connection, and credential cleanup remain ordered and joined

## Evidence

- frozen request records and synthetic tool/search SSE fixtures cover exact
  request bodies, correlation, duplicate rejection, later-turn history,
  progress events, and release order
- `cargo test -p swallowtail-adapter-anthropic`: 50 passed
- `effigy lint:rust`: passed
- `effigy check:examples`: passed
- no live provider credential was used

## Matrix

- Anthropic Messages `attachments`, `consumer_tool_exchange`, and
  `external_search` move from `No` to `Yes`
- `interactive_session` also moves to `Yes` because direct client-tool
  continuation is now a distinct supported operation
- the matrix now records 437 `No` cells; the input/callback family records 68

## Next

Card 091 re-audits the 74 starting cells, runs packaged and documentation
gates, and selects the next evidence-ranked feature family.
