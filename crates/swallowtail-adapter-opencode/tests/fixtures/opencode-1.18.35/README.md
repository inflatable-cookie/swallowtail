# OpenCode HTTP 1.18.35 Identity

Secret-free identity evidence for official npm `opencode-ai` and GitHub stable
releases `1.18.31` through `1.18.35`, observed 2026-10-07. The exact npm
tarballs and standard GitHub tag archives were downloaded and SHA-256 hashed
before extraction in a fresh temporary directory. No downloaded executable was
run. The installed OpenCode `1.18.32` version was observed with an isolated
`OPENCODE_HOME`; its binary hash matches the official `v1.18.32` Darwin arm64
asset. The host was not changed.

The official channels agree at `1.18.35`. Every published stable after the
qualified `1.18.31` ceiling is present: `1.18.32`, `1.18.33`, `1.18.34`, and
`1.18.35`. There is no unpublished or withdrawn hole in the published hop
ledger. The next unpublished patch at observation was `1.18.36`. npm metadata
has no `gitHead`, so npm remains package release authority while the exact GitHub
release tag and source archive provide implementation evidence.

`dist-inventory.json` freezes exact npm package file hashes, complete repository
tree inventory hashes and counts, every repository added/removed/changed path at
each hop, hashes for all changed OpenCode/core implementation files, selected
route mapping file hashes, and the OpenAPI identity. Complete tree inventory
serialization sorts relative paths bytewise and hashes each `path<TAB>sha256:`
file entry or `path<TAB>symlink:` target followed by LF. Standard GitHub archive
hashes are frozen separately from GitHub API tarball hashes because their
compressed bytes differ; extracted inventories were identical.

No selected route declaration, handler, lifecycle file, or OpenAPI operation
changes. Internal mapped deltas are classified in `protocol.json`: Bedrock and
xAI tool-result image conversion, Together usage reporting, dynamic Gemini
variant values and Codex model IDs, Cloudflare provider timeout behavior, and
provider request session headers. They retain the existing payload and decoder
shapes; dynamic catalogue values continue to come only from the exact provider
catalogue. Other changed implementation files are individually classified as
outside the selected attached HTTP/SSE surface. All other repository changes
are bounded by the complete per-hop path ledgers.

Decision: compatible extension of `surface-19` from `1.18.31` through
`1.18.35`, preserving baseline, historical segments, exclusions, and
`AllowUnverified`. Synthetic `1.18.36` remains unverified after qualification.
`claim.json` freezes the proposed post-claim state; this identity record itself
does not mutate production selection.

No prompt, login, live server, catalogue, session, installation, provider contact,
or host change occurred.
