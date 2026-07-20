# Codex App-Server Tool Callbacks

Date: 2026-07-19
Roadmap: g01/008 Nucleus Interactive-Session Readiness
Card: 027 Codex App-Server Tool Callbacks

## Evidence

The installed Codex CLI `0.144.6` generated its experimental app-server JSON
schema into a temporary directory outside the repository. The current protocol
accepts `developerInstructions` and `dynamicTools` on `thread/start`, reasoning
`effort` on `turn/start`, and server-initiated `item/tool/call` requests with
`threadId`, `turnId`, `callId`, `tool`, and `arguments`. Dynamic-tool responses
carry `success` and `contentItems`. `thread/resume` accepts developer
instructions but has no dynamic-tool declaration field.

## Realized Boundary

- exact preflight bindings gate developer instructions, reasoning, tools,
  schema dialect, schema size, and declaration count before provider work
- only declared tools enter the bounded consumer callback stream
- provider call ids remain adapter-private; runtime callback and turn ids stay
  distinct and redacted
- the callback request event and exchange item share one event sequence
- consumer responses are correlated exactly once to callback and turn
- cancellation and deadline expiry abandon pending callbacks, safely reject
  provider waits, interrupt the provider turn, and close the consumer stream
- malformed, unsupported, undeclared, late, duplicate, mismatched, and invalid
  callback traffic fails explicitly without exposing raw payloads
- Swallowtail does not execute tools or inherit consumer authority

Tool-enabled resume is rejected before process work until provider schema
evidence offers a declaration route. Developer instructions and reasoning on
resume remain available.

## Validation

Scripted app-server fixtures cover success, undeclared calls, duplicate and
late responses, cancellation, deadline expiry, and resume rules. Focused
integration tests and strict workspace Clippy passed before the full repository
validation round.
