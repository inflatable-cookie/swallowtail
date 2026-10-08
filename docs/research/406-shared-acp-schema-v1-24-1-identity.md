# Shared ACP Schema v1.24.1 Identity and Qualification

Status: promoted

Question: can the shared stable ACP schema axis advance from its qualified
`schema-v1.20.0` ceiling to the current official stable without changing
Swallowtail's route contracts?

## Method

Queried the official `agentclientprotocol/agent-client-protocol` GitHub stable
schema releases on 2026-10-08. The latest stable tag was `schema-v1.24.1`,
published 2026-09-30. Downloaded the four attached schema assets for
`schema-v1.20.0`, `v1.21.0`, `v1.22.0`, `v1.23.0`, `v1.24.0`, and `v1.24.1`.
Verified each file's byte count and SHA-256 against GitHub release metadata,
resolved every source tag to its commit, and compared each adjacent artifact
and stable JSON definition offline. No schema or executable was run.

The complete release-asset inventory and per-hop file sets are frozen in
[`artifact-inventory.json`](../../crates/swallowtail-protocol-acp/tests/fixtures/acp-schema-v1.24.1/artifact-inventory.json).
Selected definition digests, exact definition and method key changes, and
their boundaries are frozen in
[`protocol.json`](../../crates/swallowtail-protocol-acp/tests/fixtures/acp-schema-v1.24.1/protocol.json).

## Identity

| Stable tag | Published | Source commit | `schema.json` SHA-256 |
| --- | --- | --- | --- |
| `schema-v1.20.0` | 2026-07-21 | `5e89c71497fe07dd4ae633c181a17224f4a8956d` | `92c1dfcda10dd47e99127500a3763da2b471f9ac61e12b9bf0430c32cf953796` |
| `schema-v1.21.0` | 2026-08-20 | `272bf799f35a258c6a4107a0410ed361e83683d3` | `caf62ff962ada396878372ced11efb2c6764e59d90919a38583c319948931a42` |
| `schema-v1.22.0` | 2026-09-17 | `9b4c23b966fa3e64b21b6b2718d8dd71fcb64882` | `3c17bd6385d90cf672d8a661fddc359d73422cf8b8ce6865213d25cfd4c0eca7` |
| `schema-v1.23.0` | 2026-09-18 | `6d08f412a7a1370d3cc9a124e3be3d6acf92641e` | `3c17bd6385d90cf672d8a661fddc359d73422cf8b8ce6865213d25cfd4c0eca7` |
| `schema-v1.24.0` | 2026-09-30 | `cb50abaa3ed455e2113a9d3d4323e17c5a9785cf` | `3c17bd6385d90cf672d8a661fddc359d73422cf8b8ce6865213d25cfd4c0eca7` |
| `schema-v1.24.1` | 2026-09-30 | `1761180eeddf0828d4ecc367106a632c61be06d9` | `3c17bd6385d90cf672d8a661fddc359d73422cf8b8ce6865213d25cfd4c0eca7` |

The `schema-v1.24.1` GitHub tag is signed and GitHub-verified (key
`B5690EEEBB952194`). Its stable `meta.json` digest is
`061edb6efa8fb2aa2792459a86ec7268de5fe665bba48b2ffe7939df01481f88`.
It is the latest stable schema release tag; `schema-v1.24.2` was not
published at observation and remains the synthetic `UnverifiedNewer` point.
The release artifacts contain no runtime or host identity.

## Published Hop Ledger

| Hop | Changed release assets | Selected stable-schema result |
| --- | --- | --- |
| `v1.20.0` → `v1.21.0` | `meta.json`, `schema.json`, `schema.unstable.json` | Adds the existing Claude Agent `clientCapabilities.elicitation.form` and `elicitation/create` form subset. Auth, URL elicitation, `elicitation/complete`, and form fields outside the existing choice-and-Other mapping remain unselected. |
| `v1.21.0` → `v1.22.0` | `schema.json`, `schema.unstable.json` | Adds optional `ToolCall.name` and `ToolCallUpdate.name`. The shared decoder does not project either field. |
| `v1.22.0` → `v1.23.0` | `schema.unstable.json` | `schema.json` and `meta.json` are byte-identical. |
| `v1.23.0` → `v1.24.0` | `meta.unstable.json`, `schema.unstable.json` | `schema.json` and `meta.json` are byte-identical. |
| `v1.24.0` → `v1.24.1` | `schema.unstable.json` | `schema.json` and `meta.json` are byte-identical. |

Each release has the same four files: `meta.json`, `meta.unstable.json`,
`schema.json`, and `schema.unstable.json`. All SHA-256 digests, byte counts,
tag commits, and exact `added`/`removed`/`changed`/`identical` file sets are in
the artifact inventory. The two unstable assets are explicitly outside the
stable schema axis; their changes do not qualify or alter stable v1 behavior.

## Selected Surface

- **Wire and initialize:** `protocolVersion` remains integer `1`. The v1.21
  client capability addition formalizes the existing Claude Agent
  `elicitation.form` advertisement. The route already maps `elicitation/create`
  in form mode to its bounded choice-and-Other typed user-input exchange; this
  qualification adds no Swallowtail operation or capability.
- **Lifecycle:** selected session creation, close, and delete definitions are
  unchanged across every stable hop. The lifecycle corpus remains frozen at
  `schema-v1.20.0`; historical Gemini and Kimi corpora remain pinned to
  `schema-v1.19.0` and `schema-v1.19.1`.
- **Configuration:** selected session config response, option, update, and
  set-option definitions are unchanged across all six artifacts. Their exact
  definition digests are frozen alongside the selected wire definitions.
- **Activity and tools:** selected `ToolCall` and `ToolCallUpdate` fields retain
  their existing mapping. The v1.22 optional `name` fields remain ignored and
  are not copied into typed activity. Other activity definitions do not
  change.
- **Permission:** `RequestPermissionRequest` and
  `RequestPermissionResponse` are byte-identical at the selected definition
  level across all six releases. Existing one-shot and callback boundaries
  remain unchanged.
- **Usage:** `UsageUpdate` is byte-identical across all six releases. The
  selected `used`, `size`, and optional cost fields remain unchanged.
- **Unmapped additions:** new auth capability types, URL elicitation,
  `elicitation/complete`, unsupported richer form types, and tool names do not
  change selected behavior or route capabilities.

## Qualification

Compatible extension from the existing `schema-v1.20.0` ceiling through
`schema-v1.24.1`, including all five published stable hops. No published stable
point in the interval is omitted. Wire `protocolVersion` remains `1`; no
provider route version, operation, permission authority, lifecycle, feature
cell, or public API changes. Existing route-specific version claims and all
historical schema corpora remain intact. No `selection.rs` adapter claim exists
for this shared schema artifact axis; the current shared point is recorded in
the owning architecture and provider route matrix.

## Sources

- [ACP schema releases](https://github.com/agentclientprotocol/agent-client-protocol/releases)
- [ACP `schema-v1.24.1` release](https://github.com/agentclientprotocol/agent-client-protocol/releases/tag/schema-v1.24.1)
- [ACP release configuration for the stable v1 schema artifact](https://github.com/agentclientprotocol/agent-client-protocol/blob/main/.release-plz.toml)
- [Swallowtail ACP schema identity fixture](../../crates/swallowtail-protocol-acp/tests/fixtures/acp-schema-v1.24.1/identity.json)
