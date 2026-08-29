# 010 Claude Code Watcher Bridge

Status: planned; depends on card 015
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-29
Milestone: `../003-operation-scoped-watcher-proof.md`
Depends on: positive Research 257; completed cards 009 and 014; positive Research 260 and completed card 015

## Goal

Bind the admitted Claude Code headless watcher MCP, instruction, hook, and
completion-interception seam to the host registry.

## Scope

Implement only the exact Research 257 mechanism. Add opt-in preparation,
operation-private configuration, reserved watcher operations, same-turn active
watcher rejection, version gates, unchanged omission, and joined cleanup.

## Acceptance Criteria

- [ ] no watcher behavior when unrequested
- [ ] current empty strict MCP route remains unchanged on omission
- [ ] early completion returns active-watcher state to the same model turn
- [ ] model and operator control one registry
- [ ] no ambient settings or project files are mutated

## Readiness Gate Disposition

Research 260 closes as an evidence stop, not a positive transport proof. Card
010 remains planned.

- The current Claude Code `2.1.251` point accepts the provider-side candidate
  flags, but no current qualified version segment or live same-turn Stop proof
  is admitted.
- The host registry and ordinary process supervision are present, but this
  checkout has no host-owned MCP listener, provider-to-existing-process
  handoff, or operation-private IPC bridge into `WatcherHostService`.
- HTTP would require a new operation-scoped listener/bridge contract with
  exact host, turn, operation, authentication, and joined-cleanup semantics.
  Stdio would require a provider-launched helper and a host IPC handoff.

Do not mark this card ready or start implementation until a planning decision
promotes that bridge boundary, a current provider segment is qualified, and
the live same-turn acceptance gate is authorized and closed.

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-adapter-claude-agent`
- `git diff --check`

## Auto-Continuation

No. Research 260 is closed as an evidence stop; this card remains planned until
the provider-to-host MCP transport, current-version segment, and live
same-turn acceptance gate are positively closed.
