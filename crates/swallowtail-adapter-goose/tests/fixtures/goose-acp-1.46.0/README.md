# Goose ACP 1.46.0 identity corpus

Secret-free source identity for `goose.acp` before any Swallowtail package
or claim exists.

Official GitHub `v1.46.0` ACP sources freeze the selected stdio wire:
`goose acp`, initialize, `session/new`, one bounded `session/prompt`,
cancel, and joined cleanup.

No live ACP initialize. No provider prompt. No install. No host `goose`.
`goose serve`, `--with-builtin`, desktop, and Goose ACP-providers are not
this corpus.

No fixture contains a credential, host path, account identity, provider
payload, or real session id.
