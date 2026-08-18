# 2026-08-18 Pi RPC 0.84.2 Identity

## Result

Card 254 froze host Pi `0.83.0` and official npm
`@earendil-works/pi-coding-agent` `0.84.2` against qualified `0.83.0`.
`rpc-types.ts` and strict-LF `jsonl.ts` stay byte-identical.
`session-cwd.ts` is unchanged. `0.84.0` drops cumulative `message_update`
snapshots; Swallowtail already maps only `assistantMessageEvent` deltas.
Unpublished `0.83.1` stays a gap. No provider prompt. No live RPC session.
The host install was not changed.

## Next

Card 255 raises `pi.package` through `0.84.2`.
