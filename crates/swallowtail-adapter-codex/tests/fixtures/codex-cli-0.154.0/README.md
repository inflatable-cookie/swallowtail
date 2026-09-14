# Codex CLI 0.154.0 currentness corpus

Secret-free identity for official npm `@openai/codex` `0.154.0` before
Swallowtail raises the `codex.cli` ceiling past `0.152.1`.

Downloaded official binaries were hashed and never executed. The public
GitHub source tree was compared blob-for-blob at all seven release tag
commits (`rust-v0.152.1` through `rust-v0.154.0`); the six published stable
hops are exactly `0.153.0`, `0.153.1`, `0.153.2`, `0.153.3`, `0.153.4`, and
`0.154.0`. The exec JSONL wire sources and the selected schema params
(`ModelListParams`, `TurnStartParams`, `ThreadStartParams`,
`ThreadReadParams`, archive/delete, turn/interrupt, initialize) are
byte-identical across the window; `ThreadResumeParams` and the bundles gain
only additive definitions with `required: [threadId]` and every selected
property (including `excludeTurns`) unchanged. The only selected-feeding
source deltas are bounded: the default-off `exec --worktree` opt-in with
explicit selected-combination guards, CLI exit-text plus AWS/exec-server
additions, the rollout-cwd scanner refactor, allocator/plumbing internals,
experimental-realtime removal, and new unmapped plugin-reconcile and
user-verification dispatch.

Wrapper `README.md` is byte-identical; `package.json` files are
version-and-pin only; `bin/codex.js` changes once for vite-plus installer
detection. Platform trees keep 7/8 files with identical `README.md`;
binaries rebuild per hop and `codex-package.json` is version-only.
Unpublished `0.149.2`, `0.150.2`, `0.151.1`, and `0.152.2` stay gaps, and
unpublished `0.154.1` is the first later stable. No decoder update required.
The current host's signed `0.153.3` darwin-arm64 binary equals the official
`0.153.3` package digest; the host install was not changed.
