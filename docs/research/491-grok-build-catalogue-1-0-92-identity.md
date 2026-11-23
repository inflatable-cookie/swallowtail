# 491 Grok Build Catalogue 1.0.92 Identity and Qualification

On 2026-11-23, npm `@xai-official/grok` `latest` was `1.0.92`. The 22
published points from `1.0.71` to `1.0.92` are contiguous. `1.0.93` was
`alpha`, and `1.0.94` was the first later point absent from the packument.

## Artifact identity

The [frozen corpus](../../crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.92-catalogue/README.md)
records each wrapper and Darwin ARM64 tarball's publication time, npm SHA-1
and SHA-512 SRI, computed SHA-256, complete file inventory, and decompressed
executable identity. The wrapper and platform artifacts at `1.0.70`
reproduce [Research 443](443-grok-build-catalogue-default-header-mapping.md).
Each hop changes two manifests and `bin/grok.br`. The wrapper entrypoint,
bootstrap, postinstall, READMEs and platform notices are byte-identical.
The wrapper manifest pins the matching platform version on every hop.

The local installed Darwin ARM64 executable matches the `1.0.89` npm payload
byte for byte. It is 190386368 bytes, self-reports build `28be5bb1bf65`, and
its verified signature identifies X.AI Corporation (`5Y6N3AJ54S`). The
`1.0.92` payload is 190640272 bytes and self-reports `b06756fd6256`. The
official packages omit `gitHead`, and the vendor's public source still
identifies the earlier `559751fdcec02d413e4c57c8832ab275e4f44980` sync. The
shipped artifacts carry identity and behaviour authority for these hops.

## Selected invocation and output

The exact Darwin runtime at every point from `1.0.70` through `1.0.92`
accepted `--no-auto-update models --help` with exit 0 and empty stderr in
an isolated, network-denied, read-only probe. The usage and option names
were identical. From `1.0.77`, only the explanatory text for the unselected
`--leader-socket` option changes. No probe listed models or used credentials.

The per-hop Mach-O cross-references bind the catalogue branch to the auth
preamble renderer and the `x.ai/models/list` request. The formatter loads
the same four header pieces and three arguments as Research 443. The first
argument remains the current model id. The search uses length and byte
equality for each listed id, leaving the second argument at 24 bytes when
no id matches and setting it to zero bytes on a match. The empty-list branch
also uses 24 bytes. The third argument is empty. The row paths use the same
id equality comparison to select `  * <id> (default)` or `  - <id>` for
both the first and subsequent rows. The note placement, auth preamble
strings and ordered row grammar all fit the existing `models-text-v2` decoder.

`1.0.74` also has Bedrock setup-status tables that share the `Default model: `
prefix. The separately parsed `bedrock status` command advertises setup status
in its help. Its tables carry `Configured:`, `Enabled:`, region or AWS
profile pieces. The only table paired with the catalogue's missing-default
marker, `Available models:` and plain/marked row arrays is the one referenced
at `0x10016b960`. From `1.0.81`, both copies of the embedded default-model
document change the Grok 4.6 description from `SpaceXAI` to `SpaceXSI`.
Ids, default and all other fields remain identical. The adapter takes
membership, order and default from the CLI text and continues to supply
metadata from its frozen `1.0.30` document by exact id. The in-session
`/models` refresh advertised in the `1.0.89` changelog is a separate TUI
operation. The catalogue path retains its own request and formatter.

## Decision

Extend only `grok-build.catalogue.executable-window-1` on
`grok-build.executable`. Keep `QualifiedOnly`, the deprecated
`1.0.30..=1.0.45` `models-text-v1` segment and the maintained
`models-text-v2` revision. The maintained segment extends from `1.0.46`
through `1.0.92`. Older and later points fail closed. The ACP ceiling
remains `1.0.46` with its independent evidence and posture.

This qualification combines exact Darwin ARM64 evidence with deterministic
prepared-path and parser tests. The live catalogue capsule stays exact to
`1.0.30` and no additional platform run was used for this extension.

## Sources

- [npm wrapper](https://registry.npmjs.org/@xai-official%2fgrok) and
  [npm Darwin ARM64 payload](https://registry.npmjs.org/@xai-official%2fgrok-darwin-arm64)
- [Public source sync](https://github.com/xai-org/grok-build/blob/2bdd1d6a6369de0e8c68132ea4539e9abd9e14a8/SOURCE_REV)
  and [vendor changelog](https://x.ai/changelog/build) for discovery
