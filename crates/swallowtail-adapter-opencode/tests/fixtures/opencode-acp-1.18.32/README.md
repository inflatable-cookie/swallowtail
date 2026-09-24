# OpenCode ACP 1.18.32 Identity

Secret-free identity and surface-freeze evidence for the `opencode.acp` route,
observed 2026-09-22 (g06.015). Official npm tarballs, the platform tarball, and
the GitHub tag source archives were downloaded, hashed, and extracted in `/tmp`;
no downloaded executable was run, and no provider session, prompt, login, or
install occurred.

The window frozen here is `1.18.18..=1.18.32`:

- `identity.json` — npm/GitHub endpoint identity, the installed 1.18.18
  observation, and the `@agentclientprotocol/sdk` 0.21.0 pin.
- `surface.json` — the selected ACP surface: entrypoint and argv, `initialize`
  and its advertised capabilities, session methods, the `mcpServers` handoff
  per transport, prompt and update frames, permission callbacks, stop reasons,
  cancellation, and exit/cleanup.
- `window-ledger.json` — sha256 of every selected published-tag source file at
  each stable from `v1.18.18` through `v1.18.32`.

The installed `opencode 1.18.18` at `~/.opencode/bin/opencode` is byte-identical
to the official `opencode-darwin-arm64@1.18.18` platform binary
(`4f5979c2dadb06fbff1335335afaaea274e58f92e79aa43cf2ed98618d555422`), so the
installed observation and the official 1.18.18 artifact are the same identity.

Fifteen of the nineteen selected source files are byte-identical across the
whole window. The entire selected-surface delta is one hop, `1.18.30 ->
1.18.31`, confined to `acp/service.ts`, `acp/event.ts`, and
`acp/config-option.ts`: session restore and config-option semantics, the new
`config_option_update` notification, and the reasoning chunk `messageId`
retarget. The `mcpServers` chain is byte-identical across every hop.

The decisive finding is that `session/new` accepts a consumer-declared
`mcpServers` list with `http` and `sse` entries carrying `url` and `headers`,
and OpenCode connects them with a StreamableHTTP transport (falling back to
SSE) forwarding the declared headers. The route is therefore not stdio-only:
Longhorn's stdio carrier is not required for a loopback streamable-HTTP entry
with a per-instance bearer header. Live proof that the connection is honoured
in a real turn needs a separately authorized gate.
