# 007 Claude Code Watcher Seam Evidence

Status: done
Owner: Tom
Created: 2026-08-28
Updated: 2026-08-28
Milestone: `../003-operation-scoped-watcher-proof.md`
Depends on: Contract 059; Research 255
Research: 257

## Goal

Determine whether exact qualified Claude Code headless can carry an
operation-scoped watcher skill and private tool channel while blocking early
completion and returning control to the same `-p` turn.

## Scope

1. Freeze exact `2.1.220..=2.1.241` MCP, settings, hooks, skill loading,
   background work, and `--include-hook-events` behavior.
2. Prove whether one private strict MCP server and watcher instruction asset
   can be supplied without ambient configuration mutation.
3. Trace Stop or equivalent hook input, blocking response, model re-entry,
   repeated stop attempts, terminal ordering, and hook failure.
4. Separate watcher control from Claude-native background task identity and
   provider activity.
5. Return a complete route-mechanism table or honest empty set in Research 257.

## Output

Research and frozen Claude-local evidence only. Do not change the production
command, add an MCP server, inject a skill, start a provider prompt, or claim
watcher support.

## Acceptance Criteria

- [x] exact operation-private MCP and instruction transport is proved
- [x] same-turn completion interception is proved before terminal
- [x] omission preserves current empty strict MCP behavior
- [x] hook, model, host, and consumer authority remain separate
- [x] an empty set blocks cards 010-011

## Result

Research 257 admits a complete candidate mechanism table. Cards 010-011 are
not blocked by an empty set. Production argv is unchanged. Live same-turn
re-entry remains a card 010 gate.

## Validation

- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy qa:northstar`
- `git diff --check`

## Stop Conditions

- evidence needs credentials, provider prompt, paid work, install/update, or
  ambient host mutation
- Stop hooks observe only or cannot return control to the same model turn
- MCP, hooks, or skills require shared persistent configuration
- exact source cannot prove cleanup and terminal ordering

## Auto-Continuation

No. Return Research 257 and one reviewable PR.
