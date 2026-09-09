# Pi RPC 0.85.1 currentness corpus

This secret-free identity corpus freezes official npm
`@earendil-works/pi-coding-agent` `0.85.1` before Swallowtail widens the
`pi.package` claim. Host `pi --version` is `0.85.1` and matches official
`dist/bundle/cli.js`; it was not replaced. Downloaded official artifacts
were not executed.

Exact npm tarball, GitHub tag, selected git-blob, and shipped-tree
identities live in `identity.json`, `protocol.json`, and
`dist-inventory.json`. Mapped `rpc-types.js`, `rpc-mode.js`, `jsonl.js`,
`session-cwd.js`, and `json-event.js` stay byte-identical from `0.84.4`
through `0.85.1`. `args.js` matches `0.84.4` at `0.85.1`; `0.85.0` only
adds unmapped `PI_SERVER_DIR` / `PI_SERVER_ID` help. `0.85.0` ships an
experimental server/client tree that `0.85.1` removes. Abort still uses
the same RPC command; `session.abort()` already waited for idle at
`0.84.4` and from `0.85.0` also cancels unmapped compaction.

Unpublished `0.83.1` and `0.84.5` stay gaps. Unpublished `0.85.2` is the
synthetic later-stable point. Oh My Pi and `pi.sdk-sidecar` stay separate
axes. No provider prompt. No live RPC session.

No fixture contains a credential, host path, account identity, provider
payload, or real session id.
