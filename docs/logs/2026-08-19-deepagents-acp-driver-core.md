# 2026-08-19 Deep Agents ACP Driver Core

## Result

Card 300 added package `swallowtail-adapter-deepagents` and the smallest
`deepagents.acp` driver. Discovery is exact `deepagents-acp.package`
`0.1.25`. Spawn is host-approved `deepagents-acp` with no extra argv.
Working resource is the child cwd. First op is initialize, `session/new`,
and one bounded `session/prompt` using field `prompt`. Credentials stay
host-owned `LocalUnauthenticated` provider API keys; Swallowtail does not
bind `ANTHROPIC_API_KEY` / `OPENAI_API_KEY` as a lease. CLI
`agentInfo.version` `0.0.1` is the constructor default, not the npm
package. `npx`, `--workspace` / `--model`, `session/load`, slash
commands, and field `content` stay out. Permission advertises
`allow-always` and does not select it. Current source is 40 packages and
46 production routes. Immutable `v0.3.2` stays 30 and 36. No production
matrix yet.

`effigy validate:focused swallowtail-adapter-deepagents` passed (28 tests,
Clippy warnings denied). No live install, prompt, or API-key use.

## Next

Implement the Deep Agents ACP prepared facade (card 301).
