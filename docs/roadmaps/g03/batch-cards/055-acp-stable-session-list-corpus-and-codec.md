# 055 ACP Stable Session List Corpus And Codec

Status: completed
Owner: Tom
Created: 2026-08-01
Milestone: `../021-acp-session-list-and-kimi-import.md`
Depends on: card 054

## Goal

Freeze stable ACP `session/list` and add a bounded common codec without
claiming that every ACP agent supports catalogue or import.

## Scope

1. Freeze the stable capability, request, response, cursor, cwd, title,
   timestamp, and `_meta` shapes.
2. Add request correlation and bounded candidate projection to the shared ACP
   codec.
3. Reject malformed, oversized, unsupported, and cross-request responses.
4. Preserve unknown extension fields without exposing raw payloads.
5. Keep list capability independent from load, resume, and deletion.

## Out Of Scope

- a production adapter claim or provider-specific import
- arbitrary `_meta` projection or raw cwd diagnostics
- protocol features outside stable session listing

## Acceptance Criteria

- [x] the stable list schema is represented by deterministic fixtures
- [x] list dispatch requires explicit negotiated capability
- [x] candidates and cursors are bounded and request-scoped
- [x] unsupported or malformed replies fail closed
- [x] no load or resume authority follows from a list result
- [x] focused ACP tests pass
- [x] card 056 becomes the sole ready and next task

## Validation

- `effigy validate:focused swallowtail-protocol-acp`
- `git diff --check`
- no provider prompt or broad suite

## Auto-Continuation

Yes. Continue to card 056 after common ACP acceptance.

## Evidence

- Research 094 rechecks current stable ACP v1 and records the independently
  gated additive `additionalDirectories` field
- the pinned normalized corpus covers capability, request, response, cwd,
  cursor, title, RFC 3339 update time, `_meta`, and additive extensions
- the shared codec requires negotiated list support, correlates the exact
  JSON-RPC request, bounds every projected field, and rejects resource drift,
  duplicates, malformed timestamps, malformed metadata, and oversized pages
- opaque extension values survive protocol decoding but remain absent from
  public accessors, `Debug`, and diagnostics
- 93 focused protocol tests and extracted-package verification pass
