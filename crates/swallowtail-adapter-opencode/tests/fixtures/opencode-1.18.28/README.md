# OpenCode HTTP 1.18.28 Identity

Secret-free identity evidence for npm `opencode-ai@1.18.20..=1.18.28` and
GitHub tags `v1.18.20..=v1.18.28`, observed 2026-09-04. Official tarballs
were downloaded, hashed, and extracted in `/tmp`; no downloaded executable
was run.

The npm package has four files at every hop. `LICENSE`, `bin/opencode.exe`,
and `postinstall.mjs` are byte-identical; only package metadata changes.
GitHub tag source is discovery/correlation evidence because npm does not name a
source commit. The full `packages/opencode/src` inventory is 406 files through
`1.18.23` and 407 after `config/v2-compat.ts` is added in `1.18.24`.

OpenAPI changes at `1.18.22` affect only unselected `global.upgrade`. Changes
at `1.18.27` document optional provider header/chunk timeouts and their finite
300-second defaults. Selected health, provider, session, event, abort, delete,
import/history/reconciliation, callback, usage, and detachment route files stay
byte-identical. Changed execution internals preserve selected HTTP/SSE shapes
and remain covered by existing failure and lifecycle handling.

Host observation: `opencode 1.18.18`, SHA-256
`4f5979c2dadb06fbff1335335afaaea274e58f92e79aa43cf2ed98618d555422`,
143182562 bytes, ad-hoc linker signature. The host was not replaced.

Decision: compatible extension through `1.18.28`. Production claims remain at
`1.18.20` until serial Card 078. First unpublished later patch: `1.18.29`.
