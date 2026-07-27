# 064 Kimi Local Server Interactive Driver

Status: complete
Owner: Tom
Created: 2026-07-27
Milestone: `../020-kimi-code-local-server-route.md`

## Objective

Add a full Kimi interactive-session route over the provider-documented local
REST and WebSocket server without replacing or flattening Kimi ACP.

## Governing Refs

- cards 061-063
- Contracts 009-010, 012-014, 017, 023, 029, 033-034, 037-038

## Scope

1. Add create and resume through the local-server configured instance.
2. Map prompt submission, ordered streamed events, approvals, structured
   questions, interruption, terminal failure, and replay cursors.
3. Preserve Kimi WebSocket version `2` cursor, volatile-event, epoch-change,
   resynchronization, and connection-close semantics.
4. Return exact session and management bindings from the local-server route.
5. Keep Kimi provider tools, workspace authority, permission modes, and
   optional isolation visible and independent.
6. Add a prepared normal path plus unchanged low-level driver escape hatch.

## Acceptance Criteria

- [x] no provider-neutral `send_prompt` or generic option map
- [x] Kimi ACP and local-server sessions remain separate prepared choices
- [x] event loss and resynchronization cannot masquerade as ordered completion
- [x] cancellation, provider abort, transport loss, and handle close remain
      distinct
- [x] callback and approval authority is explicit
- [x] handle close preserves provider state
- [x] management runs only after interactive handle close
- [x] attached and owned cleanup is joined and credential-last

## Validation

- deterministic REST/WebSocket interactive corpus
- shared interactive-session conformance where semantically applicable
- attached and owned host topology matrix
- Kimi ACP full regression

## Stop Conditions

- the selected event subset cannot preserve Kimi lifecycle truth
- reconnect would require implicit replay or fallback
- callback or approval behavior cannot be represented without consumer policy
- provider-specific surface would leak through common runtime payloads

## Auto-Continuation

Yes. Continue to card 065 after interactive conformance passes.

## Evidence

- added a separate prepared local-server session route with exact create,
  resume, model, permission, version, access, and management bindings
- mapped REST prompt submission and WebSocket v2 lifecycle, ordered deltas,
  volatile events, cursors, epochs, resynchronization, disconnect, abort, and
  terminal truth
- mapped manual approvals and structured questions through the explicit
  consumer callback exchange; automatic mode rejects undeclared interaction
- preserved exact `0.28.1` behavior while qualifying the `0.29.0` profile and
  disabled-tool milestone
- covered attached and owned-foreground topology, post-close management,
  joined transport and child cleanup, and credential release
- admitted the scoped pump task before provider effects; task rejection cannot
  open a WebSocket or submit a prompt

## Validation Evidence

- full Kimi adapter: 56 deterministic tests passed; one live installed probe
  remained separately gated and ignored
- focused REST/WebSocket interactive corpus: 7 tests passed
- strict Kimi Clippy passed
- `git diff --check` passed
- Effigy doctor reported the known 9 oversized-file errors; no card 064
  interactive production file added an oversized-file finding
- no live provider call or authentication was used
