# Claude Code 2.1.310 headless and response-only identity

Shared npm artifact evidence for two claims. Headless and response-only stay separate.

`dist-inventory.json` freezes the npm wrapper, Darwin arm64 package, and Linux x64 package from the response-only ceiling `2.1.293` through official stable `2.1.310`. `2.1.293` and `2.1.294` tarball integrity still matches the Research 422 and 423 inventories, so those trees were not re-extracted. Every later tarball was downloaded, checked against current npm SHA-512 integrity, and hashed per file. No downloaded executable was run.

`2.1.297` and `2.1.298` are absent from npm and from GitHub tags. `2.1.311` is the first absent later point.

`marker-ledger.json` records the exact observed marker and stream-json help-line arrays from a byte search of each new platform `claude`. `selected-control-flow.json` binds measured selected JS on every published hop from `2.1.294` through `2.1.310` to those executable hashes: prompt `ok:false` blocking, PreToolUse deny-before-dispatch, rewrite `hookRewrites.push`, the rewrite recheck that turns `ask` into `deny` on `hookUpdatedInput`, Stop/SubagentStop `stop_hook_active` reentry, and the Stop cap default 8, including the `2.1.299` plugin fail-closed path and the `2.1.300` reserved-key `updatedInput` path. `response-only-isolation.json` freezes the exact `--safe-mode` option, builtin-only plugin filter, init `tools` / `mcp_servers` / `permissionMode` fields, permissionMode default mapper, and per-binary `@builtin` plugin ids as measured JS on every hop from `2.1.293` through `2.1.310`. No downloaded executable was run.

`headless-identity.json` and `headless-protocol.json` are the headless claim. `response-only-identity.json` and `response-only-protocol.json` are the response-only claim. At observation, npm `latest` and GitHub latest non-prerelease agree on `2.1.310`. npm `stable` remains the delayed channel at `2.1.308`. npm `next` is also `2.1.310`.

Hop classification in the protocol files cites those hash-bound records. None of the hops changes the selected stream-json fields, the headless v2 hook rules, or the response-only text-only contract. Research 445 records the qualification.
