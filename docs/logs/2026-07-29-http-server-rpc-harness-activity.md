# 2026-07-29 HTTP, Server, And RPC Harness Activity

## Changed

- projected Pi RPC message, thinking, provider-tool, compaction, warning, and
  unknown activity
- projected Kimi local-server step, message, thought, tool, shell, subagent,
  task, compaction, retry, warning, and unknown activity after cursor admission
- projected correlated OpenCode SSE message, reasoning, tool, step, warning,
  and unknown activity
- projected authoritative Managed Agents completions after persisted-event
  deduplication
- published exact observable-activity capability and prepared evidence for
  all four route families
- added one public Kimi prepared-session evidence accessor and refreshed its
  additive pre-1.0 API baseline

## Route Truth

- Pi UI relay remains callback traffic, not model activity.
- Kimi recovery and WebSocket reattachment cannot replay accepted activity.
- OpenCode `1.14.51` remains intentionally thin; correlated safe extension
  events remain namespaced unknowns.
- Managed provider and MCP tools are provider-owned. Custom tools remain
  consumer callback exchange.
- No route exposes raw payloads, tool arguments, tool results, error bodies,
  or hidden reasoning.
- Permitted newer versions inherit the last qualified profile without widening
  it.

## Validation

- `cargo test -p swallowtail-adapter-opencode` — passed
- `cargo test -q -p swallowtail-adapter-pi -p
  swallowtail-adapter-kimi -p swallowtail-adapter-anthropic` — passed
- `effigy check:rust` — passed
- `effigy lint:rust` — passed
- `effigy package:api` — passed after the intentional Kimi baseline refresh
- `cargo fmt --all -- --check` — passed
- `effigy qa:docs` — passed

No executable, credential, account, attached server, model request, paid
inference, or consumer repository was used.

## Current State

Card 129 is complete. Card 130 is ready. Cards 130-137 remain in bounds.

## Next

Map Claude Code, Gemini CLI, Kimi Code, and Qwen Code headless activity from
their qualified machine-readable streams.
