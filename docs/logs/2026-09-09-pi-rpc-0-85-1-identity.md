# 2026-09-09 Pi RPC 0.85.1 Identity

## Result

g05.044 froze official npm `@earendil-works/pi-coding-agent` `0.85.1`
against the `0.84.4` claim. Host is exact `0.85.1`
(`e6d7fcf36a239cf3746e67ddf4222081ac01a601b85a3ee688bdfe9c161d754c`) and
matches official `dist/bundle/cli.js`. npm gitHead matches GitHub tag
`v0.85.1` (`d981de1229ef899957bbe968bc8dcda02a21f477`). Published hops
after `0.84.4` are `0.85.0` and `0.85.1`. Mapped `rpc-types.js`,
`rpc-mode.js`, `jsonl.js`, `session-cwd.js`, and `json-event.js` are
byte-identical through both hops. `args.js` at `0.85.1` matches `0.84.4`;
`0.85.0` only adds unmapped `PI_SERVER_*` help. The `0.85.0` experimental
server/client tree is added then removed. Abort wire stays; compaction
cancel is a bugfix of already-mapped abort covering unmapped `compact`.
Unpublished `0.83.1` and `0.84.5` stay gaps. Unpublished `0.85.2` is the
first later stable. `pi.sdk-sidecar` stays exact `0.84.2`. Production
claims stay at `0.84.4` in this freeze. Decision: compatible extension of
`pi.rpc.strict-lf-v0.84.0-message-update-delta` through `0.85.1`.

## Next

Raise the qualified ceiling on the claim commit. Keep the sidecar pin.
