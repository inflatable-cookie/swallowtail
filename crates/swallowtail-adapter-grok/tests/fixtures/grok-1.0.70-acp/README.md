# Grok Build ACP 1.0.70 identity

Observed 2026-10-26 from the official npm package `@xai-official/grok`.
`latest` and `alpha` are both `1.0.70`. The published stable hops after the
prior qualified ceiling `1.0.46` are `1.0.47` through `1.0.70`, with no gaps.
`1.0.71` is unpublished.

`identity.json` freezes package integrity, shasum, tarball SHA-256, published
time, package GitHead where npm supplies one, runtime build banner, compressed
and decompressed runtime digests, and platform identities for each new hop.
npm omits GitHead from `1.0.50` through `1.0.70`. The exact package and
executable digests and the embedded runtime banner still bind those artifacts.
No source-parity claim is made for those hops. `1.0.47` through `1.0.49` still
carry a GitHead whose 12-character prefix matches the banner.

The same file records a reproduction of the frozen Research 385 `1.0.46`
executable and brotli digests, and records that wrapper and Darwin arm64 file
hashes plus the Darwin runtime SHA-256 match the Research 443 inventory. That
match is artifact reproduction. It does not import the catalogue mapping.

`dist-inventory.json` freezes every file in the wrapper, Linux x64, and Darwin
arm64 package trees for `1.0.46` through `1.0.70`, with each hop's added,
removed, changed, and identical paths plus `package_file_counts`. No file
names were added or removed. Wrapper `README.md` changes only at `1.0.56`, in
the platform table.

`protocol.json` records measured selected-literal presence digests, the
curated public operation substrings, and the embedded model document. Those
stay equal to the frozen `1.0.46` ACP corpus on every hop. ACP source-path
additions and removals are listed and classified as provider-internal.

`control-flow.json` is the attributable decode of the selected ACP dispatch
tables: interned stdio, client, and permission concatenations, serde element
counts, and Darwin `ADRP` / Linux `LEA` xrefs onto compact-unwind /
`.eh_frame_hdr` function ranges. Recreate it with
`scripts/analyze-grok-acp-selected-control-flow.py`. Interned tables match
`1.0.46`. Selected handler xref bodies do not, so those hops stay
`UnverifiedNewer`. Join, drain, and release stay Swallowtail `begin_close`
(`close_stdin` plus `request_stop`).

Artifacts were fetched from the public npm registry and inspected without
executing vendor binaries. Brotli decompression was used only to hash and scan
runtime bytes. The host `grok` command was not on `PATH` and was not installed.
No provider prompt, session, credential, catalogue request, or host mutation
occurred.
