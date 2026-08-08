# 180 Multi-Route Provider History Docs

Status: done
Owner: Tom
Created: 2026-08-08
Milestone: `../058-multi-route-provider-session-history.md`
Depends on: card 179

## Goal

Document which routes advertise history, which stay unsupported and why, and
refresh inventories/baselines after the OpenCode and Alibaba mappings.

## Scope

1. Update the history guide, OpenCode/Alibaba prepared guides, route matrix,
   Contract 054 route mapping, architecture note, and changelog.
2. Regenerate public API baselines for new prepared history surfaces.
3. Record the milestone closeout log.

## Out Of Scope

- inventing ACP history support
- CSV feature-matrix column expansion
- live probes

## Acceptance

- [x] guide lists Codex, OpenCode, and Alibaba retained as advertising routes
- [x] Claude Agent ACP and Kimi ACP called out as unsupported until a
      control-free wire exists
- [x] docs QA for touched guides passes when run for this card
- [x] package API baseline check passes

## Stop Conditions

- stop if docs would imply every load_session route supports history pages

## Auto-Continuation

Close g03.058 and return to the g03 evidence gate.

## Closeout

Log: `docs/logs/2026-08-08-multi-route-provider-history.md`
