# OpenCode HTTP 1.19.4 identity

Secret-free identity evidence for official npm `opencode-ai` and GitHub stable
releases `1.18.35` through `1.19.4`, observed 2026-10-26. The npm launcher and
Linux x64 runtime tarballs, and the standard GitHub tag archives, were
downloaded and SHA-256 hashed before comparison. No downloaded executable was
run. `opencode` was not on `PATH`. The host was not changed.

npm `latest` and GitHub latest stable agree at `1.19.4`. The published stables
after the qualified `1.18.35` ceiling are `1.19.0`, `1.19.1`, `1.19.2`,
`1.19.3`, and `1.19.4`. No stable `1.18.36` or later `1.18` patch was
published. `1.19.5` was not published. Prereleases were ignored. npm metadata
has no `gitHead`.

`dist-inventory.json` freezes npm file hashes, complete repository tree
digests, and every added, removed, and changed path. Selected HTTP route
declaration and handler files stay byte-identical. Selected OpenAPI path
objects stay byte-identical. Classified source edits are the discontinued
share side channel, optional `reasoning_options` catalogue input, CLI update
behavior, a PTY connect check, and the bundled Bun runtime.

Decision: compatible extension of `opencode.http-sse.surface-19`. Keep
`1.18.11..=1.18.35`. Add `1.19.0..=1.19.4`. Leave unpublished `1.18.36`
incompatible and synthetic `1.19.5` unverified. `opencode.acp` stays a
separate claim. This identity record does not by itself mutate production
selection.

No prompt, login, live server, catalogue, session, installation, or provider
contact occurred.
