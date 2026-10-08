# Ollama currentness corpus through 0.40.1

This corpus freezes official GitHub stables `v0.40.0` and `v0.40.1` after the
qualified `0.35.1` ceiling. Complete recursive Git trees, archive digests, and
the `0.40.0`→`0.40.1` file classification are in `dist-inventory.json`. The
Research 379 `ollama-0.40.0` stop corpus stays unchanged as historical
evidence.

`0.40.0` starts a provider-owned background local compatibility migration from
chat scheduling. Later `/api/tags` can return one row per child runner for the
same display name. Tom ruling `29ebdfbc-ef0b-47f1-9a6b-47b5cd24245b` accepts
that migration as a disclosed inference side effect. The adapter binds the
preflight tag and digest, pins `ggml` or `llamacpp` from the matching row,
observes extra same-tag sibling rows, skips unmapped non-gguf, empty-family,
or unknown runners, and fails closed when the selected tag is present only
under another digest. Empty family fails only for the selected tag and digest.

`0.40.1` keeps the selected types, list, migrate, show, and chat mapping on
milestone `ollama.native-text-v1.manifest-list-runner`. Prior qualified
points stay on Deprecated `ollama.native-text-v1`. `server/routes.go` adds
unselected cloud `/api/balance` and `/api/usage`. Store lock and Windows
copy changes do not alter catalogue wire fields.

No local runtime was reachable or started. No provider prompt, inference,
model download, host mutation, or model-store write occurred.
