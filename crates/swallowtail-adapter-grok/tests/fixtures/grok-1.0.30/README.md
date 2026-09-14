# Grok Build ACP 1.0.30 identity

Frozen against the official npm `@xai-official/grok` stable channel on
2026-09-14. Previous qualified ceiling `1.0.5`; official `latest` `1.0.30`;
`alpha` `1.0.31` is a channel candidate, not the stable target. The host
`grok 1.0.30 (04b7ffed98c6) [stable]` executable is byte-identical to the
brotli-decompressed `bin/grok.br` from the official
`@xai-official/grok-darwin-arm64@1.0.30` package.

`identity.json` freezes the npm dist-tags, every published stable from
`1.0.5` through `1.0.30`, each hop's wrapper and platform integrity, tarball
digest, git head, decompressed executable digest and size, and the embedded
default-model document digest. `protocol.json` freezes the mapped ACP
surface: method and channel literals, initialize/session/activity keys,
permission option ids, auth and model literals, tolerated vendor channels,
the model document, and the ACP module-path inventory. `dist-inventory.json`
freezes the complete shipped file set for both packages.

No downloaded artifact was executed; no prompt, ACP initialize, session,
login, credential, catalogue command, install, or host update occurred.
Session ids and account metadata are not retained. The `1.0.4`/`1.0.5`
compatibility corpus and the exact `1.0.25` catalogue corpus stay separate
and unchanged.
