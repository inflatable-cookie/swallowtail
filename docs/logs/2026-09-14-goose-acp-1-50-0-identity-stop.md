# 2026-09-14 Goose ACP 1.50.0 identity stop

Research 319 froze official GitHub `1.46.0` plus all four published stable
successors through `1.50.0` as exact tagged source trees. The ten frozen
`1.46.0` module digests reproduce byte-for-byte; Darwin-arm64 assets for the
four successors were hashed without extraction and never executed. The
26-file mapped-module closure changes 12/16/14/4 files per hop with one docs
removal; seven core files (prompt conversion, mode membership, permission
inspector, builtin registries, ACP common) are byte-identical throughout.

The identity decision is a typed **stop**, and the production claim does not
move. Exact `goose.release` stays at `1.46.0` under
`goose.acp.release-window-1`, one `QualifiedOnly` maintained point with the
unchanged `goose.acp.stdio-v1` behavior revision. The smallest exact
counterexample is the hop `1.46.0..1.47.0`: provider-authentication failure
stops being generic text plus `end_turn` and becomes typed JSON-RPC
`auth_required` on `session/new` (via `agent_creation_error`) and on
`session/prompt` (dedicated `ProviderError::Authentication` arm feeding
`prompt_error_from_message_content`), so identical host provider-auth state
completes at `1.46.0` and fails the turn from `1.47.0` onward.

Everything else on the selected surface is unchanged: `goose acp` argv and
dispatch, absolute-cwd authority (stdio `session_cwd` stays `None`),
initialize echo, stop reasons, cancel lookup, permission kinds, mode state,
and the `sessionId`-only `session/new` parse. Advertised-only additions
(`session/delete`, `recipeParameterScopes`, thinking-effort menus) stay
unmapped, as do ACP-provider paths, client-MCP OAuth fields, extension
management siblings, and serve/roam/TUI/desktop internals. Research 148's
`1.46.0` specimens and Research 250/253's empty builtin/mode dispositions
stand; no advertised capability is converted into an operation.

The evidence is frozen in
`crates/swallowtail-adapter-goose/tests/fixtures/goose-acp-1.50.0/` with a
mutation-sensitive ledger test. No downloaded artifact was executed or
extracted and no provider operation, prompt, login, credential,
installation, or host update occurred. The route returns to the operator via
Chatterbox for a ruling on the failure-binding policy before any
`1.47.0`-or-later claim lands.
