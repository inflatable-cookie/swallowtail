# 2026-08-19 Deep Agents ACP 0.1.25 Identity

## Result

Card 299 froze official Deep Agents ACP identity at npm
`deepagents-acp@0.1.25` without installing, running the CLI, or sending
`initialize`. The selected wire is host-approved `deepagents-acp` with no
extra argv: initialize, `session/new`, one bounded `session/prompt` with
field `prompt`, cancel, and joined cleanup. Working resource is the child
cwd; `session/new` cwd is ignored. CLI `agentInfo.version` is constructor
default `0.0.1`, not npm `0.1.25`. Registry still pins `0.1.7`. `npx`,
library embed, custom `tsx`, `--model`/`--workspace`, and `session/load`
stay out. Named fixtures live under the future adapter tree. No production
claim. Current source stays 39 packages and 46 routes.

## Validation

- `effigy qa:northstar`
- `effigy qa:docs:index:research`
- `effigy qa:docs:index:logs`
- `effigy qa:docs:index:roadmaps`
- `effigy qa:docs:index:roadmaps:g03`
- `effigy qa:docs:index:roadmaps:batch-cards`
- `effigy qa:docs:next-action:roadmaps`

## Next

Implement the Deep Agents ACP driver core (card 300).
