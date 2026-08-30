# 010 Claude Code Watcher Bridge

Status: planned; depends on card 018 currentness
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-30
Milestone: `../003-operation-scoped-watcher-proof.md`
Depends on: positive Research 257; completed cards 009, 014-016; completed g05.005 card 018

## Goal

Bind the admitted Claude Code headless watcher MCP, instruction, hook, and
completion-interception seam to the host registry.

## Scope

Implement only the exact Research 257 Claude mechanism on the completed
Contract 060 bridge. Add opt-in preparation; operation-private MCP, settings,
hook, and skill material through temporary working-resource leases; reserved
watcher operations; deterministic provider-free completion-loop fixtures;
version gates; unchanged omission; and joined cleanup. Endpoint and bearer
material never enter argv, ambient environment, shared settings, public
records, or default formatting.

This card does not run Claude, consume provider access, or advertise watcher
support. It returns a bound route candidate for card 011's separately
authorized exact live same-turn acceptance.

## Acceptance Criteria

- [ ] no watcher behavior when unrequested
- [ ] current empty strict MCP route remains unchanged on omission
- [ ] deterministic provider fixtures return active-watcher state through the
      exact Stop continuation path before terminal admission
- [ ] model and operator control one registry
- [ ] no ambient settings or project files are mutated
- [ ] no watcher support claim or capability advertisement lands before card
      011's live acceptance

## Readiness Gate Disposition

Research 260's evidence stop is promoted into Contract 060, and card 016 has
landed the provider-neutral bridge. Card 010 remains planned behind the base
Claude route's currentness qualification.

- The installed and npm-latest Claude Code point is `2.1.251`, while the
  existing headless and response-only qualified ceilings remain `2.1.241`.
  g05.005 cards 017-018 own that one-family currentness prerequisite without
  mapping watcher behavior.
- The host registry, ordinary process supervision, and Contract 060 HTTP/MCP
  bridge are present.
- Claude binding must use operation-scoped temporary configuration rather than
  argv secrets, ambient settings, project mutation, or a provider-launched
  helper.
- Live same-turn proof is not a prerequisite that can close before binding
  exists. Card 011 owns that exact provider gate after card 010 lands.

Do not mark this card ready or start implementation until card 018 lands and
the orchestrator rechecks exact provider evidence. Card 010 itself remains
credential-free; it must not absorb card 011's live gate or route claim.

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-adapter-claude-agent`
- `git diff --check`

## Auto-Continuation

No. This card remains planned until g05.005 currentness lands and the
orchestrator marks it ready. It returns one PR and cannot continue into live
acceptance.
