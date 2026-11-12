Qualify `qwen.headless` through official npm `latest` `0.27.2`, walking the
six published stable hops `0.26.7` through `0.27.2` after Research 475's
`0.26.6` ceiling on the existing `qwen-code.headless.package-window-2` claim
and `qwen-code.headless.v0.21.15-reasoning-control` behavior. The maintained
segment becomes `0.22.0..=0.27.2`; there are no interior gaps in the window,
and unpublished `0.20.2`, `0.21.16`, `0.22.4`, `0.23.5`, and `0.25.4` stay
excluded. Every hop keeps selected argv, the stream-json schema, approval
mode, permission flow, Plan policy, reasoning, and the SSH workspace store
byte-identical, and npm SLSA provenance binds each tarball to its release
checkout. The `0.26.9` build was cut from a later `main` than its tag; that
divergence is recorded file by file and its shipped hooks-watcher and
`.title.json` migration changes remain behind the route's `--safe-mode`
accessor gates. New Mods, project-hooks approval, LSP opt-in, and
post-result memory-drain changes are sized against the safe-mode, opt-in,
and joined-cleanup bounds, with mutation-checked evidence; a fake child that
writes its result and then keeps running proves cancellation still
force-stops, waits and cleans up. Baseline
`0.19.11`, the deprecated segments, exact `0.21.15`, plan through `0.22.3`,
and the reasoning and budget selections stay put, and Node `>=22.0.0`
remains unchanged. See Research 481.

The shared nextest shards no longer guess where a nested Cargo put the
`grok-acp-echo-mcp` probe example: the test helper asks Cargo for the built
artifact path, so the probe resolves in any target directory.
