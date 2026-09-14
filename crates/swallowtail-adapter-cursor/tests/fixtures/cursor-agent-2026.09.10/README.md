# Cursor Agent 2026.09.10 currentness corpus

Secret-free identity for official Cursor Agent `2026.09.10-fd3934a` before
Swallowtail raises the `cursor-agent.release-date` ceiling past
`2026.08.11-e8db854`.

Downloaded official darwin-arm64 archives were hashed and never executed.
The three published hops after the ceiling are exactly
`2026.08.31-4057e58`, `2026.09.02-c22c1a3`, and `2026.09.10-fd3934a`, each
bound to its opaque build revision from Cursor's official downloads and the
ACP registry Cursor entry. Per-hop tree deltas are `+61/-91/~76`,
`+97/-58/~90`, and `+59/-59/~78`; chunk filenames embed content hashes so
renames dominate the added/removed sets.

The selected catalogue (`models`), ACP (`acp`), and headless (`--print`
with `stream-json`, `--model`, `--trust`, `--mode plan`) definitions are
text-identical across all four hops modulo minifier renames. The ACP
initialize construction (protocol version, `cursor_login`, load/list
advertisement, image prompt, HTTP/SSE MCP, no `agentInfo`) and the
stream-json event keys are unchanged; load stays advertised without a proven
replay and continuation recovery stays blocked. New flags attach only to the
unmapped worker/controller pool commands; the SEA/native packaging refactor
and the launcher HOME guard feed no selected surface. The host keeps
observation-only `2026.08.04-aaa8809` with digests equal to the frozen
record; the host install was not changed.
