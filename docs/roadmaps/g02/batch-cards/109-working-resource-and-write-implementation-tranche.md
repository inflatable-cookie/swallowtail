# 109 Working Resource And Write Implementation Tranche

Status: completed
Owner: Tom
Created: 2026-07-28
Milestone: `../032-working-resource-and-workspace-authority-feature-closure.md`
Depends on: card 108

## Objective

Implement only the contract-ready resource and bounded-write routes selected by
cards 107-108.

## Scope

1. Add exact prepared inputs and capability claims for selected routes.
2. Bind resource scope, host identity, access, topology, and version before
   effects.
3. Preserve ambient, provider-enforced, and host-enforced authority truth.
4. Reject path, scope, access, and callback drift before provider work.
5. Join process, network, callback, and cleanup work before releasing leases.
6. Change matrix cells only after focused conformance passes.

## Acceptance Criteria

- [x] every converted cell has a public prepared path
- [x] raw paths or provider records cannot mint resource authority
- [x] write and containment outcomes never strengthen
- [x] cancellation and cleanup failures remain visible
- [x] focused exact-range conformance passes offline

## Evidence

- `GeminiSessionProfileInput::bounded_write` is the explicit public prepared
  path. Existing `new` remains read-only.
- Plan capabilities, session policy, resource resolution, ACP negotiation,
  process mode, returned mode, and callback dispatch all agree on `ReadWrite`
  before effects.
- The write callback passes only a bounded redacted locator and UTF-8 content
  through `WorkingResourceIo`; host path authorization remains host-owned.
- Both profiles remain `AmbientHost` with explicit ambient configuration.
  No sandbox, shell, provider-tool, approval, or containment claim was added.
- Six focused ACP and four prepared-facade tests pass, including
  unnegotiated-write rejection before host mutation.

## Auto-Continuation

Continue to card 110 only after every selected cell has deterministic
production evidence.
