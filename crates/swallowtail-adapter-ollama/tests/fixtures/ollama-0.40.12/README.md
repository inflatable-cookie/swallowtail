# Ollama currentness corpus through 0.40.12

This corpus freezes official GitHub stables `v0.40.11` and `v0.40.12`.
`v0.40.11` is the previous qualified ceiling. `v0.40.12` is the one published
hop. Complete recursive Git trees, archive digests, and every file
classification are in `dist-inventory.json`. The Research 448
`ollama-0.40.11` corpus stays unchanged. The re-downloaded `v0.40.11` tag
tarball matches that record.

`v0.40.12-rc0` is a different commit and stays a prerelease. `v0.40.12-rc1`
is the same commit as the stable and stays an unqualified prerelease version
string.

`api/types.go`, `server/model_list.go`, `server/sched.go`,
`compatmigrate/migrate.go`, and `manifest/paths.go` are byte-identical.
`/api/tags` still lists only the locally preferred manifest-list child.
`scheduleRunnerForModel` still comments out `StartLocalCompatibilityMigration`.

`ChatRequest`, `ChatResponse`, `Message`, `ShowRequest`, `ShowResponse`, and
`ListModelResponse` keep the same struct text. Show still sets omitempty
`voices` only for speech models. The adapter does not read `voices`,
`template_source`, or model info, and it does not read ps context length.

OpenAI audio routes and `ollama launch` stay off the native facade. The GGUF
runner selects the changed llama.cpp pin, and renderers can be selected
independently of parsers. Their effects remain qualification gaps; the claim
ceiling stays at 0.40.11. See Research 462.

No local runtime was installed or started. No provider prompt, inference,
model download, or model-store write occurred.
