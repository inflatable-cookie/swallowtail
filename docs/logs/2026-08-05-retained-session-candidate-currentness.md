# 2026-08-05 Retained Session Candidate Currentness

Roadmap: `../roadmaps/g03/037-retained-session-recovery-promotion.md`
Card: 097

## Changed

- revalidated Pi RPC through maintained `0.83.0`
- kept Pi persistent recovery blocked on public cwd binding and corroboration
- revalidated Alibaba conversation retrieval, ordered item listing,
  continuation, retention, and separate deletion operations
- selected a distinct retained Alibaba profile for contract promotion
- superseded Pi implementation card 099 and made Alibaba contract card 098
  ready
- split settled-restoration tests that had added structural findings in the
  preceding batch

## Current State

Pi exposes ordered history and an internal cwd override, but public RPC does
not bind or report effective cwd. It cannot attach a persisted session to an
exact host-leased resource.

Alibaba can retrieve an exact conversation, page its ordered items, and
continue it independently of stored response objects. This supports contract
work for a separate retained profile. The existing operation-owned
delete-on-close profile remains unchanged. Production support is not yet
claimed.

No authenticated provider work, provider prompt, remote conversation mutation,
installed harness invocation, or model inference ran.

## Validation

- Pi and Alibaba focused validation: 67 passed
- runtime and OpenCode focused validation after test-module splitting: 243
  passed
- structural scan returned to the known 219-finding baseline, including 21
  errors; `effigy doctor` remains red on existing oversized-file debt
- `git diff --check`

## Next Move

Execute card 098. Promote the retained Alibaba ownership, attachment, replay,
readiness, and cleanup contract plus deterministic corpus before implementation.
