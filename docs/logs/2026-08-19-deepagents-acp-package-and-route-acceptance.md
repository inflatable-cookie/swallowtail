# 2026-08-19 Deep Agents ACP Package And Route Acceptance

## Result

Card 302 accepted `deepagents.acp` as an unreleased additive production route.

Current source is 40 packages and 47 routes. Immutable `v0.3.2` stays 30
packages and 36 routes. Package `swallowtail-adapter-deepagents` is separately
selectable. Exact claim remains npm `deepagents-acp@0.1.25` /
`deepagents-acp.package`, spawn `deepagents-acp` with no extra argv, field
`prompt`, qualified-only. CLI `agentInfo.version` `0.0.1` is constructor
default, not the package axis. Swallowtail does not wrap `npx`, does not pass
`--workspace` / `--model`, does not bind `ANTHROPIC_API_KEY` /
`OPENAI_API_KEY` as a credential lease, and does not map `session/load` or
`allow-always`. In-process sessions are not durable across restart.

Live install, `npx`, and prompt were not justified: this host has no
`deepagents-acp`, and the card forbids unbounded live qualification.
Deterministic acceptance stands alone.

## Validation

- `effigy validate:focused swallowtail-adapter-deepagents swallowtail-testkit`
- `effigy package:verify-affected swallowtail-adapter-deepagents`
- `effigy check:examples`
- `effigy qa:docs`
- `effigy qa:routes` / `effigy qa:guides` as index agreement

## Next

Close the harness-route watchlist and registry-only disposition (card 303).
