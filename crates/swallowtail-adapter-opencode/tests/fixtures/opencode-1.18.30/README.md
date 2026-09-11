# OpenCode HTTP 1.18.30 Identity

Secret-free identity evidence for npm `opencode-ai@1.18.29..=1.18.30` and
GitHub tags `v1.18.29..=v1.18.30`, observed 2026-09-11. Official tarballs and
tag source archives were downloaded, hashed, and extracted in `/tmp`; no
downloaded executable was run. Host `opencode` reports `1.18.18`; it was not
installed, updated, or invoked beyond `--version`.

The published GitHub tags diverge: npm `opencode-ai` carries no `gitHead`, and
`v1.18.30` is not a linear descendant of `v1.18.29` (merge base
`02a167e048d3bd7299225068d79e4fce5c830d67`, `ahead_by` 30, `behind_by` 1). The
hop is therefore proved from deterministic complete source-tree inventories,
not from a commit-range summary.

The npm package has four files at both hops. `LICENSE`, `bin/opencode.exe`, and
`postinstall.mjs` are byte-identical; only package metadata changes.
`packages/opencode/src` grows from 407 to 408 files, and the repository tree
grows from 6557 to 6561 files (four added paths, no removals, 101 changed
files, 60 identical symlink targets at both points).

OpenAPI is byte-identical. Selected health, provider, session, event, abort,
delete, import/history/reconciliation, callback, usage, and detachment route
files stay byte-identical. The only `packages/opencode/src` changes are
unmapped provider-internal Bedrock model-id resolution, unmapped GitLab
reasoning option shaping, and the unmapped GPT-6 Astra system-prompt selection
plus prompt text. The bundled core package repeats the Bedrock change. Provider
SDK bumps and the new explicit-service-tier patch are provider-facing only.

Decision: compatible extension of `surface-19` through `1.18.30`. Production
claims remain at `1.18.29` until the g05.051 claim commit. First unpublished
later patch: `1.18.31`.

`claim.json` freezes the admitted after-state: the same claim id, baseline,
historical gaps, `surface-19`, and `AllowUnverified`, with `1.18.30` qualified
and `1.18.31` left unverified newer.
