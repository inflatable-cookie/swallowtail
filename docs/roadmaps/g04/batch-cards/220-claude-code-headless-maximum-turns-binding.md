# 220 Claude Code Headless Maximum Turns Binding

Status: conditional
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Milestone: [g04.079 Claude Code Headless Maximum Turns](../079-claude-code-headless-maximum-turns.md)
Depends on: card 219; promoted Research 226 with a non-empty deliver-now set

## Goal

Bind only Research 226's exact maximum-turn rows through a closed Claude
Code-local type, immutable prepared/driver evidence, and canonical headless
argv.

## Scope

1. Add only the adapter-local closed positive-value type Research 226 admits.
   Do not expose raw strings, generic agent budgets, output-token limits, tool
   budgets, cost limits, or configuration maps.
2. Preserve existing construction as exact omission. Add a fallible typed
   selection path only for the admitted numeric domain.
3. Retain the selection immutably in prepared result, plan/evidence, and
   low-level driver binding. Reject version/value/prepared/driver mismatches
   before process work.
4. Dispatch exactly one canonical `--max-turns N` in the position Research 226
   freezes. Explicit argv must retain exact precedence over
   `CLAUDE_CODE_MAX_TURNS`; do not inspect, clear, or rewrite the approved
   environment.
5. Preserve exact no-flag argv and environment for omission. Do not claim that
   omission means unlimited turns when ambient configuration remains opaque.
6. Preserve selected model/reasoning, Plan/read-only policy, fixed
   `Read,Glob,Grep`, strict empty MCP, no-session-persistence, configuration,
   `AmbientHost`, activity, deadline, cancellation, terminal, retention, and
   joined cleanup.
7. Advance an adapter-private behavior revision only when exact evidence
   requires it. Do not change the compatibility window beyond Research 226's
   exact rows.

## Acceptance Criteria

- [ ] only Research 226 deliver-now versions and values prepare the selection
- [ ] public seams are closed and Claude Code-local; no portable or raw API
      appears
- [ ] prepared state, plan/evidence, driver state, version, and argv agree
- [ ] omission retains exact prior argv and approved-environment behavior
- [ ] unsupported, stale, mismatched, or wider rows reject before process work
- [ ] docs claim no stronger enforcement or observation than exact evidence
      supports
- [ ] route authority, configuration, activity, terminal, retention, and
      lifecycle do not widen

## Validation

```sh
cargo fmt -p swallowtail-adapter-claude-agent
effigy validate:focused swallowtail-adapter-claude-agent
effigy package:verify-affected swallowtail-adapter-claude-agent
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 221 only when typed preparation, exact evidence gating,
canonical argv, omission compatibility, rejection, and lifecycle proof pass.

## Stop Conditions

- immutable adapter-local state cannot bind the admitted row without shared
  contract/runtime or breaking public change
- the selected bound can drift, be shadowed, or become inert after preparation
- implementation needs environment mutation, live provider work, sibling-route
  changes, or authority widening

## Out Of Scope

- portable budget promotion, response-only/ACP work, another Claude control,
  provider prompting, currentness, release, merge, rollover, or g04 closure
