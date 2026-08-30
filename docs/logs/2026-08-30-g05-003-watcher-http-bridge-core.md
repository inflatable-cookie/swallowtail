# 2026-08-30 g05.003 Watcher HTTP Bridge Core

Status: complete
Owner: Tom
Card: 016
Contract: 060
Research: 260

## Result

`HostServiceKind::WatcherBridge` is a separate optional port. Registration
binds nothing. Opening a local lease binds an ephemeral `127.0.0.1` HTTP/MCP
listener, fresh bearer material, exact host/operation/turn correlation, and
the same `WatcherHostService` used by operator controls. Model calls keep
requester identity `Model`. Admission freezes when the completion gate sees
no active or unjoined work. Close and drop freeze admission, join listener
and connection threads, stop and join turn-owned watchers, and release
private material.

The closed protocol admits initialize, initialized, tools/list, and the
reserved start/inspect/list/wait/stop plus completion-gate tools. Unknown,
malformed, oversized, unauthenticated, duplicate, cross-lease, and
post-freeze start requests fail before watcher work. Endpoint and bearer
stay driver-only and redacted.

No Claude adapter, MCP config file, Stop hook, container, public listener,
TLS, or generic MCP framework entered the diff.

## Evidence

- `swallowtail-core`: `HostServiceKind::WatcherBridge`
- `swallowtail-runtime`: object-safe `WatcherBridgeHostService`, redacted
  lease/endpoint/bearer, closed protocol constants
- `swallowtail-host-local`: loopback listener, bounded HTTP/MCP decode,
  completion barrier, joined cleanup; default composition registers the port
- `swallowtail-testkit`: portable redaction and closed-protocol assertions
- Unreleased public-api baseline updated

## Next

Orchestrator planning checkpoint. Cards 010-011 stay planned until current
Claude version evidence and the live same-turn gate close. Contract 029
currentness remains standing.
