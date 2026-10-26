# Cursor Agent 2026.10.22 catalogue identity

Secret-free identity evidence for `cursor-agent.catalogue` from its qualified
`2026.10.01-14929f9` ceiling to official ACP registry stable
`2026.10.22-27dcf5f`. The published hops are `2026.10.14-0f9edde`,
`2026.10.14-941492e`, `2026.10.16-e3cb207`, `2026.10.18-ea085cb`,
`2026.10.19-6bbf2c4`, and `2026.10.22-27dcf5f`. Registry commits, exact
Darwin ARM64 and Linux x64 archive URLs and hashes, selected module
identities, and the complete package file manifests are frozen in
`identity.json` and `dist-inventory.json`.

`2026.10.14-0f9edde` was published and then replaced the same calendar day
by `2026.10.14-941492e`. The date qualifies only the later build. The earlier
build stays rejected by the exact build gate.

The inventory excludes macOS AppleDouble `._` resource-fork entries. It
records path, byte size, and SHA-256 for every remaining file in each of the
seven compared release trees on both platforms. Adjacent-hop added, removed,
changed, and byte-identical path sets are included.

Static inspection keeps the `models` registration and output mapping: model
id from `displayModelId` or `modelId`, and display name from `displayName` or
`displayNameShort`. The adapter extracts default status only from the exact
`(current, default)` suffix. The account path retains endpoint, api key,
auth token, and insecure. At `2026.10.22` the registration stops wrapping
`insecure` in `Boolean` before the handler, which still applies `Boolean`
itself. The stderr failure string uses an unknown-guard helper. The internal
model clone helper builds a protobuf message with the same printed fields.
Embedded Node changes at `2026.10.22`; the shell launcher does not. Package
metadata gains private dependency versions from `2026.10.16`. None of those
changes the selected stdout rows, argv, or nonzero-exit failure.

`cursor-agent` was not on PATH. Downloaded archives were read and hashed,
never executed. No catalogue request, provider prompt, credential use,
installation, or host update occurred. npm package metadata remains a
separate identity axis. ACP and headless stay at `2026.10.01-14929f9`.
