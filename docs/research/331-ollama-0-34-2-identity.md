# Research 331: Ollama 0.34.2 Identity

Status: promoted
Owner: standing currentness lane
Date: 2026-09-21
Authority: Contract 029; Research 313 decoder-tolerance follow-up; the
Ollama selection, decoder, and frozen corpora.

## Question

Is official GitHub `ollama/ollama` `v0.34.2` a compatible extension of
Maintained `ollama.runtime` `0.14.0..=0.33.2` excluding `0.32.2` and
`0.32.10`, a private milestone, a new driver/facade, or a stop?

## Remaining AllowUnverified rank

Named family only. This run does not rank other families.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Ollama attached | not installed | `0.14.0..=0.33.2` excluding `0.32.2` and `0.32.10` | operator-named family; official GitHub latest is `v0.34.2`; Research 313 named this decoder-tolerance follow-up |

Gemini stays deferred. Do not flatten this family onto Ollama Cloud, the
generate API, tools, thinking, or llama.cpp attached/owned.

## Method

Re-probed GitHub `ollama/ollama` latest stable immediately before identity.
Latest is `v0.34.2`, published 2026-09-15T21:21:07Z, `prerelease: false`.
`v0.34.3-rc1` exists and is ignored. Host `ollama` is absent from PATH;
missing install is not a gap. Official app archives were not downloaded.
No prompt, login, install, or live session.

Retrieved official tag tarballs for previous ceiling `v0.33.2` and every
published non-prerelease hop `v0.33.3`, `v0.34.0`, `v0.34.1`, `v0.34.2`
into `/tmp/ollama-identity`. Tarballs were never executed. Compared
`api/types.go` selected structs and `server/routes.go` selected
registrations and handlers with the same brace-matched extraction that
reproduces every Research 313 digest. `v0.33.2`, `v0.33.3`, and `v0.34.0`
file hashes match the frozen 0.34.0 corpus exactly.

## Identity

| Surface | Version | Evidence |
| --- | --- | --- |
| Host CLI | not installed | `ollama` absent from PATH |
| Official GitHub latest | `0.34.2` | tag commit `dfabde4539e42ba1e1eab50a3a50b88aea7958a0`; tree `f7a55d8ed225cbddcaa585b7c5dad7fbb7c80d00`; tag tarball SHA-256 `3fb06dc496f321749423792066f299205c433b815540c1f23f18368540401640` |

Published non-prerelease stables after previous ceiling `0.33.2`:
`0.33.3`, `0.34.0`, `0.34.1`, `0.34.2`. First unpublished later stable is
`0.34.3`. GitHub still marks plain `v0.32.2` and `v0.32.10` as
prereleases. Keep those named holes.

| Version | Tag commit | Tag tree | Selected-surface note |
| --- | --- | --- | --- |
| `0.33.2` | `f96e7aa0` | `c35ae275` | previous ceiling; Research 313 hashes reproduce |
| `0.33.3` | `b79067b0` | `69ea0f1f` | `Metrics.prompt_eval_cached_count` reaches selected `/api/chat` |
| `0.34.0` | `d8ab4b4f` | `61d8627a` | inherits cached-count; rest unselected |
| `0.34.1` | `38fdb5dd` | `0a4e45de` | ListHandler cache removal; ChatHandler `getModel` rename; Options comment |
| `0.34.2` | `dfabde45` | `f7a55d8e` | `types.go` identical to `0.34.1`; selected handlers identical |

Full commits, trees, tarball digests, and the per-hop ledger are frozen in
the
[0.34.2 identity](../../crates/swallowtail-adapter-ollama/tests/fixtures/ollama-0.34.2/identity.json)
and
[protocol](../../crates/swallowtail-adapter-ollama/tests/fixtures/ollama-0.34.2/protocol.json)
fixtures.

## Selected protocol

Seven selected structs stay byte-identical from `v0.33.2` through
`v0.34.2` under the Research 174/313 extraction. `Options` changes only
by a `typical_p` deprecation comment at `v0.34.1`; the JSON tag is
unchanged and the adapter sends only `options.num_ctx`. `Runner` is
byte-identical. The five selected routes stay registered. `ShowHandler`
and `PsHandler` are byte-identical across the window.

`0.33.3` is the Research 313 stop: optional `prompt_eval_cached_count`
lands on selected `/api/chat` NDJSON and the strict decoder fail-closes
on unmapped keys. Generation-defaults fill gaps only (explicit request
options still win). Intermediate-metrics gating fires only when `format`
is requested; the adapter never sends `format`. Both stay bounded
unmapped.

`0.34.0` remainder (Codex desktop proxy, `/v1/responses` compaction,
recommendation thinking advertisement) stays unselected.

`0.34.1` ListHandler drops the unused model-list cache and calls
`listModels` directly; `ListResponse` / `ListModelResponse` stay
byte-identical. Tags "capabilities reported consistently" fills existing
fields the inventory decoder does not read. ChatHandler replaces
`s.getModel` with `GetModel`; error paths stay. `typical_p` deprecation
is unmapped.

`0.34.2` `api/types.go` is byte-identical to `0.34.1`. Selected handlers
are byte-identical. Leftover `routes.go` is a safetensors helper rename
on the show path the adapter rejects (`format` must be `gguf`).
First-run setup, `ollama://apps`, MLX speculative-decoding memory, and
llama.cpp updates stay unmapped.

Decoder specimen remains `ollama-native-v0.14.0-v0.32.1`.

## Decision

Compatible extension of `ollama.native-text-v1` after decoder-tolerance:
accept and ignore `prompt_eval_cached_count`. Raise
`OLLAMA_LATEST_QUALIFIED_VERSION` to `0.34.2`. Qualify published hops
`0.33.3`, `0.34.0`, `0.34.1`, and `0.34.2`. Keep baseline `0.14.0`, claim
id `ollama.native-runtime-window-2`, AllowUnverified, and exclusions
`0.32.2` and `0.32.10`. Synthetic later-stable UnverifiedNewer is
`0.34.3`.

No new public selected operation. No flattening onto Cloud, generate, or
llama.cpp. Production claims stay at `0.33.2` until the claim edit.

## Sources

- [GitHub `v0.34.2`](https://github.com/ollama/ollama/releases/tag/v0.34.2)
- [GitHub `v0.34.1`](https://github.com/ollama/ollama/releases/tag/v0.34.1)
- [GitHub `v0.34.0`](https://github.com/ollama/ollama/releases/tag/v0.34.0)
- [GitHub `v0.33.3`](https://github.com/ollama/ollama/releases/tag/v0.33.3)
- [GitHub `v0.33.2`](https://github.com/ollama/ollama/releases/tag/v0.33.2)
- [GitHub `v0.32.10` still prerelease](https://github.com/ollama/ollama/releases/tag/v0.32.10)
- [GitHub `v0.32.2` still prerelease](https://github.com/ollama/ollama/releases/tag/v0.32.2)
- Tagged `api/types.go` and `server/routes.go` at each hop
- frozen `crates/swallowtail-adapter-ollama/tests/fixtures/ollama-0.34.0/`
- Research 313
