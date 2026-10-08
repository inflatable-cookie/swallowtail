# Grok Build ACP 1.0.46 identity

Observed 2026-10-08 from the official npm package `@xai-official/grok`:
`latest` is `1.0.46`; `alpha` is `1.0.50`. The five stable hops from the
prior qualified ceiling `1.0.41` are `1.0.42` through `1.0.46`. Published
`1.0.47`–`1.0.49` have no `latest` tag and remain unverified; `1.0.50` is
excluded from this selection because it carries the `alpha` tag.

`identity.json` freezes package integrity, shasum, tarball SHA-256, published
time, package GitHead where npm supplies one, runtime build banner, compressed
and decompressed runtime digests, and platform identities. npm omits GitHead
for `1.0.42`; the exact package and executable digests and embedded runtime
build banner still bind that artifact. No source-parity claim is made for that
hop. `dist-inventory.json` freezes every file in the wrapper, Linux x64 and
Darwin arm64 package trees for `1.0.41`–`1.0.46`, with exact hop-to-hop and
ceiling-to-hop file classifications.

`protocol.json` compares selected literal presence and embedded model data
against the prior identity corpus. The selected literal maps and
`grok-4.6`/`grok-4.5` model document are byte-identical across the selected
hops. The ACP source-path inventory grows from 85 paths at `1.0.41` to 86 at
`1.0.42` with `batch_memory_dream.rs`; there are no later path additions or
removals. That memory path, like the prior `subagent_handoff.rs`, is an
unmapped provider-internal feature. It adds no route operation or authority.

Artifacts were fetched from the public npm registry and inspected without
executing vendor binaries. Brotli decompression was used only to hash and scan
runtime bytes. The host `grok` command was found on `PATH` and was not run.
No provider prompt, session, credential, catalogue request, installation, or
host mutation occurred.
