# Claude Agent ACP 0.109.1 artifact identities

[Research 490](../../../../../docs/research/490-claude-agent-acp-0-109-1-identity-stop.md)
records the qualification stop. The ledger freezes every published ACP hop above
0.106.0 to 0.109.1 and their exact Agent SDK and ACP SDK dependencies.

`artifact-inventory.json` retains the npm publication time, tarball, verified
SHA-512 integrity, and computed SHA-256. ACP npm git heads match the corresponding
GitHub tags. For each package, start from the baseline path-to-SHA-256 map and apply
each adjacent hop's removals, additions and replacements to reconstruct its full
tree. The remaining files are byte-identical. The reconstructed trees were checked
against the full extracted trees.

The Agent SDK rows also record the compressed and uncompressed native identities
advertised by their shipped manifests on Darwin arm64 and Linux x64. These are
manifest values. The platform tarballs and native binaries still need independent
verification and selected-route execution on trusted disposable runners.
