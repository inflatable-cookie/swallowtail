# Claude Agent ACP 0.113.0 frozen evidence

This frozen inventory records the 13 published stable hops after `0.106.0`
through `0.113.0` (`0.107.0`, `0.107.1`, `0.107.2`, `0.107.3`, `0.108.0`,
`0.108.1`, `0.109.0`, `0.109.1`, `0.110.0`, `0.110.1`, `0.111.0`, `0.112.0`,
and `0.113.0`) together with the `0.106.0` baseline for adjacent-hop deltas.

- `registry.json` freezes npm publication timestamps, dependency pins,
  tarball URLs, verified SHA-512 integrity values, computed tarball SHA-256
  digests, and GitHub tag matches (`tag_commit_matches_gitHead: true` for every
  hop).
- `dependencies.json` freezes the exact `@agentclientprotocol/sdk` (`2.6.0`,
  `2.7.0`, `2.8.0`, `2.9.1`, `2.11.0`, `2.12.0`) and
  `@anthropic-ai/claude-agent-sdk` (`0.3.322`, `0.3.323`, `0.3.325`, `0.3.326`,
  `0.3.328`, `0.3.330`, `0.3.331`) tarballs, full file trees, and bundled
  native manifests (`2.1.322` through `2.1.331`).
- `dist-inventory.json` freezes every file SHA-256 across `0.106.0` through
  `0.113.0` and all 13 adjacent hop deltas.

Every tarball was downloaded and inspected statically with zero package
installation or execution on Tom's Mac (decision `1ec44a26` v2). The matching
guarded capture graphs live in
`sidecar/claude-agent-acp-guard-graphs/0.107.0.json` through `0.113.0.json`,
carrying the fail-closed `mcp_refusal_fallback` veto (`0.111.0`–`0.113.0`),
the `remote_control` (`RemoteControlAtStartup`) veto (`0.113.0`), and the
`denied_child_process_parents` denial for `acp/dist/plugin-command.js`
(`0.113.0`).
