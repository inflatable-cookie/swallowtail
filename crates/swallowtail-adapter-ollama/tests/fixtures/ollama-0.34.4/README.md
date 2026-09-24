# Ollama 0.34.4 currentness corpus

This secret-free identity corpus freezes official GitHub `v0.34.3` and
`v0.34.4` against the `0.34.2` claim before Swallowtail moves the
`ollama.runtime` native attached ceiling. Host `ollama` reported client
`0.33.3`; no runtime was reachable and the host was not mutated. Official
app archives were not downloaded. The frozen `0.32.14`, `0.32.15`,
`0.34.0`, and `0.34.2` corpora stay.

Research 342's `v0.34.2` tarball, `api/types.go`, and `server/routes.go`
hashes reproduce. Five selected routes stay registered. `ChatResponse`,
`Message`, `ListResponse`, `ProcessResponse`, `ShowRequest`, `Options`,
`Runner`, and `Metrics` are byte-identical from `v0.34.2` through
`v0.34.4`. `ListHandler`, `ShowHandler`, and `PsHandler` are
byte-identical across the window.

`v0.34.3` adds optional `ShowResponse.thinking`. The catalog decoder
ignores unknown keys and still reads `capabilities` plus gguf details.
`ChatRequest.Think` changes only by comment. Remaining ChatHandler,
GenerateHandler, and compatibility-middleware deltas stay bounded
unmapped. `v0.34.4` `types.go` is byte-identical to `v0.34.3`. GitHub
still marks plain `v0.32.2` and `v0.32.10` as prereleases. Those stay
named exclusions. `v0.34.3-rc1` is ignored.

No fixture contains a credential, host path, account identity, provider
payload, real model observation, or live inference response.
