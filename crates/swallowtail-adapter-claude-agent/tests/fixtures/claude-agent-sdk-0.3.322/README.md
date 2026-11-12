# Claude Agent SDK 0.3.322 evidence fixture

Identity and file ledger for the published SDK hops above the accepted
`0.3.317`/`2.1.317` pair (Research 480). This is evidence only. No claim moves.

- `identity.json`: official channel, the accepted base, each hop's tarball and
  integrity, wrapper and native manifest identities, the CLI native bindings,
  the selected-surface probe, the per-hop classification, and the no-execution
  flags.
- `dist-inventory.json`: per-file SHA-256 and byte counts for all 21 files of
  each hop, plus exact adjacent added, removed, changed, and identical sets.

Hops: `0.3.317` (accepted base), `0.3.318`, `0.3.319`, `0.3.321`, `0.3.322`
(official npm latest). `0.3.320` is absent from npm. No version above
`0.3.322` is published at observation time.

The tarballs and extracted packages are not stored here. The file hashes are
the binding. The `0.3.317` per-file hashes reproduce the retained Research 472
ledger exactly. This fixture contains no vendor source, no executable bytes,
and no host paths.
