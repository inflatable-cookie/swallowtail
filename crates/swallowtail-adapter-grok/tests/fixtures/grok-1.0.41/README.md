# Grok Build ACP 1.0.41 identity

Frozen against the official npm `@xai-official/grok` stable channel on
2026-09-24. Previous qualified ceiling `1.0.40`; official `latest` and
`alpha` both `1.0.41`. Host `grok` is not on `PATH`; missing install is
not a gap and was not installed.

`identity.json` freezes the npm dist-tags, the `1.0.41` hop after
`1.0.40`, the wrapper and linux-x64 platform integrity, tarball digest,
git head, decompressed executable digest and size, the darwin-arm64
cross-check, and the embedded default-model document digest. The
`1.0.40` baseline hop is copied from the frozen
[`grok-1.0.40`](../grok-1.0.40/identity.json) corpus so the delta is
reproducible. `protocol.json` freezes the mapped ACP surface and its
byte-identical presence digest against `1.0.40`, the unchanged
`grok-4.6` model document, and the ACP module-path delta — one added
unmapped `subagent_handoff` module, no removals. `dist-inventory.json`
freezes the complete shipped file set for both hops and the
`1.0.40` -> `1.0.41` package delta.

No downloaded artifact was executed; no prompt, ACP initialize, session,
login, credential, catalogue command, install, or host update occurred.
The `1.0.4`/`1.0.5` compatibility corpus, the exact `1.0.30` catalogue
corpus, and the registered-tool courier stay separate and unchanged.
