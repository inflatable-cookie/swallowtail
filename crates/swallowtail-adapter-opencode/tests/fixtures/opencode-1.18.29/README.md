# OpenCode HTTP 1.18.29 Identity

Secret-free identity evidence for npm `opencode-ai@1.18.28..=1.18.29` and
GitHub tags `v1.18.28..=v1.18.29`, observed 2026-09-07. Official tarballs
were downloaded, hashed, and extracted in `/tmp`; no downloaded executable
was run. Host `opencode` was not on `PATH`. Missing install is not a gap.

The npm package has four files at both hops. `LICENSE`, `bin/opencode.exe`,
and `postinstall.mjs` are byte-identical; only package metadata changes.
GitHub tag source is discovery/correlation evidence because npm does not name a
source commit. `packages/opencode/src` stays 407 files.

OpenAPI is byte-identical. Selected health, provider, session, event, abort,
delete, import/history/reconciliation, callback, usage, and detachment route
files stay byte-identical. The only implementation-source change is unmapped
Codex OAuth model-id filtering in `plugin/openai/codex.ts`.

Decision: compatible extension of `surface-19` through `1.18.29`. Production
claims remain at `1.18.28` until card 136. First
unpublished later patch: `1.18.30`.

`claim.json` freezes the admitted after-state: the same claim id, baseline,
historical gaps, `surface-19`, and `AllowUnverified`, with `1.18.29` qualified
and `1.18.30` left unverified newer.
