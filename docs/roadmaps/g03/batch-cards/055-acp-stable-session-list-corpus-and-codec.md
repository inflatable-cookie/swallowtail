# 055 ACP Stable Session List Corpus And Codec

Status: planned
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

- [ ] the stable list schema is represented by deterministic fixtures
- [ ] list dispatch requires explicit negotiated capability
- [ ] candidates and cursors are bounded and request-scoped
- [ ] unsupported or malformed replies fail closed
- [ ] no load or resume authority follows from a list result
- [ ] focused ACP tests pass
- [ ] card 056 becomes the sole ready and next task

## Validation

- `effigy validate:focused swallowtail-acp`
- `git diff --check`
- no provider prompt or broad suite

## Auto-Continuation

Yes. Continue to card 056 after common ACP acceptance.
