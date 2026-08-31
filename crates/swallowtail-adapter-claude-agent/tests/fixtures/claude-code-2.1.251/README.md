# Claude Code 2.1.251 identity

Frozen against npm `@anthropic-ai/claude-code@2.1.251`. Host `claude`
reports the same exact native `darwin-arm64` binary. The host was not
replaced.

Official extracted darwin-arm64 `--help` is not byte-identical to
`2.1.241`. Selected mapped headless and response-only flags, format
choices, and effort/permission enumerations stay. Help additions
`--restricted`, `attach`, `logs`, `stop`/`kill`, `respawn`, and `rm`
stay unmapped. Wrapper installer files are byte-identical to `2.1.241`;
`package.json` changes only the version pin and platform packages.

`identity.json` and `protocol.json` record the headless axis.
`response-only.json` records the separate response-only axis. Neither
replaces the `claude-code-2.1.220` headless decoder specimen, the
`2.1.227`/`2.1.228` response-only specimens, nor the frozen `2.1.234`/
`2.1.235`/`2.1.238`/`2.1.241` identity and feature-stop corpora.
Maximum-turn, autocompact, spend-cap, advisor, permission-mode,
ultracode, fast-mode, and watcher-seam evidence stay on the
`2.1.220..=2.1.241` probed sets. No provider prompt was sent.

`watcher-tool-admission.json` reuses the byte-identical official/host help
corpus to freeze `--tools` as a built-in-set filter, separate from MCP
configuration. It rejects watcher-MCP suppression as the card 026 hypothesis
and records `--bare` authentication as the alternative pre-initialization
blocker. No provider prompt or credential was used.
