# 2026-09-14 Cursor Agent 2026.09.10 Identity

## Result

Official Cursor Agent `2026.09.10-fd3934a` was frozen against the
`2026.08.11-e8db854` claim. The ACP registry reports Cursor `2026.09.10`
and the official darwin-arm64 download corroborates exact
`2026.09.10-fd3934a`; no newer stable exists. Downloaded archives were
hashed and never executed. All three published hops after the ceiling
(`2026.08.31-4057e58`, `2026.09.02-c22c1a3`, plus `2026.09.10-fd3934a`)
are frozen with a complete tree inventory: per-hop deltas `+61/-91/~76`,
`+97/-58/~90`, and `+59/-59/~78`. The selected catalogue, ACP initialize,
and stream-json wire are text-identical modulo minifier renames; new flags
attach only to unmapped worker/controller commands, and the SEA/native
packaging refactor feeds no selected surface. The classification is a
compatible extension. Calendar dates between the seven exact points stay
incompatible. The host keeps observation-only `2026.08.04-aaa8809` with
digests equal to the frozen record; the host install was not changed.

Production claims stayed at `2026.08.11-e8db854` in this record; two
mutation-sensitive identity tests enforce that boundary.

## Next

Apply the compatible-extension decision on the Cursor claim surfaces through
g05.062.
