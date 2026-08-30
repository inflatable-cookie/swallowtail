# 010 Claude Code Watcher Bridge

Status: ready
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

The watcher candidate is gated to exact Claude Code `2.1.251`. Base-route
qualification through `2.1.251` does not prove the watcher seam on earlier
qualified points or any later stable. Other versions reject the watcher opt-in
before bridge open, private materialization, or provider process work.

This card does not run Claude, consume provider access, or advertise watcher
support. It returns a bound route candidate for card 011's separately
authorized exact live same-turn acceptance.

## Acceptance Criteria

- [ ] no watcher behavior when unrequested
- [ ] current empty strict MCP route remains unchanged on omission
- [ ] exact `2.1.251` admits the watcher candidate; every other version rejects
      the opt-in before effects
- [ ] opted-in argv and leased material carry only the exact private MCP,
      settings, skill, Stop, and hook-event composition from Research 260
- [ ] deterministic provider fixtures return active-watcher state through the
      exact Stop continuation path before terminal admission
- [ ] model and operator control one registry
- [ ] no ambient settings or project files are mutated
- [ ] no watcher support claim or capability advertisement lands before card
      011's live acceptance

## Readiness Gate Disposition

Research 260's evidence stop is promoted into Contract 060, and card 016 has
landed the provider-neutral bridge. g05.005 cards 017-018 qualified both Claude
Code axes through official `2.1.251`. PR 121 landed that claim at `a70254fb`.
The post-merge orchestrator reassessment passes for exact `2.1.251`.

- Headless and response-only qualified ceilings are now `2.1.251`. Unpublished
  `2.1.244` and `2.1.249` stay incompatible. Watcher behavior remains unmapped.
- The host registry, ordinary process supervision, and Contract 060 HTTP/MCP
  bridge are present.
- Claude binding must use operation-scoped temporary configuration rather than
  argv secrets, ambient settings, project mutation, or a provider-launched
  helper.
- Live same-turn proof is not a prerequisite that can close before binding
  exists. Card 011 owns that exact provider gate after card 010 lands.

Research 260's historical no-ready disposition recorded missing card 016,
current route qualification, and live acceptance. The first two prerequisites
are now closed. Live same-turn proof is intentionally not a card 010
prerequisite because binding must exist before that proof can run. Card 010
remains credential-free and must not absorb card 011's live gate or route claim.

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -p swallowtail-runtime -p swallowtail-host-local`
- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-host-local swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-host-local swallowtail-adapter-claude-agent`
- `effigy qa:northstar`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g05`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:next-action:roadmaps`
- `git diff --check`

## Auto-Continuation

No. Return one PR and stop. Card 011 remains planned until card 010 lands and
the operator separately authorizes live provider access and any paid work.
