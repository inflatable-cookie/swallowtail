# 2026-07-31 Cursor ACP Interactive Driver

## Changed

- promoted Research 076 from exact installed Cursor source and the existing
  initialize-only capture
- froze source chunk identities and a normalized interactive ACP corpus
- added a separate `cursor-agent.acp` descriptor and compatibility claim
- implemented joined ACP v1 stdio transport, new sessions, text turns,
  streaming updates, cancellation, deadlines, and cleanup
- projected assistant, thought, correlated provider-tool, plan, and task-list
  activity without raw tool input or output
- preserved provider-owned local login and ambient read-write workspace access
  without a credential lease or authentication request
- added deterministic local-authoritative, remote-authoritative, permission,
  cancellation, disconnect, malformed-negotiation, activity, and cleanup tests

## Boundary

The route is qualified only for installed Cursor Agent
`2026.07.01-41b2de7`. Later dates remain visible as unverified newer.

The driver does not claim load, list, resume, delete, image input, consumer MCP,
model selection, callback exchange, or child topology. Cursor's separate
catalogue remains the only model-discovery route. No live prompt or provider
mutation ran.

## Validation

`effigy validate:focused swallowtail-protocol-acp swallowtail-adapter-cursor`
passed 109 tests across six binaries plus warnings-denied clippy in one second.

## Current State

Card 012 is complete. Cursor now has separate production catalogue and ACP
interactive drivers. Card 013 is the sole next task and owns the independent
headless stream-JSON structured driver.
