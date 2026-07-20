# Codex App-Server Driver

Date: 2026-07-19
Status: recorded

## Result

g01 card 019 is complete.

- Current Codex evidence narrows the first proof to the stable local stdio
  app-server surface. WebSocket and experimental methods remain excluded.
- `swallowtail-adapter-codex` exposes model-catalog and interactive-session
  roles without merging them into the exec driver.
- JSONL-RPC correlation, initialization, paginated model listing, thread
  open/resume, turn start, output streaming, completion, interruption, and
  cleanup stay private to the adapter.
- Runtime session and turn ids remain separate from opaque provider references.
- The proof fixes read-only/no-approval policy. Unsupported callbacks and
  unowned inputs fail explicitly rather than hanging or escalating authority.

## Evidence

- Full Effigy QA passes with 58 tests.
- Scripted bidirectional fixtures cover model pages, session open/resume,
  output deltas, final output, active-turn interruption, whole-session
  cancellation, callback rejection, and pre-side-effect input rejection.
- The Contract 011 long-lived RPC profile and exact descriptor claims pass.
- Effigy doctor reports no errors after splitting turn state and the scripted
  app-server fixture along focused module boundaries.

## Next Lane

Card 020 audits both Codex drivers together. It proves their shared runtime
vocabulary does not erase their different capabilities, lifecycle, or input
support.
