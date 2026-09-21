# Pi RPC 0.86.1 currentness corpus

This secret-free identity corpus freezes official npm
`@earendil-works/pi-coding-agent` `0.86.1` before Swallowtail widens the
`pi.package` claim. Host `pi` was not on PATH; missing install is not a
gap. Downloaded official artifacts were not executed.

Exact npm tarball, GitHub tag, selected git-blob, and shipped-tree
identities live in `identity.json`, `protocol.json`, and
`dist-inventory.json`. Mapped `rpc-types.js`, `jsonl.js`,
`session-cwd.js`, `json-event.js`, and `docs/rpc.md` stay byte-identical
from `0.85.1` through `0.86.1`. `args.js` matches `0.85.1` at `0.86.0`;
`0.86.1` only adds unmapped `META_API_KEY` help. `rpc-mode.js` at
`0.86.0` passes `{ source: "rpc" }` on already-mapped `steer` /
`follow_up` so unmapped extension `input` handlers can see those
commands. `_runInputHandlers` is a no-op without extension input
handlers; Swallowtail selected argv includes `--no-extensions`. Abort
and the 33-command RPC dispatch stay.

Unpublished `0.83.1`, `0.84.5`, and `0.85.2` stay gaps. Unpublished
`0.86.2` is the synthetic later-stable point. Oh My Pi and
`pi.sdk-sidecar` stay separate axes. No provider prompt. No live RPC
session.

No fixture contains a credential, host path, account identity, provider
payload, or real session id.
