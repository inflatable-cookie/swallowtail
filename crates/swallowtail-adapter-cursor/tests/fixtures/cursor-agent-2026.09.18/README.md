# Cursor Agent 2026.09.18 currentness corpus

Secret-free identity for official Cursor Agent `2026.09.18-9a7762b` before
Swallowtail raises the `cursor-agent.release-date` ceiling past
`2026.09.10-fd3934a`.

Downloaded official darwin-arm64 and linux-x64 archives were hashed and never
executed. The published hops after the ceiling are exactly
`2026.09.15-d2fe57e` and `2026.09.18-9a7762b`, each bound to its opaque build
revision from Cursor's official downloads and the ACP registry Cursor entry.
Per-hop tree deltas (darwin-arm64, excluding AppleDouble junk) are
`+62/-61/~55` and `+61/-60/~77`. Chunk filenames embed content hashes, so
renames dominate the added/removed sets. 305 files are byte-identical through
all three compared trees. Launcher and `package.json` stay byte-identical.

The selected catalogue (`models`), ACP (`acp`), and headless (`--print` with
`stream-json`, `--model`, `--trust`, `--mode plan|ask`) definitions persist
across both hops. The selected ACP initialize subset (protocol version,
`cursor_login`, load/list, image prompt, HTTP/SSE MCP, no `agentInfo`) is
unchanged on Swallowtail's `fs`-only client. `2026.09.15` can advertise
`sessionCapabilities.subagents` only when the client negotiates it; that extra
stays unmapped. Stream-json event keys are unchanged. Load stays advertised
without a proven replay. Continuation recovery stays blocked.

`2026.08.25-3e8eec8` and `2026.09.08-6caf4ff` remain independently unqualified
older published points and stay calendar gaps. Host `cursor-agent` is missing
on this machine; missing install is not a gap. npm `cursor-agent@1.0.3` is a
different axis.
