# 2026-09-21 Cursor Agent 2026.09.18 Identity

## Result

Official Cursor Agent `2026.09.18-9a7762b` was frozen against the
`2026.09.10-fd3934a` claim. The ACP registry reports Cursor `2026.09.18`
and official darwin-arm64 plus linux-x64 downloads corroborate exact
`2026.09.18-9a7762b`; no newer stable exists. Downloaded archives were
hashed and never executed. Both published hops after the ceiling
(`2026.09.15-d2fe57e` and `2026.09.18-9a7762b`) are frozen with a complete
tree inventory: per-hop deltas `+62/-61/~55` and `+61/-60/~77`. Selected
CLI definitions and stream-json keys persist. Selected ACP initialize
subset is unchanged on Swallowtail's `fs`-only client; the
`2026.09.15` negotiated `sessionCapabilities.subagents` extra stays
unmapped. Load stays advertised without proven replay. The classification
is a compatible extension. Calendar dates between the nine exact points
stay incompatible. Older published `2026.08.25-3e8eec8` and
`2026.09.08-6caf4ff` stay gaps. Host `cursor-agent` is missing; missing
install is not a gap.

Production claims stayed at `2026.09.10-fd3934a` in this record; two
mutation-sensitive identity tests enforce that boundary.

## Next

Apply the compatible-extension decision on the Cursor claim surfaces
through official `2026.09.18-9a7762b`.
