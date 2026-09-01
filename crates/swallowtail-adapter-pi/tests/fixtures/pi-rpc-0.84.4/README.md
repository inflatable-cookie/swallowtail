# Pi RPC 0.84.4 currentness corpus

This secret-free identity corpus freezes official npm
`@earendil-works/pi-coding-agent` `0.84.4` before Swallowtail widens the
`pi.package` claim. Host `pi --version` is qualified `0.83.0` and was not
replaced. Downloaded official artifacts were not executed.

Exact npm tarball and selected git-blob identities live in `identity.json`.
Mapped `jsonl.ts`, `session-cwd.ts`, `json-event.ts`, and `args.ts` stay
byte-identical to `0.84.3`. `rpc-types.ts` and `rpc-mode.ts` add unused
`clear_queue` only. Extracted `dist/cli.js` matches the frozen `0.84.2` /
`0.84.3` digest. Executing `dist/bundle/index.js` packaging is unmapped:
twelve provider-transport chunks plus a main-application-chunk rehash
whose only RPC dispatch delta is additive `clear_queue`. Standing-unused
help-unselected `--use-theme`, `defaultTools`, `--`, and `powershell` are
carried from `0.84.3`, not new `0.84.4` deltas. Streaming `usage`,
`toolcall_start` `id`/`toolName`, terminal capability overrides, extension
UI prompt events, auto-compaction tool-result `_compactBeforeNextAssistantResponse`,
and `triggerTurn:false` extension messages stay unmapped.

Unpublished `0.83.1` stays a gap. Unpublished `0.84.5` is the synthetic
later-stable point. Oh My Pi and `pi.sdk-sidecar` stay separate axes. No
provider prompt. No live RPC session.

No fixture contains a credential, host path, account identity, provider
payload, or real session id.
