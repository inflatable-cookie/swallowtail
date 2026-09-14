# 2026-09-14 Kiro ACP 2.21.4 Identity

Research 320 froze official Kiro stable `2.18.1` plus all eleven published
stable successors through `2.21.4` as exact official archives. The baseline
archive reproduces Research 156's recorded digest; the `2.21.4` archive and
universal DMG match the g05.072 planning digests. Every selected wire,
lifecycle, permission, authentication, process-authority, and cleanup input
is byte-stable across the chain; the `sacp-11.0.0` and
`agent-client-protocol-0.10.4` pins never move, and the `chat-cli-v2` ACP
module closure only adds the unmapped `extension_request.rs` at `2.19.2`.

Bounded unmapped deltas: the `_kiro.dev` extension-request set appears at
`2.19.2` and gains `mcp/startup_status` at `2.20.2`; the serve method list
gains `session/load` at `2.20.2`; and the `2.21.4` chat JS rebuild swaps the
bridge method map while the selected `prompt` field, argv, and method set
stay byte-identical. The official ACP docs page still shows the stale
`content` prompt-field example at `2.21.4`. The provider-free per-hop
evidence closes Research 251/254's 403 package gate without changing their
empty deliver-now sets.

The identity corpus is frozen in
`crates/swallowtail-adapter-kiro/tests/fixtures/kiro-acp-2.21.4/`. No
downloaded artifact was executed, no ACP message was sent, and no install,
login, credential use, or host mutation occurred.
