# ACP v1 Lifecycle Fixtures

Deterministic stable ACP v1 evidence for `session/close` and
`session/delete`.

Sources were accessed 2026-07-27. The corpus pins schema release
`schema-v1.20.0`, source commit
`5e89c71497fe07dd4ae633c181a17224f4a8956d`, and stable schema SHA-256
`92c1dfcda10dd47e99127500a3763da2b471f9ac61e12b9bf0430c32cf953796`.
ACP wire version remains `1`.

Close and delete are independent optional capabilities. Omitted or null
capabilities forbid the matching request. Close cancels active work and frees
active resources. Delete guarantees removal from future `session/list`
results. It does not guarantee provider-data or hard deletion.

The same bounded JSON-RPC records are used by stdio NDJSON and explicit remote
ACP transports. Transport selection, recovery, and fallback remain outside
these records.

These are normalized protocol fixtures, not captured provider traffic.
