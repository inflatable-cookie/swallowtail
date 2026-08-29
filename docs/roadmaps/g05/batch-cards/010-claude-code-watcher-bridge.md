# 010 Claude Code Watcher Bridge

Status: planned; depends on card 016 and provider gates
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-30
Milestone: `../003-operation-scoped-watcher-proof.md`
Depends on: positive Research 257; completed cards 009, 014-016; current qualified Claude segment; authorized live same-turn gate

## Goal

Bind the admitted Claude Code headless watcher MCP, instruction, hook, and
completion-interception seam to the host registry.

## Scope

Implement only the exact Research 257 Claude mechanism on the completed
Contract 060 bridge. Add opt-in preparation; operation-private MCP, settings,
hook, and skill material through temporary working-resource leases; reserved
watcher operations; same-turn active-watcher rejection; version gates;
unchanged omission; and joined cleanup. Endpoint and bearer material never
enter argv, ambient environment, shared settings, public records, or default
formatting.

## Acceptance Criteria

- [ ] no watcher behavior when unrequested
- [ ] current empty strict MCP route remains unchanged on omission
- [ ] early completion returns active-watcher state to the same model turn
- [ ] model and operator control one registry
- [ ] no ambient settings or project files are mutated

## Readiness Gate Disposition

Research 260's evidence stop is promoted into Contract 060. Card 016 owns the
provider-neutral bridge implementation. Card 010 remains planned.

- The current Claude Code `2.1.251` point accepts the provider-side candidate
  flags, but no current qualified version segment or live same-turn Stop proof
  is admitted.
- The host registry and ordinary process supervision are present. Contract 060
  selects the missing host-owned HTTP/MCP bridge; card 016 must land before
  Claude binds it.
- Claude binding must use operation-scoped temporary configuration rather than
  argv secrets, ambient settings, project mutation, or a provider-launched
  helper.

Do not mark this card ready or start implementation until card 016 lands, a
current provider segment is qualified, and the live same-turn acceptance gate
is authorized and closed.

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-adapter-claude-agent`
- `git diff --check`

## Auto-Continuation

No. This card remains planned until the provider-neutral bridge,
current-version segment, and live same-turn acceptance gate are positively
closed.
