# Gemini CLI 0.63.0 identity and source evidence

These fixtures freeze official npm `@google/gemini-cli` and GitHub
`google-gemini/gemini-cli` artifacts observed on 2026-10-07. The npm tarballs
and GitHub source archives were downloaded and inspected without executing
them. No Gemini prompt, login, credential, install, or host update was used.

- `identity.json` records the npm/GitHub channel reconciliation, exact
  identities for `0.61.0`, `0.62.0`, and `0.63.0`, the pre-existing claim,
  and the decision to stop at the existing `0.61.0` ceiling.
- `protocol.json` records the selected headless command and external stream
  contract, mapped source hashes, and the per-hop behavior classification.
- `npm-tree-inventory.json` records a SHA-256 for every file in each complete
  449-file npm package tree and deterministic sorted hop inventories.
- `source-tree-inventory.json` records a SHA-256 for every file and symlink in
  each complete tagged source tree, deterministic hop inventories, and a
  classification for every changed source path. Each source manifest digest
  hashes sorted UTF-8 rows of `path NUL kind:sha256 LF`; symlink hashes cover
  the link target bytes.

The npm tree manifest uses the same row format with kind `file`. The ledger
recomputes both tree manifest digests from every path and file hash.

The selected Plan Mode permission, authority, and tool-output/context changes
in `0.63.0` are unqualified pending an operator ruling. These fixtures do not
extend either Gemini route claim.
