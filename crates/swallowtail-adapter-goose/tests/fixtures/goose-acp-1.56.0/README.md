# Goose ACP 1.56.0 Currentness Evidence

`identity.json` freezes the official release-channel result and exact release,
source, and Darwin ARM64 asset identities from the previous ceiling `1.53.0`
through official stable `1.56.0`. Each `manifest-*.tsv` is a complete tag-tree
inventory sorted by path with mode and Git blob SHA. The canonical manifest
digest uses UTF-8 `path NUL mode NUL blob-SHA LF` rows.

`dist-inventory.json` records complete added, removed, changed, and identical
path sets for every published hop after `1.53.0`. `protocol.json` records the
selected method boundary, every changed selected-source file classification,
and exact residual file categories. The route test recomputes the complete
hop sets and manifest digests and rejects added, missing, or reclassified
paths.

The `v1.53.0` Darwin ARM64 digest and byte count reproduce Research 392.
Later asset digests are the GitHub release asset digest field. No release
binary was downloaded or executed. The host Goose executable was not installed
or inspected. No provider prompt, live session, credential, host configuration,
or permission was touched. HTTP MCP remains emission-only; this evidence does
not qualify live honouring.
