# 2026-08-26 Pi RPC 0.84.3 Identity

## Result

Card 187 froze official npm `@earendil-works/pi-coding-agent` `0.84.3`
against the `0.84.2` claim. Host was not installed. `rpc-types.ts`,
`rpc-mode.ts`, `jsonl.ts`, and `session-cwd.ts` are byte-identical to
`0.84.2`. Extracted `dist/cli.js` matches the `0.84.2` digest
(`840d1e8e689ed9e4937bcb00b9a810e02a8567d9afb10a47097f11ca93ea1521`).
Selected mapped RPC commands and argv flags stay. Unused extras
(`toolcall_start` `id`/`toolName`, `--`, `powershell`, bundled
`dist/bundle/cli.js`, streaming `usage`) stay unmapped. Production claims
stayed at `0.84.2` in this card. Decision for card 188: compatible
extension of `pi.rpc.strict-lf-v0.84.0-message-update-delta` through
`0.84.3`.

## Next

Raise the qualified ceiling on card 188.
