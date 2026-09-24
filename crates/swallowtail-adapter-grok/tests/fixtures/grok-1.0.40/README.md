# Grok Build ACP 1.0.40 identity

Frozen against the official npm `@xai-official/grok` stable channel on
2026-09-21. Previous qualified ceiling `1.0.30`; official `latest` and
`alpha` both `1.0.40`. Host `grok` is not on `PATH`; missing install is
not a gap and was not installed.

`identity.json` freezes the npm dist-tags, every published stable from
`1.0.30` through `1.0.40`, each hop's wrapper and linux-x64 platform
integrity, tarball digest, git head, decompressed executable digest and
size, and the embedded default-model document digest. Darwin-arm64
`1.0.30` reproduces Research 314's executable, tarball, and brotli
digests; darwin-arm64 selected-literal presence stays the frozen
`4a548e4dc768…` digest at every hop. `protocol.json` freezes the mapped
ACP surface and the ACP module-path inventory. `dist-inventory.json`
freezes the complete shipped file set for both packages.

No downloaded artifact was executed; no prompt, ACP initialize, session,
login, credential, catalogue command, install, or host update occurred.
The `1.0.4`/`1.0.5` compatibility corpus, the exact `1.0.30` catalogue
corpus, and the registered-tool courier stay separate and unchanged.
