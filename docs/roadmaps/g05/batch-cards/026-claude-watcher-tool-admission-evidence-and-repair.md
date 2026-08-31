# 026 Claude Watcher Tool Admission Evidence And Repair

Status: ready
Owner: Tom
Created: 2026-08-31
Updated: 2026-08-31
Milestone: `../011-watcher-route-admission-recovery.md`
Depends on: Contracts 059-060; cards 019-020; exact Claude Code `2.1.251`

## Goal

Prove provider-free whether the Claude watcher command's `--tools
Read,Glob,Grep` admission suppresses the reserved
`mcp__swallowtail-watchers__*` family. Repair the exact command boundary if it
does.

## Scope

1. Freeze exact `2.1.251` command/help or local protocol evidence for built-in
   and MCP tool admission without sending a prompt.
2. Build a deterministic operation-private fixture that records MCP
   initialization, tool listing, reserved watcher-tool visibility, and Stop
   hook admission for the exact prepared command.
3. Compare watcher-disabled, current watcher-enabled, and minimally repaired
   command shapes.
4. If the hypothesis is confirmed, change only the Claude watcher command
   admission and its fixtures. Preserve the normal non-watcher command.
5. Prove unreserved MCP tools stay unavailable, operation-private authority and
   exact scope remain intact, and all terminal paths join.
6. Record the result and reassess whether one later live acceptance turn is
   worth authorizing.

## Out Of Scope

- provider prompts, authentication, paid work, or a live watcher attempt
- a watcher capability claim or Contract 059/060 amendment
- generic MCP registry, ambient settings, public HTTP, consumer tool, or
  process-supervision redesign
- skill inventory, feature-façade, currentness, papercut, or release work

## Acceptance Criteria

- exact current and repaired command admissions are observable provider-free
- the reserved watcher family is either proved admitted or the hypothesis is
  rejected with a named alternative blocker
- any repair is opt-in, operation-private, exact-scope, and regression-tested
- no new provider turn is authorized or consumed

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check`
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy qa:routes`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Auto-Continuation

No. Stop for exact-head review and live-readiness reassessment.
