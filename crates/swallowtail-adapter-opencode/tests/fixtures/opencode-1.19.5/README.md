# OpenCode HTTP 1.19.5 identity

Secret-free identity evidence for official npm `opencode-ai` and GitHub stable
`v1.19.5`, compared with the qualified `1.19.4` ceiling, observed 2026-10-28.
The npm launcher and Linux x64 runtime tarballs, and the standard GitHub tag
archives, were downloaded and SHA-256 hashed before comparison. No downloaded
executable was run. `opencode` was not on `PATH`. The host was not changed.

npm `latest` and GitHub latest stable agree at `1.19.5`. The only published
stable after `1.19.4` is `1.19.5`. No stable `1.18.36` exists. `1.19.6` was
not published. Snapshot, dev, next, beta, and other non-latest dist-tags were
ignored. npm metadata has no `gitHead`.

The ACP registry `opencode` and `opencode-v2` entries both advertise `2.2.0`
and name OpenCode 2 release assets. npm `opencode-ai` has no `2.x` version.
That line is not this family's official stable and is not qualified here.

`dist-inventory.json` freezes npm file hashes, both repository tree digests,
and every added, removed, and changed path. Selected HTTP route declaration
and handler files stay byte-identical. `packages/sdk/openapi.json` stays
byte-identical: 162 paths, `info.version` `1.0.0`. The source edits are a TUI
worker error reply, a PTY connect route match, unused-import and dead-file
removal, and the `gitlab-ai-provider` pin `6.19.0` to `6.23.1`.

Decision: compatible extension of `opencode.http-sse.surface-19`. Keep
`1.18.11..=1.18.35`. Extend `1.19.0..=1.19.4` to `1.19.0..=1.19.5`. Leave
unpublished `1.18.36` incompatible and synthetic `1.19.6` unverified.
`opencode.acp` stays a separate claim. This identity record does not by itself
mutate production selection.

No prompt, login, live server, catalogue, session, installation, or provider
contact occurred.
