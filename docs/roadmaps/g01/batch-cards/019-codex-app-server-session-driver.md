# Codex App-Server Session Driver

Status: completed
Owner: Tom
Roadmap: 006 Codex Proof Drivers
Updated: 2026-07-19

## Goal

Extract a long-lived Codex app-server driver for interactive sessions and
turns.

## Scope

- local stdio launch, initialization, and model catalog
- JSON-RPC request, response, and notification correlation
- open, resume, turn, interruption, close, and cleanup lifecycle
- provider reference opacity and extension isolation
- applicable Contract 011 profile

## Out Of Scope

- Nucleus tasks, goals, memory, tools, approvals, receipts, or persistence
- `codex exec`
- generic ACP implementation
- WebSocket, Unix-socket, remote attach, and daemon transports
- experimental app-server methods or fields
- server-initiated approvals, tools, elicitations, and attestation callbacks
- write-capable or unrestricted sandbox posture

## Acceptance Criteria

- provider and Swallowtail ids remain distinct
- session and active-turn cancellation scopes work independently
- model catalog, thread, and turn calls remain correlated across interleaved
  responses and notifications
- unsupported server requests fail explicitly rather than hanging
- close joins reader, writer, and child work
- long-lived RPC profile and claimed capability assertions pass

## Validation

- generated schemas from the installed Codex CLI and current Codex manual
- deterministic scripted JSONL-RPC process fixtures
- focused adapter and Contract 011 long-lived RPC profile tests
- `effigy qa`
- `effigy doctor`
- `git diff --check`

## Closeout

- `swallowtail-adapter-codex` now exposes separate model-catalog and
  interactive-session roles over a local stdio app-server process.
- initialization, paginated model discovery, thread open/resume, turn start,
  streamed output, completion, interruption, and cleanup remain isolated in
  the adapter.
- runtime ids remain separate from opaque provider thread and turn references.
- session and active-turn cancellation have independent scopes. Session close
  joins the reader task and child process.
- the first proof fixes read-only/no-approval policy and rejects deadlines,
  attachments, structured output, and server callbacks it cannot safely own.
- WebSocket, remote attachment, experimental methods, and callback handling
  remain outside this proof.
