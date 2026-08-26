# 2026-08-26 g04.072 Grok Build ACP Subagents Disabled Closeout

Status: stopped after evidence
Owner: Tom
Milestone: g04.072
Cards: 198 complete; 199-200 blocked
Branch: `t3code/add-grok-acp-handoff`
Worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-18b006bc`
Base: `d8481fdc444c9c0e5069a265cea64839492f3a9c` (`origin/main` at dispatch)
Planning base ancestor: `4d8c6db6ac29ce470bf77e0307051ffd572154f9`
PR: https://github.com/inflatable-cookie/swallowtail/pull/71

## Result

Card 198 completed an exact `1.0.4`/`1.0.5` package, parser, configuration,
initialize, spawn-path, and production-seam audit. Research 219 admits no
deliver-now subagents-disabled row. Cards 199 and 200 are blocked and were not
executed. The Grok Build ACP adapter, prepared facade, child argv, fixtures,
guide, matrices, and API baseline are unchanged. No install, host-binary
replacement, login, account inspection, authenticate, session allocation,
prompt, tool/subagent execution, or paid operation was used.

## Evidence Stop

Exact extracted `1.0.4` and `1.0.5` help registers root `--no-subagents`
("Disable subagent spawning"). `grok --no-auto-update --no-subagents agent
stdio --help` parses. The flag is unexpected on `agent` and `agent stdio`.
Repeats fail at parse. `--subagents` is not a clap option.

Unauthenticated ACP `initialize` with and without the flag is structurally
identical after stripping `agentInstanceId`. `subagent_stop` and
`deep-research` remain advertised. Initialize has no tool table and no applied
disabled-profile field.

Exact binaries contain `spawn_subagent` and multiple subagent/ACP spawn paths.
Later public `SubagentsConfig::resolve` documents `--subagents` as
always-enable and is not 1.0.4/1.0.5 source. Ambient `GROK_SUBAGENTS`,
`[subagents] enabled`, `--agents` JSON, and plugins stay unfrozen under
`AmbientHost`.

The restriction is provider-native behavior, not permission and not isolation.
Current `grok --no-auto-update agent stdio` argv, empty `SessionOptions`,
observe-and-stop permission, and `AmbientHost` stay unchanged.

## Changed Surfaces

- `docs/research/219-grok-build-acp-subagents-disabled-evidence.md`:
  promoted exact package, parser, initialize, spawn-path, production audit,
  claim strength, and empty deliver-now table
- cards 198-200, g04.072, programme, triage, indexes, this closeout

No production code, public API, shared contract/runtime, guide capability,
matrix, or changelog edit.

## Validation

Passed:

- `effigy validate:focused swallowtail-adapter-grok` — 30 tests passed
- `effigy qa:northstar`
- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g04`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:next-action:roadmaps`
- `git diff --check`

No production code changed. Doctor was not re-run; the inherited 378 god-file
baseline is unchanged by docs-only edits.

## Continuation

Keep g04 open. Reassess the remaining per-route feature inventory for the next
serial lane unless the operator supplies a different direction. Contract 029
currentness remains standing. Do not compile the next family from this closeout.
