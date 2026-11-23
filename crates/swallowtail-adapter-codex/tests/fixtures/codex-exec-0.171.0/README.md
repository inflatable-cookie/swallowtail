# Codex Exec 0.171.0 Identity Fixture

Secret-free frozen identity, protocol classification, npm distribution
inventory, and source tree inventory for the `codex.exec` Contract 029
compatible extension from `0.164.0` through official stable `0.171.0`
(`docs/research/487-codex-exec-0-171-0-identity.md`).

- Covers all ten published stable hops after `0.164.0`: `0.165.0`, `0.166.0`,
  `0.167.0`, `0.167.1`, `0.167.2`, `0.168.0`, `0.169.0`, `0.169.1`, `0.170.0`,
  and `0.171.0`.
- Binds every shipped Linux `x86_64` `codex-resources/bwrap` binary to its
  GitHub Release `bwrap-x86_64-unknown-linux-musl.sigstore` Fulcio/Rekor
  provenance and embedded `CODEX_BWRAP_SHA256` inside the Linux `codex` binary.
- No downloaded vendor binary was executed; no prompt, login, live session, or
  host mutation was performed.
- `codex.app-server` and lifecycle claims remain untouched at `0.164.0`.
