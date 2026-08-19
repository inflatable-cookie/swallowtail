# 2026-08-19 Deep Agents ACP Prepared Facade

## Result

Card 301 added `prepare_deepagents_acp` and a typed session operation on
`swallowtail-adapter-deepagents`. Preflight names `swallowtail.deepagents.acp`
and exact `deepagents-acp.package` `0.1.25`. Access stays host-owned
`LocalUnauthenticated` provider API keys; Swallowtail does not bind a
credential lease or wrap `npx`. Missing working-resource authority, the
Python Deep Agents axis, and unqualified packages fail before ACP work.
Spawn stays empty extra argv. `session/prompt` uses field `prompt`.
Current source stays 40 packages and 46 production routes.

`effigy validate:focused swallowtail-adapter-deepagents` (31 tests) and
`effigy package:verify-affected swallowtail-adapter-deepagents` passed. No
live install, prompt, or API-key use.

## Next

Implement the Deep Agents ACP package and route acceptance (card 302).
