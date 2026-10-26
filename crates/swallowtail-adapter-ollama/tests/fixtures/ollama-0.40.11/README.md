# Ollama currentness corpus through 0.40.11

This corpus freezes official GitHub stables `v0.40.1` through `v0.40.11`.
`v0.40.1` is the previous qualified ceiling. The ten later stables are the
hops. Complete recursive Git trees, archive digests, and every file
classification are in `dist-inventory.json`. The Research 418
`ollama-0.40.1` corpus stays unchanged.

`v0.40.2` is the same GitHub release id Research 418 saw as a prerelease.
That release is now non-prerelease. Its tag commit is not `v0.40.2-rc0`.

From `0.40.2`, `/api/tags` lists only the locally preferred manifest-list
child and skips the llamacpp downgrade-guard tag. The row still carries the
parent name, child digest, format, family, and runner. The adapter still
binds that tag and digest.

From `0.40.3` through `0.40.11`, `scheduleRunnerForModel` comments out
`StartLocalCompatibilityMigration`. `compatmigrate/migrate.go` and
`manifest/paths.go` are byte-identical across the whole span.

`ChatRequest`, `ChatResponse`, `Message`, `ShowRequest`, and
`ListModelResponse` keep the same struct text. `ShowResponse` gains
omitempty `template_source` at `0.40.7` and omitempty `voices` at `0.40.11`.
The adapter does not read those fields. Speech, System One, OpenAI, cloud,
MLX, and CLI edits stay unselected.

No local runtime was installed or started. No provider prompt, inference,
model download, or model-store write occurred.
