# Codex CLI 0.152.1 currentness corpus

Secret-free identity for official npm `@openai/codex` `0.152.1` before
Swallowtail raises the `codex.cli` ceiling.

Downloaded official binaries were hashed and never executed. The public
GitHub source tree at `rust-v0.152.0` and `rust-v0.152.1` differs in exactly
12 files: the workspace version bump, Guardian auto-review/node-REPL policy
sources, and test files. No crate generating a selected surface changed, and
every upstream-published schema file is byte-identical between the tags, so
the selected exec and app-server surfaces equal the frozen `0.152.0` corpus.

Only `package.json`, `codex-package.json`, and the two rebuilt binaries
change in the shipped platform trees (plus a darwin-only vendored ripgrep and
zsh refresh); the darwin `rg` and `zsh` changes have byte-identical linux
counterparts and are not selected surfaces. No published stable sits between
`0.152.0` and `0.152.1`. Unpublished `0.149.2`, `0.150.2`, and `0.151.1` stay
gaps, and unpublished `0.152.2` is the first later stable. No decoder update
required. The current host's signed `0.150.1` darwin-arm64 binary keeps its
recorded identity; the host install was not changed.
