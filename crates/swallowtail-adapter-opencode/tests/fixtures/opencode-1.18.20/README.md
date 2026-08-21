# OpenCode HTTP 1.18.20 Identity

Secret-free official identity for npm `opencode-ai@1.18.20` on
`opencode.server`. Observed 2026-08-21.

Host `opencode` was not on `PATH`. Official evidence is npm `latest`,
GitHub tag `v1.18.20` (`7248bc1964b13fa67e601733f89ee9dc6dfa0563`), and
`packages/sdk/openapi.json`. ACP registry listed `1.18.20` as discovery
only; the axis is npm `latest`.

Published stables since qualified `1.18.18`: `1.18.19` and `1.18.20`.
No unpublished patch in that span. First unpublished later stable is
`1.18.21`.

OpenAPI at `v1.18.18`, `v1.18.19`, and `v1.18.20` is byte-identical
(`5bbd6493a1a488ef4294889341c896e420f814ecea95822100aaa9f3f95ab2d1`).
Path count 162. Operation count 188. Selected execution, delete, import,
and continuity closures are unchanged. Decoder specimen stays
`opencode-1.14.48`.

Claim at observation: latest qualified `1.18.18`; both newer points
`UnverifiedNewer`. Decision: compatible-extension; keep `surface-19`;
raise through `1.18.20`. Import, reconcile, history, and detach still
do not inherit on unverified-newer.

No prompt, live session, attach, install, or host change.
