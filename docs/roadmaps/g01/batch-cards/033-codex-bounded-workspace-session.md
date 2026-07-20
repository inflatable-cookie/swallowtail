# 033 Codex Bounded Workspace Session

Status: completed
Owner: Tom
Updated: 2026-07-20
Milestone: `../010-bounded-workspace-session-access.md`

## Objective

Map the bounded workspace profile into Codex app-server without changing the
read-only session mapping.

## Acceptance

- [x] exact sandbox, writable-root, network, and approval fields are tested
- [x] approval and user-input extensions observe-and-stop with correlation
- [x] completion, timeout, cancellation, failure, and cleanup stay distinct
- [x] arbitrary and secondary writable roots are rejected
- [x] existing Agent Chat fixtures remain unchanged

## Evidence

- `app_server_workspace` proves exact thread and turn policy fields.
- The host lease is the only writable-root source and is released after joined
  provider cleanup.
- Provider approval and user-input requests keep callback, turn, provider
  request, namespace, sequence, and deadline correlation without authority.
- Existing read-only and lifecycle suites pass unchanged.

## Stop Condition

Stop on provider schema ambiguity or any wider filesystem/network grant.
