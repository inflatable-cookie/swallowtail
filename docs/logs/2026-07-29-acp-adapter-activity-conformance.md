# 2026-07-29 ACP Adapter Activity Conformance

## Changed

- Claude Agent, Gemini CLI, and Kimi Code now decode ACP session updates
  through the shared bounded protocol boundary.
- Each adapter owns exact classification, stable operation-local identity,
  lifecycle completion, bounded content, provider references, and unknown
  namespaces.
- Message and thought compatibility events remain. Plans and tools no longer
  become empty progress.
- Gemini readable thought and operational-warning display remain distinct.
  Mode-display text does not become authoritative mode evidence.
- Prepared ACP operations now expose exact positive activity profiles bound
  to their qualified interface behavior revision.
- Permitted newer releases inherit the last qualified profile.

## Preserved

- ACP transport does not select provider identity.
- Raw input, raw output, and untyped metadata remain excluded.
- Permission and callback exchanges remain separate from harness-owned tool
  activity.
- Existing access, mode, retention, continuity, cancellation, deadline, and
  joined cleanup behavior remains unchanged.

## Evidence

- adapter projection unit tests: Claude Agent 1, Gemini CLI 1, Kimi Code 1
- ACP driver suites: Claude Agent 13, Gemini CLI 6, Kimi Code 5
- prepared facade suites: Claude Agent 12, Gemini CLI 4, Kimi Code 5
- prepared traces pass the shared observable-activity conformance assertion
- `effigy format:check`
- `effigy check:rust`
- `effigy package:api` — 23 crate declarations

Two standalone corpus binaries and the standalone remote-transport test
compiled, then remained blocked in macOS `_dyld_start` before libtest. They
were stopped. Shared decoder tests, adapter projection tests, exact-version
ACP driver suites, and both local and remote-authoritative host seams passed.

## Current State

Roadmap g02.037 and cards 125-127 are complete. Card 128 is the sole ready
task. Cards 129-137 remain in bounds.
