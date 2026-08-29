# 010 Claude Code Watcher Bridge

Status: planned; gated behind exact containment composition proof and implementation
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-29
Milestone: `../003-operation-scoped-watcher-proof.md`
Depends on: positive Research 257; completed card 009; exact containment-capable host composition

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

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-adapter-claude-agent`
- `git diff --check`

## Auto-Continuation

No. Remains planned until every dependency closes.
