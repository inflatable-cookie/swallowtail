# 012 Cursor ACP Interactive Driver

Status: planned
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

- [ ] ACP v1 negotiation is exact and capability-gated
- [ ] session and turn lifecycle are joined and cancellable
- [ ] activity correlation is stable without raw provider payloads
- [ ] access and ambient configuration remain explicit in preflight
- [ ] unsupported lifecycle and callback operations fail predictably
- [ ] local and remote-authoritative fixtures agree
- [ ] focused Cursor and shared ACP validation passes

## Validation

- `effigy validate:focused swallowtail-protocol-acp swallowtail-adapter-cursor`
- focused ACP, activity, lifecycle, cancellation, and cross-host tests
- no broad workspace suite or live Cursor prompt

## Stop Conditions

- Stop if the exact artifact negotiates a materially different ACP surface.
- Stop if correlation requires exposing raw session or tool payloads.
- Do not infer a capability from the registry entry alone.

## Auto-Continuation

Yes. Continue to card 013 after focused ACP validation passes.

