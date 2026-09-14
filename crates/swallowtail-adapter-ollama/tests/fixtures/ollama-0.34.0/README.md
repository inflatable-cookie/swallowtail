# Ollama 0.34.0 currentness corpus

This secret-free identity corpus freezes official GitHub `v0.33.0` through
`v0.34.0` against the `0.32.15` claim before Swallowtail moves the
`ollama.runtime` native attached ceiling. Host `ollama` is a 50-byte
launcher shim reporting client `0.33.3`; no runtime was reachable and no
server was started. Official app archives were not downloaded. The frozen
`0.32.14` and `0.32.15` corpora stay.

All eight selected structs (`ChatRequest`, `ChatResponse`, `Message`,
`ListResponse`, `ProcessResponse`, `ShowRequest`, `ShowResponse`,
`Options`) are byte-identical from `v0.32.15` through `v0.34.0`, and
`server/routes.go` is byte-identical from `v0.32.15` through `v0.33.2`.
Hops `0.33.0`, `0.33.1`, and `0.33.2` are a compatible extension: the only
`api/types.go` delta is the unselected experimental model-recommendation
mappings surface, whose `/api/experimental/model-recommendations` route
predates this window.

`0.33.3` is the stop: `Metrics` gains optional `prompt_eval_cached_count`,
plumbed into selected `POST /api/chat` NDJSON records, and the strict
`ollama.native-text-v1` chat decoder fail-closes on that unmapped key (the
frozen counterexample record proves rejection). `0.33.3` and `0.34.0` stay
`UnverifiedNewer`; the remaining `0.34.0` deltas (Codex desktop proxy
route, `/v1/responses` compaction, recommendation thinking advertisement)
are bounded unmapped. GitHub still marks plain `v0.32.2` and `v0.32.10` as
prereleases. Those stay named exclusions.

No fixture contains a credential, host path, account identity, provider
payload, real model observation, or live inference response.
