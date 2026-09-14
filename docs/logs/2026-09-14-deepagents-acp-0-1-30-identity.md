# 2026-09-14 Deep Agents ACP 0.1.30 Identity

Research 322 froze official npm `deepagents-acp@0.1.25` plus all five
published stable successors through `0.1.30` as exact registry-verified
tarballs, together with the exact-pinned runtime dependency chain
`deepagents@1.12.4..=1.13.4`. The `0.1.25` baseline reproduces Research 157
and Research 206 byte-for-byte, and the `deepagents@1.12.4` baseline
reproduces Research 206.

The selected no-extra-argv stdio wire is byte-stable at every point:
initialize shape, `session/new` (cwd ignored), the `prompt` field, stop
reasons, cancel, permission options with the unchanged catch-returns-allow
fallback, slash-command interception, and the joined-child cleanup never
move. `@agentclientprotocol/sdk` stays `^1.1.0` and `dist/index.d.ts` is
byte-identical across the whole window. The one CLI body change
(`0.1.27..0.1.28`) rewrites `ACPFilesystemBackend.read` pagination and is
unreachable on the selected route: the CLI passes an explicit
`FilesystemBackend` config backend before any client-capability branch, and
the selected initialize advertises host `fs` false.

The runtime dependency `deepagents` moves 1.12.4 through 1.13.4 and every hop
is classified. The selected deltas are additive and decoder-invisible: a
builtin `delete` tool through the unchanged generic tool/permission paths
(kind `other`; the child's already-unbounded local-file authority now also
covers recursive deletion), `write_file` `content` becoming required, and
`read_file` pagination footer text inside opaque tool-result content. The
subagent fork/isolated machinery stays unselected.

The ACP registry still carries the stale discovery-only `deepagents` `0.1.7`
entry, and initialize `agentInfo.version` stays the constructor default
`0.0.1`; neither is ever compared to the npm package axis.

The identity and surface-ledger corpus is frozen in
`crates/swallowtail-adapter-deepagents/tests/fixtures/deepagents-acp-0.1.30/`.
No tarball was executed or installed, no `npx` was run, no prompt or
provider call was made, and no credential, host state, or host mutation was
used.
