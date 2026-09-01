# 2026-09-01 Pi RPC 0.84.4 Identity

## Result

Card 039 froze official npm `@earendil-works/pi-coding-agent` `0.84.4`
against the `0.84.3` claim. Host remains exact `0.83.0`
(`af302f231437eaf6f37691bce4b34234fcb626bcb5eb3910d4fc3f6519bf78ca`).
npm gitHead matches GitHub tag `v0.84.4`
(`b79e4cc834970cca69daebffab7df1da7d1e52c4`). Mapped `jsonl.ts`,
`session-cwd.ts`, `json-event.ts`, and `args.ts` are byte-identical to
`0.84.3`. `rpc-types.ts` and `rpc-mode.ts` add unused `clear_queue` only.
Extracted `dist/cli.js` matches frozen `0.84.2` /
`0.84.3` (`840d1e8e689ed9e4937bcb00b9a810e02a8567d9afb10a47097f11ca93ea1521`).
Selected mapped RPC commands and argv flags stay. Unused extras
(`clear_queue`, `toolcall_start` `id`/`toolName`, executing
`dist/bundle/index.js` with twelve provider-transport chunks plus a
main-application-chunk rehash whose only RPC dispatch delta is additive
`clear_queue`, streaming `usage`, terminal capability
overrides, extension UI prompt events, auto-compaction tool-result
`_compactBeforeNextAssistantResponse`, and `triggerTurn:false`
extension messages) stay unmapped. Standing-unused help-unselected
`--use-theme`, `defaultTools`, `--`, and `powershell` are carried from
`0.84.3`, not new `0.84.4` deltas. Unpublished
`0.83.1` stays a gap. Unpublished `0.84.5` is the first later stable.
`pi.sdk-sidecar` stays exact `0.84.2`. Production claims stayed at
`0.84.3` in this card. Decision for card 040: compatible extension of
`pi.rpc.strict-lf-v0.84.0-message-update-delta` through `0.84.4`.

## Next

Raise the qualified ceiling on card 040. Keep the sidecar pin.
