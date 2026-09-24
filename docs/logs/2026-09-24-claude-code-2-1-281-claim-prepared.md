# Claude Code 2.1.281 Claim Prepared

The claim edit is held under g06.018 branch 3. Proposed response-only wording:
the adapter suppresses provider tool calls and consumer MCP servers with an
empty tool list and empty strict MCP config, disables slash commands and
session persistence, and returns one bounded text response. From `2.1.280`,
`--safe-mode` still loads built-in plugin hooks. At `2.1.281`, the built-in
`agents-md` prompt-context hook is enabled by default and may inject project
instructions; first-party telemetry may send network requests. The adapter
cannot guarantee absence of those provider-owned effects.

If the operator accepts that narrowed guarantee, headless can be assessed
independently through `2.1.281`; response-only needs a new behavior revision
starting at `2.1.280`, with per-version argument tests and the historical
`2.1.279` gap retained. Keep watcher exact `2.1.251`, feature-specific exact
sets, ACP and SDK families unchanged. Re-probe latest before any claim push.
