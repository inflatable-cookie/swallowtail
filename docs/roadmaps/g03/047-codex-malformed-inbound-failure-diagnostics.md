# 047 Codex Malformed-Inbound Failure Diagnostics

Status: completed
Owner: Tom
Created: 2026-08-07
Depends on: g03.046
Vision tags: consumer stability, safe diagnostics, compatibility maintenance
Contract refs: 009, 029, 044, 051
Planning state: card 143 completed

## Problem

Nucleus hit Codex app-server `0.147.0` emitting a notification the adapter
rejected as `swallowtail.codex.app_server.malformed_notification`; the next
turn then failed with "connection is closed". The consumer receives only the
sanitized safe diagnostic, and the adapter discarded the offending payload,
so provider-drift bugs were undebuggable from consumer evidence.

## Goals

- [x] re-issue malformed-inbound failures with the notification method and a
      bounded sanitized excerpt of the raw line
- [x] retain a bounded app-server stderr tail for protocol terminal failures
- [x] preserve exact diagnostic codes and poisoned-session behavior
- [x] keep the public API unchanged

## Execution Plan

- [x] Execute card 143.
- [x] share the discovery stderr sanitizer as one crate-internal module
- [x] append method plus bounded sanitized line excerpt to
      `malformed_notification` and `malformed_message` failures
- [x] retain a 2048-byte stderr tail and append a sanitized excerpt to
      protocol terminal diagnostics
- [x] add the scripted malformed-notification fixture and boundary test
- [x] state the bounded-context policy in the failure guides

## Boundaries

- no public API change; internal helpers only
- no raw provider payload, prompt, credential, path, or endpoint in safe
  diagnostics
- no diagnostic-code or classification change
- no version-range, provider, transport, session-behavior, or consumer change
- no tag, release, registry publication, or live provider work

## Acceptance Criteria

- [x] the terminal diagnostic names the notification method and carries a
      bounded sanitized excerpt
- [x] the message stays within the 240-character excerpt bound
- [x] the next request still fails with
      `swallowtail.codex.app_server.connection_closed`
- [x] focused, affected-package, and public-API checks pass

## Next Planning Checkpoint

Return to the g03 evidence gate. The `v0.2.1` source tag remains a separate
operator-authorized step.
