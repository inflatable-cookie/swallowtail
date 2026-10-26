# Grok Build catalogue 1.0.70 selection window

This secret-free corpus freezes the official `@xai-official/grok` package
identity, complete shipped-file inventories, runtime digests, and the selected
catalogue formatter surface for every published stable from `1.0.30` through
`1.0.70`. It supports the `grok-build.catalogue` window claim; it is not a
release, tag, or provider-execution record.

- `identity.json` records the refreshed registry channel, every compared
  wrapper and Darwin ARM64 package identity, published times, source commit
  identifiers where published, npm SRI and SHA-1 values, locally computed
  tarball SHA-256 values, and the exact catalogue claim before and after the
  adaptation.
- `dist-inventory.json` records every shipped path, size, and SHA-256 per
  version plus added, removed, changed, and identical paths per hop.
- `runtime-surface.json` records each decompressed executable's digest and
  size, the whole-binary catalogue literal counts, both embedded
  `default_models.json` documents, and the exact catalogue formatter class with
  its format pieces and argument placement.
- `identity.json` also carries each executable's single embedded
  `<version> (<revision>)` self-reported build id and banner fragment. The
  carved ids reproduce the frozen ACP identity corpus at `1.0.42..=1.0.46` and
  the discovery pin table, which the prepared-facade sweep exercises.

Registry SRI and SHA-1 values were verified against all downloaded tarballs.
No package script or executable was run, no host install was changed, and no
provider, credential, or inference traffic occurred. The binaries were
brotli-decompressed for static inspection only.

The formatter moves through three classes. `1.0.30` through `1.0.45` render
only a listed default (`Default model: <id>`). `1.0.46` through `1.0.61` add
a header piece for a default that is not in the list. `1.0.62` through `1.0.70`
move the same emitted text into a different piece and argument layout. The
default model id stays interpolated in every class, and Research 443 records
the exact source and disassembly evidence for that decode.
