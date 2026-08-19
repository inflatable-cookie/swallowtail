# Kiro ACP 2.18.1 identity corpus

Secret-free source identity for `kiro.acp` before any Swallowtail package
or claim exists.

Official stable installer manifest `2.18.1` plus ACP docs freeze the
selected stdio wire: `kiro-cli acp`, initialize, `session/new`, one
bounded `session/prompt` with field `prompt`, cancel, and joined cleanup.

No live ACP initialize. No provider prompt. No install. No host
`kiro-cli`. `kiro.headless`, `--cloud`, `_kiro.dev/*`, and advertised
`session/load` are not this corpus.

No fixture contains a credential, host path, account identity, provider
payload, or real session id.
