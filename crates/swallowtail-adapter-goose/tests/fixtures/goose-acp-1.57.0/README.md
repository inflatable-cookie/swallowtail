# Goose ACP 1.57.0 Currentness Evidence

`identity.json` freezes the official release-channel result and exact release,
source, and Darwin ARM64 asset identities from the previous ceiling `1.56.0`
through official stable `1.57.0`, and carries forward the `1.53.0` through
`1.56.0` points that the `goose-acp-1.56.0` fixture froze. Each
`manifest-*.tsv` is a complete tag-tree inventory sorted by path with mode and
Git blob SHA. The canonical manifest digest uses UTF-8
`path NUL mode NUL blob-SHA LF` rows. The four earlier manifests reproduce the
`goose-acp-1.56.0` fixture byte for byte.

`dist-inventory.json` records complete added, removed, changed, and identical
path sets for every published hop after `1.53.0`, including the single
`1.56.0` to `1.57.0` hop. `protocol.json` records the selected method
boundary, every changed selected-source file classification, and exact
residual file categories. The selected closure adds
`crates/goose/src/model_config.rs` because the session/new provider and model
resolution moved into that file, and five prompt-lifecycle files on the
session/prompt reply pipeline (prompt_manager, reply_parts,
state_machine/inference_preparation, state_machine/ops_compaction, and
state_machine/ops_exit_on_error), each classified on every hop that touched
it back through 1.53.0. The route test recomputes the complete hop
sets and manifest digests and rejects added, missing, or reclassified paths.

The v1.53.0 through v1.56.0 Darwin ARM64 digests and byte counts reproduce the
goose-acp-1.56.0 fixture. Later asset digests are the GitHub release asset
digest field. No release binary was downloaded or executed. The host Goose
executable was not installed or inspected. No provider prompt, live session,
credential, host configuration, or permission was touched. HTTP MCP remains
emission-only; this evidence does not qualify live honouring.
