# Grok Build catalogue 1.0.92 identity

The corpus starts at the Research 443 boundary, `1.0.70`, and covers the 22
published stable hops through npm `latest` `1.0.92`. See
[Research 491](../../../../../docs/research/491-grok-build-catalogue-1-0-92-identity.md)
for the compatibility decision.

- `identity.json` binds npm publication records and SHA-1/SRI to the downloaded
  wrapper and Darwin ARM64 tarballs, their computed SHA-256 values, and each
  Brotli-decoded runtime's size, SHA-256 and embedded build id.
- `dist-inventory.json` records every regular file and every adjacent delta.
  The tar members contained no special entries. The wrapper's Darwin ARM64
  optional dependency pins the matching platform package.
- `runtime-surface.json` records sealed help output identities and the static
  decode of the `models` text output. Addresses are unslid Mach-O VMAs,
  except fields named `file_offset`. Read the format table as consecutive
  little-endian `(pointer, length)` pairs. The first four pairs build the
  header; the next two build the marked row; the last builds the plain row.
  Disassemble around the recorded call and branch sites with `llvm-objdump -d`.
  Where a comparison calls a range-extension thunk, follow its single `b`
  instruction to the `_memcmp` stub in the indirect symbol table.

The vendor binaries ran only for isolated `--help` probes. The probes used
`run-with-isolated-home.sh --home-var GROK_HOME --` under a cleared
environment, with a macOS sandbox denying networking, file writes, reads
under `/Users`, and process forks. The catalogue body was inspected statically.
