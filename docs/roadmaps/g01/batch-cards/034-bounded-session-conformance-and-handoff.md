# 034 Bounded Session Conformance And Handoff

Status: completed
Owner: Tom
Updated: 2026-07-20
Milestone: `../010-bounded-workspace-session-access.md`

## Objective

Prove the portable bounded-session seam and publish the exact Nucleus consumer
mapping before downstream transport replacement.

## Acceptance

- [x] public-API local and remote-host conformance passes
- [x] read-only regression coverage passes
- [x] observed wait states and cleanup are deterministic
- [x] Nucleus identity/outcome mapping is documented without product types
- [x] full Swallowtail QA passes

## Evidence

- `app_server_workspace` runs bounded open against local and
  remote-authoritative host identities.
- `session_access_policy` covers provider-neutral preflight and lease mismatch.
- `nucleus-task-execution-handoff.md` fixes the downstream mapping without
  importing consumer code.
- Full repository QA passes with 119 tests.

## Stop Condition

Keep the card open with exact failed dimensions if policy or cleanup diverges.
