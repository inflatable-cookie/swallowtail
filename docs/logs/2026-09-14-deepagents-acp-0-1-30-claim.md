# 2026-09-14 Deep Agents ACP 0.1.30 Claim

g05.074 advanced the exact `QualifiedOnly` Deep Agents ACP point from
`0.1.25` to official npm `0.1.30` with the unchanged
`deepagents.acp.stdio-v1` behavior revision, empty selected argv, and route
boundaries. No argv adaptation was needed: the sole CLI body change across
the six published points (`0.1.27..0.1.28`) rewrites
`ACPFilesystemBackend.read` pagination, which the selected route never
reaches because the CLI passes an explicit `FilesystemBackend` config
backend before any client-capability branch and the selected initialize
advertises host `fs` false.

The exact-pinned runtime dependency `deepagents` moved `1.12.4` through
`1.13.4` and every hop is classified. The selected deltas are additive and
decoder-invisible: a builtin `delete` tool through the unchanged generic
tool/permission paths (kind `other`; the child's already-unbounded local
`FilesystemBackend` authority now also covers recursive deletion, still
outside the Swallowtail bounded-write disposition and still gated by the
same observe-and-cancel permission flow), `write_file` `content` becoming
required, and `read_file` pagination footer text inside opaque tool-result
content. The subagent fork/isolated machinery stays unselected. The
`@agentclientprotocol/sdk` range and the `MemorySaver` import never move,
so in-process session persistence stays nondurable.

The frozen `deepagents-acp-0.1.25` fixture corpus remains the decoder
corpus because every selected wire input is byte-stable through `0.1.30`.
The ACP registry's stale discovery-only `0.1.7` entry and the
constructor-default `agentInfo.version` `0.0.1` stay outside the claim, and
Research 206's empty explicit model-selection set stays independent.

No provider operation, prompt, login, credential use, installation,
`npx`, host mutation, or downloaded-artifact execution occurred. PR review
and merge remain queue-owned.
