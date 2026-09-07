# 132 Claude SDK Registered-Tool Real-Route Gate

Status: planned; ready when Desktop schedules it under its isolated-testing authorization
Owner: Tom
Created: 2026-09-07
Updated: 2026-09-07
Milestone: `../035-shared-harness-capability-and-producer-boundary.md`
Depends on: card 125 merged (`1cbc21ad`); Contract 063 Mediated Stdio Proxy Attachment live tuple; Desktop Coordinator `914728cd` as sole test owner

## Goal

Turn the Claude SDK registered-tool seam from "callable, live gate pending"
into a qualified capability by running the frozen live tuple once: SDK
`0.3.259` at its recorded digest, native `2.1.259`, pinned Node, JSONL v1,
proxy wire `swallowtail-registered-tool-mcp-v1`, exact MCP protocol,
`private-loopback-http` plus `mediated-stdio-proxy`, `strictMcpConfig` true,
`settingSources` empty, `allowedTools` omitted, one disposable tool.

## Scope

1. Swallowtail supplies the hand-off packet: exact tuple, courier build
   (feature `mediated-stdio-proxy`), expected frames, stop conditions
   (authority leakage, ambient MCP, lazy attach, auto respawn/retry,
   `canUseTool` bypass, ambiguous ids, incomplete cleanup), and the redacted
   capsule format.
2. Desktop runs it under its isolated-testing authorization; Swallowtail does
   not run it.
3. On pass: Contract 061 row moves from `Unqualified / real_route_gate_pending`
   to qualified for that exact tuple; matrix cells `registered_tools` and
   `consumer_tool_exchange` become available; guide updated.
4. On a typed failure: the code and capsule come back as evidence for the
   next producer card; the row does not move.

## Acceptance Criteria

- [ ] one real registered call through the provider-spawned courier with a result correlated back, under the frozen tuple
- [ ] no stop condition hit
- [ ] Contract 061 row and matrix cells updated only from the capsule

## Validation

- `effigy qa:routes`; `effigy qa:docs`; `git diff --check`

## Review Oracle

Invariant: the claim is exactly the tuple that ran. Smallest counterexample:
a row qualified from the fixture.

## Auto-Continuation

No.
