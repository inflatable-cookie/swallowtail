# Goose ACP 1.59.0 Identity and Qualification Evidence

`identity.json` freezes the official stable release, tag, source commit, recursive tree, and Darwin ARM64 release asset identities from the previous qualified ceiling `1.57.0` through stable `1.59.0`. The published hops are `1.58.0`, `1.58.1`, and `1.59.0`; unpublished interstitials `1.57.1` and `1.58.2` remain outside the qualified points. The earlier `1.53.0` through `1.57.0` source manifests are copied byte for byte from the accepted `goose-acp-1.57.0` fixture.

`dist-inventory.json` records complete sorted path, mode, and Git blob SHA inventories and exact per-hop added, removed, changed, and identical path sets. `protocol.json` extends the previous selected closure into Goose's `StateMachineServices` and `goose-agent` sources. It classifies every changed selected path and every residual path.

The selected interrupted-run result is unchanged: Goose `1.57.0` synthesizes an interruption error response for each unanswered unique tool request. In `1.59.0`, `ToolCallState::is_open` remains true for every request without a `ToolResponse`, and the same request IDs are deduplicated. The new permission validation still accepts active advertised one-shot replies; it rejects only stale or unmatched request history outside the selected path. Research 478 therefore qualifies the same maintained claim through `1.59.0`, preserving its earlier points, gaps, and boundaries.

Asset digests are GitHub release asset metadata. Source tags and recursive tree identities corroborate each source point; no binary was downloaded or executed. No provider prompt, live session, credential, installation, or host configuration was used. HTTP MCP remains emission-only.
