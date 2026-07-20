# Codex Exec Driver

Date: 2026-07-19
Status: recorded

## Result

g01 card 018 is complete.

- Current Codex CLI and manual evidence narrowed exec to a one-shot JSONL
  surface. Model discovery belongs to app-server.
- The runtime now transports opaque prompt/output content, binds structured
  runs to an opaque working resource, and exposes the exact preflight-bound
  target and model to the selected driver.
- `swallowtail-adapter-codex` launches `codex exec --json --ephemeral` in the
  read-only sandbox through host task and process ports.
- JSONL lifecycle and agent-message output normalize into runtime events and a
  final terminal result. Raw provider payloads remain private.
- Cancellation force-stops the owned child. Wait and close join process and
  task cleanup.
- Deadline, image, and schema inputs reject before process start until their
  host-authority boundaries exist.

## Evidence

- Full Effigy QA passes with 51 tests.
- Fake-process fixtures prove exact executable, model, environment, workspace,
  arguments, stdin, JSONL output, cancellation, wait, and join behavior.
- The adapter has no dependency on `swallowtail-host-local`, a consumer, or a
  concrete executor.

## Next Lane

Card 019 proves Codex app-server as a long-lived JSON-RPC interactive-session
driver with model discovery, provider references, turn lifecycle, and
independent session/turn interruption.
