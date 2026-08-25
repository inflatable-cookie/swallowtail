# 2026-08-25 Kimi Code 0.38.0 Headless V2 Identity

## Result

Card 179 froze official npm `@moonshot-ai/kimi-code@0.38.0` against the
post-g04.063 headless claim. npm `latest` remained `0.38.0`. Tarball SHA-256
`d5c047db…`, tag commit `0999454b…`. Default naked `kimi -p` dispatches to
agent-core-v2 `runV2Print` unless `KIMI_CODE_LEGACY_FLAG` is truthy;
Swallowtail does not set that flag. `prompt-render.ts` is byte-identical
`0.37.2..=0.38.0`. v2 prepends `system.version` meta before shared
`PromptJsonWriter` output. Decision for card 180: adapter-private milestone
under `kimi.headless.stream-json.v2` with public facade
`kimi-headless-stream-json-v1` unchanged.

## Next

Apply the production headless claim edit on card 180.
