# 2026-09-21 Grok Build ACP 1.0.40 Identity

## Result

Official Grok Build npm stable `1.0.40` was frozen against the `1.0.30`
ACP ceiling. `@xai-official/grok` reports `latest` and `alpha` both
`1.0.40` (published 2026-09-20T23:48:33.620Z). Under the same channel
rule Research 294/314 used, every published non-prerelease version at or
below `latest` is a stable, so all ten points `1.0.31..=1.0.40` are hops.
`1.0.31`, recorded as alpha at Research 314, now sits at or below
`latest` and counts as a published stable. The first unpublished stable
after `latest` is `1.0.41`. Host `grok` is missing; that is not a gap and
was not installed.

Every hop's launcher and linux-x64 platform tarball was verified against
its published `sha1`/`sha512` integrity, decompressed, hashed, and
probed; darwin-arm64 was cross-checked at every hop. Downloaded
artifacts were never executed. The `1.0.30` wrapper tarball and
darwin-arm64 executable/tarball/brotli reproduce Research 314; the
`1.0.30` linux-x64 integrity matches the frozen Research 314
`other_platform_integrities` row. `xai-org/grok-build` releases and tags
are empty, so published artifacts are the support authority.

Every mapped ACP method, callback, key, permission id, auth literal,
model and effort literal, and vendor channel is present in all 11
compared linux-x64 executables with a byte-identical presence map
(`4cceb3e6fc78…`). `agentVersion` is already absent on linux-x64 at the
ceiling and stays absent; darwin-arm64 keeps the Research 314 presence
digest including `agentVersion`. The embedded default-model document is
unchanged (`9d6924ec760a…`); default `grok-4.6`, ids `grok-4.6`/`grok-4.5`,
efforts `xhigh`/`high`/`medium`/`low`. The 62 mapped-core ACP modules
persist. Unmapped churn is memory control/forget/carryover, the
`memory_status` rename, `mcp_file_input`, and `prompt_origin`. Shipped
trees stay wrapper 5 / platform 4; only `package.json` and `bin/grok.br`
change per hop. Classification is a compatible extension. The exact
`1.0.30` catalogue claim and the `1.0.4`/`1.0.5` registered-tool courier
stay independently bounded and do not move.

Production claims stayed at `1.0.30` in this record. Mutation-sensitive
identity tests enforce that boundary across the frozen identity,
protocol, and dist inventories.

## Next

Apply the compatible-extension decision on the Grok Build ACP claim
surface only. Do not reopen the catalogue exact pin.
