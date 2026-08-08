# 2026-08-08 v0.3.1 Source Release

Status: closed
Owner: Tom

## Decision

Ship annotated source tag `v0.3.1` for the Contract 053 / 054 patch over
immutable `v0.3.0`.

## Evidence

- candidate commit `4e5aa8fcdb7ee89648668fa606e742b79c76e025`
- local release gates: all 11 passed
- canonical CI:
  https://github.com/inflatable-cookie/swallowtail/actions/runs/31282315991
  succeeded on that SHA
- annotated tag `v0.3.1` peels to the same commit; no GitHub Release or
  registry publication

## Next Move

g03 evidence gate.
