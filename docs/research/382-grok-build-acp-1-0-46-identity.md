# Grok Build ACP 1.0.46 Identity

Task swallowtail#108 qualifies the selected `grok-build.acp` executable window
from its `1.0.41` ceiling through official npm `latest` `1.0.46`.

## Channel and hops

The official `@xai-official/grok` npm packument was re-read on 2026-10-08.
`latest` is `1.0.46` and `alpha` is `1.0.50`. The published stable hops after
`1.0.41` and through `latest` are `1.0.42`, `1.0.43`, `1.0.44`, `1.0.45`,
and `1.0.46`. Versions `1.0.47`–`1.0.49` are published later but have no
`latest` tag; they remain `UnverifiedNewer`. The alpha-tagged `1.0.50` was not
selected for qualification.

The hop ledger in
[the identity fixture](../../crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.46/identity.json)
freezes npm publication time, `gitHead` where present, wrapper and platform
integrity and shasum, tarball SHA-256, compressed payload digest, decompressed
runtime digest and size, and the runtime version-banner literal. Linux x64 and
Darwin arm64 payloads were compared at every selected hop. npm omits `gitHead`
for `1.0.42`; that hop is still bound to exact wrapper/platform tarballs,
compressed and decompressed runtime digests, and the embedded
`1.0.42 (4651fbdf9f13)` banner. This does not claim source parity for 1.0.42.
`gitHead` is present and matches the embedded build revision for 1.0.43–1.0.46.

The deterministic complete package-tree ledger is in
[dist-inventory.json](../../crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.46/dist-inventory.json).
Each compared wrapper contains five files; each Linux x64 and Darwin arm64
package contains four. There are no file additions or removals. On every hop,
only wrapper `package.json` changes; the platform package's `package.json` and
`bin/grok.br` change, while its README and third-party notices remain
byte-identical. The package digests classify the whole shipped tree, not just
the changelog.

## Mapped behavior and boundaries

The exact Linux x64 selected literal-key map and all presence values match
the `1.0.41` identity. Each selected Darwin arm64 hop retains the prior
Darwin presence digest, and this record freezes the complete per-hop literal
key maps for both architectures. Linux x64 presence digest
`4cceb3e6fc7893dd2e26e8487b5e7266c4040d2fd189ac38ea4569365a10b311`.
Darwin arm64 presence digest is
`4a548e4dc7687941e640dc410146a0ed99dc8b95724871317ffe37a14fe3dfb3`; its
`agentVersion` literal remains present while Linux x64 omits it. Both embedded
`default_models.json` copies are the same 2,323-byte document
with SHA-256
`9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a`; it
retains default `grok-4.6`, models `grok-4.6` and `grok-4.5`, and the previously
mapped effort values.

The ACP source-path set grows from 85 paths at `1.0.41` to 86 at `1.0.42`,
adding only
`crates/codegen/xai-grok-shell/src/session/acp_session_impl/batch_memory_dream.rs`;
no ACP source paths are removed, and the set stays unchanged through `1.0.46`.
The new memory path and existing `subagent_handoff.rs` are provider-internal
features outside the selected mapping. The wire/lifecycle literal map, model
document, and existing mapped-core path set do not move. Exact literal and file
key sets are asserted by the new identity target. This static identity does
not transfer behavior to unobserved provider internals or routes.

The claim is a compatible extension. It keeps baseline `0.2.114`, the existing
deprecated `0.2.114..=0.2.117` segments, the maintained floor `1.0.4`, claim
ID `grok-build.acp.executable-window-2`, behavior revision
`grok-build.acp-v1.cached-token-model-4-6-v3`, and `AllowUnverified`; the
maintained window now ends at `1.0.46`. The exact `1.0.30` catalogue claim,
registered-tool evidence exact to `1.0.4`/`1.0.5`, and prior HTTP MCP boundary
remain independent and unchanged. No new operation or authority is qualified.

The host `grok` command is on `PATH` and was not executed. Public npm tarballs
were statically inspected without executing vendor binaries. No credentials,
provider prompts, live sessions, catalogue requests, installation, or host
mutation occurred.
