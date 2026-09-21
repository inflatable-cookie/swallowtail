# 2026-09-21 Pi RPC 0.86.1 Identity

## Result

Research 331 froze official npm `@earendil-works/pi-coding-agent` `0.86.1`
against the `0.85.1` claim. Host `pi` was not on PATH; missing install is
not a gap. npm gitHead matches GitHub tag `v0.86.1`
(`13cbf77df2396303013a41646bcfa77b4271ae56`). Published hops after
`0.85.1` are `0.86.0` and `0.86.1`. Mapped `rpc-types.js`, `jsonl.js`,
`session-cwd.js`, `json-event.js`, `docs/rpc.md`, and `dist/cli.js` are
byte-identical through both hops. `args.js` at `0.86.0` matches `0.85.1`;
`0.86.1` only adds unmapped `META_API_KEY` help. `rpc-mode.js` at `0.86.0`
passes `{ source: "rpc" }` on already-mapped `steer` / `follow_up` so
unmapped extension `input` handlers can see those commands.
`_runInputHandlers` is a no-op without those handlers; selected argv
includes `--no-extensions`. Abort and the 33-command RPC dispatch stay.
Unpublished `0.83.1`, `0.84.5`, and `0.85.2` stay gaps. Unpublished
`0.86.2` is the first later stable. `pi.sdk-sidecar` stays exact
`0.84.2`. Production claims stay at `0.85.1` in this freeze. Decision:
compatible extension of
`pi.rpc.strict-lf-v0.84.0-message-update-delta` through `0.86.1`.

## Next

Raise the qualified ceiling on the claim commit. Keep the sidecar pin.
Add a new Maintained `0.86.0..=0.86.1` segment so unpublished `0.85.2`
stays an incompatible gap.
