# Goose ACP 1.50.0 identity stop

Identity evidence for the g05.071 useful-newer qualification: the exact
`goose.release` baseline `1.46.0` plus all four published stable successors
through official `1.50.0`, retrieved as exact tagged source trees from the
official GitHub channel into `/tmp` and classified without executing a
downloaded artifact, sending a prompt, or touching provider state.

- `identity.json` — official tag/commit/publication identity for all five
  points, Darwin-arm64 asset digests (hashed, never extracted or executed),
  ACP dependency pins, and the typed-stop decision.
- `tree-ledger.json` — the 26-file mapped-module closure with per-version
  SHA-256 and per-hop added/removed/changed/identical sets.
- `protocol.json` — the selected-surface classification: the stop boundary
  at `1.46.0..1.47.0`, everything else held stable, and bounded unmapped
  deltas with non-effect reasons.

The mutation-sensitive assertions live in
`tests/goose_1_50_0_delta_ledger.rs`. The production claim is unchanged:
exact `1.46.0`, `QualifiedOnly`, `goose.acp.stdio-v1`.
