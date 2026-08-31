# Codex CLI 0.151.0 currentness corpus

Secret-free identity for official npm `@openai/codex` `0.151.0` before
Swallowtail raises the `codex.cli` ceiling.

The mapped exec JSONL flags are byte-identical to `0.149.1`. Selected
app-server methods remain. `ModelListParams` is unchanged. Generated v2
and experimental schema bundles differ by unmapped extras
(`thread/turns/list`, `thread/items/list`, resume `excludeTurns`, turn
`serviceTierForTurn` / `toolOutput` / `turnTrigger`, `--code-mode-host`).
Published intermediates `0.150.0` and `0.150.1` keep the same exec help
digest. Unpublished `0.149.2` and `0.150.2` stay gaps. No decoder update
required. The current host's signed `0.150.1` darwin-arm64 binary matches the
official platform package; the host install was not changed.

No fixture contains a credential, host path, account identity, provider
payload, or real session id.
