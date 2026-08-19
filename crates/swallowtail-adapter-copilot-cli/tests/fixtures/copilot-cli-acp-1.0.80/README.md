# Copilot CLI ACP 1.0.80 identity corpus

Secret-free source identity for `copilot-cli.acp` before any Swallowtail
package or claim exists.

Official npm `@github/copilot@1.0.80` plus ACP server docs freeze the
selected stdio wire: `copilot --acp --stdio`, initialize, `session/new`,
one bounded `session/prompt`, cancel, and joined cleanup. Public preview
stays visible.

No live ACP initialize. No provider prompt. No install. No host `copilot`.
TCP `--port`, `--yolo`, and interactive-only slash commands are not this
corpus.

No fixture contains a credential, host path, account identity, provider
payload, or real session id.
