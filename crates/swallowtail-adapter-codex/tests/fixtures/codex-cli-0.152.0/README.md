# Codex CLI 0.152.0 currentness corpus

Secret-free identity for official npm `@openai/codex` `0.152.0` before
Swallowtail raises the `codex.cli` ceiling.

Mapped exec JSONL flags and app-server help are byte-identical to
`0.151.0`. Selected app-server methods remain. `ModelListParams` and the
selected required fields on `thread/list`, `thread/read`, `thread/start`,
`thread/resume`, `thread/archive`, `thread/delete`, `turn/start`,
`turn/interrupt`, `initialize`, and `model/list` are byte-identical.
`thread/resume` params including already-selected mapped `excludeTurns`
are byte-identical to `0.151.0`
(`8ac68582a81d60940b10b330be8546123f56bfe246b56f8a4f121da00f347cf2`).
Generated v2 and experimental schema bundles differ by unmapped extras
(`thread/shellCommand` optional `timeoutMs`, ModelProvider auth-recovery
notifications). No published stable sits between `0.151.0` and `0.152.0`.
Unpublished `0.149.2`, `0.150.2`, and `0.151.1` stay gaps. No decoder
update required. The current host's signed `0.150.1` darwin-arm64 binary
matches the official platform package; the host install was not changed.

No fixture contains a credential, host path, account identity, provider
payload, or real session id.
