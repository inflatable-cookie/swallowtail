# OpenCode ACP 1.18.35 identity fixture

This fixture freezes the official `opencode.acp` package and tagged-source
identity observed on 2026-10-08. No downloaded artifact was executed.

- `identity.json` records npm and GitHub stable-channel observations, exact
  tag commits, source archive hashes, npm tarball hashes and integrity, and
  the preserved Swallowtail claim window.
- `dist-inventory.json` inventories every file in the `opencode-ai` launcher
  and `opencode-darwin-arm64` runtime packages. It also records each complete
  source archive hop, exact selected ACP source hashes, and bounded
  provider-internal source changes.
- `source-tree.tsv` lists every directory, regular file, and symlink in each
  official GitHub tag archive. Paths are relative to the archive root;
  regular files include exact byte length and SHA-256, and symlinks include
  their target. The canonical TSV SHA-256 is
  `6ccadbecb9d91183cb79e236b046cf89ffe031602e96a1451c3c88e64102faff`.
- `protocol.json` retains the prior frozen ACP surface and identifies the
  exact behavior-file slice reviewed for wire, lifecycle, failure,
  permission, usage, configuration, and tool behavior.

The complete tree delta records added, removed, and byte-changed source paths
for `1.18.32 → 1.18.33`, `1.18.33 → 1.18.34`, and `1.18.34 → 1.18.35`.
The ACP SDK pin remains `0.21.0`. HTTP MCP live honouring remains exact to
`1.18.18` and is not inferred for newer releases.
