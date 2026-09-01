# Claude Code 2.1.252 identity

Frozen against npm `@anthropic-ai/claude-code@2.1.252`. Host `claude`
reports exact native `darwin-arm64` `2.1.251`. The host was not replaced.

Official extracted darwin-arm64 `--help` is byte-identical to frozen
`2.1.251` (`5ff2e7a0bca8535fb9ec097fa0a21e9d6b735ed94104fa0d1f58ac73a841d52d`).
Selected mapped headless and response-only flags, format choices, and
effort/permission enumerations stay. Wrapper installer files and
`sdk-tools.d.ts` are byte-identical to `2.1.251`; `package.json` changes
only the version pin and platform packages. Changelog `2.1.252` is four
unmapped bugfixes.

`identity.json` and `protocol.json` record the headless axis.
`response-only.json` records the separate response-only axis. Neither
replaces the `claude-code-2.1.220` headless decoder specimen, the
`2.1.227`/`2.1.228` response-only specimens, nor the frozen `2.1.234`/
`2.1.235`/`2.1.238`/`2.1.241`/`2.1.251` identity and feature-stop corpora.
Maximum-turn and other feature-specific exact version sets stay on the
`2.1.220..=2.1.241` probed sets.

Watcher help/digest stays on exact `2.1.251`. Official `2.1.252` help is
the same digest as the frozen watcher isolation and tool-admission
corpora. This family does not copy those fixtures, does not raise
`CLAUDE_CODE_WATCHER_VERSION`, and does not widen watcher help, digest,
or live authorization. The watcher route remains behind its separate
mechanism-change gate. No provider prompt was sent.
