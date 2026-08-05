# 2026-08-05 Oh My Pi RPC Foundation

Roadmap: `../roadmaps/g03/040-oh-my-pi-rpc-foundation.md`
Cards: `../roadmaps/g03/batch-cards/111-oh-my-pi-artifact-protocol-and-contract.md`
through `../roadmaps/g03/batch-cards/114-oh-my-pi-package-and-route-acceptance.md`

## Changed

- qualified `@oh-my-pi/pi-coding-agent` `17.2.9` as a separate
  `oh-my-pi.package` axis and `oh-my-pi.rpc` route
- added `swallowtail-adapter-oh-my-pi` rather than aliasing Pi's artifact,
  transport, compatibility, or auth claims
- negotiated RPC v2 on catalogue and operation paths
- bounded physical frames at 1 MiB and reassembled logical frames at 64 MiB
- admitted OMP ready, command-update, usage, activity, question, failure, and
  `agent_end` lifecycle shapes with strict correlation
- added local-auth prepared catalogue, structured-run, and interactive-session
  profiles with exact model and optional reasoning confirmation
- selected provider-suppressed read tools, typed questions, one bounded PNG,
  activity, usage, cancellation, and fresh context-losing replacement
- kept writes, permission exchange, host-tool injection, session switching and
  import, provider-state retention, and subagent authority outside the route
- expanded public truth to 33 production routes, 26 solution rows, and 66
  activity operations

## Validation

- `effigy validate:focused swallowtail-adapter-oh-my-pi swallowtail-testkit` —
  124 tests passed; focused package checks passed
- `effigy package:verify-affected swallowtail-adapter-oh-my-pi` — 85 packaged
  files compiled from the extracted crate
- `effigy qa:docs` — passed
- `effigy qa:routes` — 33 routes, 26 solution rows, and 66 activity operations
  passed
- installed identity-only probe — `omp/17.2.9` classified through the exact
  `#!/usr/bin/env bun` launcher
- operator-gated authenticated prepared probe — catalogue exposed
  `openai-codex` / `gpt-5.6-luna`; one `low` structured run returned the exact
  requested answer with usage and clean joined cleanup
  (`SWALLOWTAIL_LIVE_OMP_PROMPT=1 effigy probe:omp-luna-low`)
- `cargo fmt --all -- --check`
- `git diff --check`

The live probe used local OMP auth without inspecting or serializing its
credential store. One initial attempt stopped at catalogue startup. Two
bounded diagnostic prompt attempts reached provider lifecycle but did not
complete. One final prompt completed. No write-capable provider tool ran.

## Live Findings

- pre-turn `setWidget` display and available-command updates are legal startup
  lifecycle
- `model_changed` and `thinking_level_changed` are session lifecycle and do
  not require an active turn
- an empty `setWidget` clears provider UI state and carries no portable display
  content
- both shapes now degrade to lifecycle/drop behavior instead of failing the
  operation

## Next Move

Hold at the g03 evidence gate until a consumer-reproduced portable defect,
material non-deferred provider or interface drift, or explicit operator
promotion supplies the next roadmap input.
