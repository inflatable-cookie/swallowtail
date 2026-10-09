# Claude Agent SDK 0.3.295 evidence fixture

This fixture contains artifact identity and a complete wrapper file-hash ledger for the adjacent 0.3.293 → 0.3.294 → 0.3.295 hops. The `.293` bytes were reacquired only after their SHA-256 matched Research 416; `.294` identity reuses Research 416 and is independently rehashed here. `.295` was fetched from the official npm registry and checked against npm SHA-1 and integrity metadata. No vendor package, native binary or provider interaction was executed.

`dist-inventory.json` records every path, per-version hash, complete tree digest and exact adjacent added/removed/changed/identical sets. `identity.json` binds npm and embedded native manifests. `protocol.json` records the selected behavior classification and preserved boundaries. Full source/control-flow reasoning is in [Research 420](../../../../../docs/research/420-claude-agent-sdk-0-3-295-qualification.md).
