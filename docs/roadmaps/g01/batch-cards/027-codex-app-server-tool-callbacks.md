# Codex App-Server Tool Callbacks

Status: completed
Owner: Tom
Roadmap: 008 Nucleus Interactive-Session Readiness
Updated: 2026-07-19

## Goal

Implement declared Codex dynamic-tool callback transport over the app-server
driver without granting tool execution authority.

## Scope

- dynamic-tool declaration translation
- correlated `item/tool/call` request and response
- callback wait, cancellation, timeout, and terminal failure
- session instructions and reasoning selection
- model reasoning metadata
- unsupported callback rejection retained for all other methods

## Out Of Scope

- executing tools
- approval or filesystem callbacks
- Nucleus product records
- unrestricted provider permissions

## Acceptance Criteria

- only declared tool callbacks reach the consumer
- the provider cannot hang on unsupported or abandoned callbacks
- callback and turn completion remain independently observable
- callback content and provider payloads remain redacted by default
- the long-lived RPC profile retains every common assertion

## Validation

- bidirectional scripted app-server fixtures
- callback cancellation and cleanup fixtures
- `effigy qa`
- schema-evidence revalidation

## Stop Condition

Stop if Codex callback shape has changed or requires an uncontracted authority.

## Closeout

- Codex CLI `0.144.6` generated app-server schema evidence fixes
  `developerInstructions` and `dynamicTools` on `thread/start`, `effort` on
  `turn/start`, and `item/tool/call` request/response shapes.
- exact reasoning and tool declarations are validated against the preflight
  plan before process work; inline JSON Schema declarations remain bounded.
- a bounded callback exchange maps provider call ids to distinct runtime
  callback and turn ids, emits matching ordered events, and enforces exactly-
  once response correlation.
- cancellation, deadline expiry, malformed calls, undeclared tools, abandoned
  waits, and invalid response content receive safe provider failures. Late and
  duplicate consumer responses fail explicitly.
- Swallowtail transports callbacks but contains no generic tool executor.
- current `thread/resume` schema has no dynamic-tool field. Tool-enabled resume
  therefore fails before provider work; developer instructions and reasoning
  remain supported.
- scripted bidirectional fixtures, focused integration tests, workspace tests,
  Clippy, formatting, and docs QA provide closeout evidence.
