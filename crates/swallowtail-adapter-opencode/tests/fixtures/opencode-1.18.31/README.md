# OpenCode HTTP 1.18.31 Identity

Secret-free identity evidence for npm `opencode-ai@1.18.30..=1.18.31` and
GitHub tags `v1.18.30..=v1.18.31`, observed 2026-09-21. Official tarballs and
tag source archives were downloaded, hashed, and extracted in `/tmp`; no
downloaded executable was run. Host `opencode` is not on `PATH` and was not
installed.

The published GitHub tags diverge: npm `opencode-ai` carries no `gitHead`, and
`v1.18.31` is not a linear descendant of `v1.18.30` (merge base
`5cd8e68fdd72b27818d26d168b9c7a06b359567e`, `ahead_by` 25, `behind_by` 1). The
hop is therefore proved from deterministic complete source-tree inventories,
not from a commit-range summary.

The npm package has four files at both hops. `LICENSE`, `bin/opencode.exe`, and
`postinstall.mjs` are byte-identical; only package metadata changes.
`packages/opencode/src` stays at 408 files. The repository tree grows from 6561
to 6566 files (six added paths, one removed e2e spec, 101 changed files, 60
identical symlink targets at both points).

OpenAPI is byte-identical. Selected health, provider, session, event, abort,
delete, import/history/reconciliation, callback, usage, and detachment route
files stay byte-identical. The only `packages/opencode/src` changes are the
unmapped remote-config auth defect-to-400 mapping, unmapped OpenCode ACP
restore/config-option work, the unmapped TUI exit-status change, and unmapped
GitHub Copilot summarized-thinking request shaping. Provider SDK bumps stay
provider-facing.

Decision: compatible extension of `surface-19` through `1.18.31`. Production
claims remain at `1.18.30` until the claim commit. First unpublished later
patch: `1.18.32`.

`claim.json` freezes the admitted after-state: the same claim id, baseline,
historical gaps, `surface-19`, and `AllowUnverified`, with `1.18.31` qualified
and `1.18.32` left unverified newer.
