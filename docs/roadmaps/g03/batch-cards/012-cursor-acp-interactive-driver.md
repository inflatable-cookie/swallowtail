# 012 Cursor ACP Interactive Driver

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../005-cursor-installed-dual-route-foundation.md`
Depends on: card 011

## Goal

Implement Cursor's first-party ACP server as a bounded interactive-session
driver with exact negotiated capabilities.

## Scope

1. Launch the approved executable with the `acp` subcommand through joined host
   process services.
2. Implement ACP initialize, session creation, prompt turns, streaming updates,
   cancellation, interruption, and cleanup from the frozen corpus.
3. Project exact tool, assistant, plan, task, and child activity only where the
   Cursor transcript supplies it.
4. Preserve ambient harness configuration and delegated Cursor access.
5. Decline load, resume, deletion, consumer MCP, catalogue, callback, or model
   options not proven by the selected artifact.

## Acceptance Criteria

- [x] ACP v1 negotiation is exact and capability-gated
- [x] session and turn lifecycle are joined and cancellable
- [x] activity correlation is stable without raw provider payloads
- [x] access and ambient configuration remain explicit in preflight
- [x] unsupported lifecycle and callback operations fail predictably
- [x] local and remote-authoritative fixtures agree
- [x] focused Cursor and shared ACP validation passes

## Validation

- `effigy validate:focused swallowtail-protocol-acp swallowtail-adapter-cursor`
- focused ACP, activity, lifecycle, cancellation, and cross-host tests
- no broad workspace suite or live Cursor prompt

## Stop Conditions

- Stop if the exact artifact negotiates a materially different ACP surface.
- Stop if correlation requires exposing raw session or tool payloads.
- Do not infer a capability from the registry entry alone.

## Auto-Continuation

Completed. Continue to card 013.

## Result

Research 076 qualifies the installed `2026.07.01-41b2de7` ACP interactive
surface from exact initialize evidence and source-derived normalized protocol
fixtures. `CursorAcpDriver` now owns new sessions, text turns, assistant,
thought, tool, and plan activity, cancellation, optional turn deadlines,
provider-request observation, and joined process/resource cleanup.

The driver sends no authentication or model-selection request. Load, list,
resume, delete, image input, consumer MCP, and callback exchange remain
unclaimed. Raw provider tool input and output are not projected.

Focused validation passed for `swallowtail-protocol-acp` and
`swallowtail-adapter-cursor`; no live Cursor prompt ran.
