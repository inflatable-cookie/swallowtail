# ACP v1 Stable Session List Fixtures

Deterministic stable ACP v1 evidence for optional `session/list`.

Sources were rechecked 2026-08-02. The corpus pins protocol source commit
`a5b23d65366cdad16122989b490593db7795245d` and the `schema/v1/schema.json`
SHA-256 `7f1fba1561163729115247df75b67aeed02085115fbc7ef0131fb01d456c08f9`.
ACP wire version remains `1`; the schema crate at that source is `1.6.0`.

The current stable schema adds `additionalDirectories` to `SessionInfo` only
when its independent capability is advertised. Swallowtail records that wire
shape without widening its first catalogue beyond one exact working resource.

Unknown `_meta` and additive fields are retained only as bounded opaque
extensions. Their values are absent from `Debug`, diagnostics, and portable
candidate projection. A list response grants no load, resume, or deletion
authority.

These are normalized protocol fixtures, not captured provider traffic.
