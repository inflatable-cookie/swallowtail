# Research 350: Ollama 0.34.4 Identity

Status: promoted
Owner: standing currentness lane
Date: 2026-09-24
Authority: Contract 029; Research 342; the Ollama selection, decoder, and
frozen corpora.

## Question

Is official GitHub `ollama/ollama` `v0.34.4` a compatible extension of
Maintained `ollama.runtime` `0.14.0..=0.34.2` excluding `0.32.2` and
`0.32.10`, a private milestone, a new driver/facade, or a stop?

## Remaining AllowUnverified rank

Named family only. This run does not rank other families.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Ollama attached | client `0.33.3`; no runtime reachable | `0.14.0..=0.34.2` excluding `0.32.2` and `0.32.10` | operator-named family; official GitHub latest is `v0.34.4` |

Gemini stays deferred. Do not flatten this family onto Ollama Cloud, the
generate API, tools, thinking content, or llama.cpp attached/owned.

## Method

Re-probed GitHub `ollama/ollama` latest stable immediately before identity.
Latest is `v0.34.4`, published 2026-09-23T02:24:43Z, `prerelease: false`.
`v0.34.3` is the only published non-prerelease hop after `v0.34.2`.
`v0.34.3-rc1` exists, shares the `v0.34.3` commit, and is ignored.
`v0.34.5` is unpublished. Host `ollama --version` reports client `0.33.3`
with no running instance. Host was not mutated. Official app archives were
not downloaded. No prompt, login, install, or live session.

Retrieved official tag tarballs for previous ceiling `v0.34.2` and hops
`v0.34.3` and `v0.34.4` into `/tmp/ollama-identity`. Tarballs were never
executed. Research 342's `v0.34.2` tarball SHA-256
`3fb06dc496f321749423792066f299205c433b815540c1f23f18368540401640`,
`api/types.go` SHA-256
`2f38141506af42d3ece12a89517ace40077e6327b034128ef7d0ee054ab83d3e`, and
`server/routes.go` SHA-256
`fe40523b0a35b03c12bb8d530d3478e9d08a32e4723366ccaed0148ac8705e5a`
reproduce. Compared selected structs and handlers with the same
brace-matched extraction that reproduces every Research 342 digest.

## Identity

| Surface | Version | Evidence |
| --- | --- | --- |
| Host CLI | client `0.33.3` | `--version` only; no runtime reachable; executable SHA-256 `75b6b83ab9c06712c7097807c91d2bb86a3ef3dd143d59359a5fbb3314970c0a`; host not mutated |
| Official GitHub latest | `0.34.4` | tag commit `b2da9e468af2479058ae18c6d908ed29de410684`; tree `525c37066242cc8feb9f7d909927270b168c6a7c`; tag tarball SHA-256 `c265dfff78cd2fe909f9f9fa0bb21b1e34dfcd354e1bfa13ed8eac2cfce90940` |

Published non-prerelease stables after previous ceiling `0.34.2`:
`0.34.3`, `0.34.4`. First unpublished later stable is `0.34.5`. GitHub
still marks plain `v0.32.2` and `v0.32.10` as prereleases. Keep those
named holes.

| Version | Tag commit | Tag tree | Selected-surface note |
| --- | --- | --- | --- |
| `0.34.2` | `dfabde45` | `f7a55d8e` | previous ceiling; Research 342 hashes reproduce |
| `0.34.3` | `6383a0fa` | `45bb0e75` | additive `ShowResponse.thinking`; ChatRequest comment-only |
| `0.34.4` | `b2da9e46` | `525c3706` | `types.go` identical to `0.34.3`; ChatHandler single-pass format |

Full commits, trees, tarball digests, and the per-hop ledger are frozen in
the
[0.34.4 identity](../../crates/swallowtail-adapter-ollama/tests/fixtures/ollama-0.34.4/identity.json)
and
[protocol](../../crates/swallowtail-adapter-ollama/tests/fixtures/ollama-0.34.4/protocol.json)
fixtures.

## Selected protocol

Five selected routes stay registered. Eight selected structs stay
byte-identical from `v0.34.2` through `v0.34.4` except `ChatRequest`
(comment-only `Think` wording at `v0.34.3`; JSON tag unchanged) and
`ShowResponse` (additive optional `thinking`). `ListHandler`,
`ShowHandler`, and `PsHandler` are byte-identical. `types.go` is
byte-identical from `v0.34.3` through `v0.34.4`.

The catalog decoder does not deny unknown keys. A `v0.34.3` show payload
carrying `thinking.values` / `thinking.default` still parses; admitted
capabilities remain `completion` and `thinking`. The adapter does not map
the advertisement. `GetModelInfo` fills the new field outside
`ShowHandler`.

`v0.34.3` ChatHandler resolves `think` against advertised controls. The
adapter still sends `false` or a mapped mode string and omits `think` when
reasoning is unset. GenerateHandler and OpenAI/Anthropic
`lookupThinking` middleware stay unselected.

`v0.34.4` ChatHandler drops the two-pass structured-output restart and
passes `ThinkingClose` into completion. Leftover `getExistingName` casing
and GenerateHandler stay unselected. Nemotron-H vision, HuggingFace pull,
macOS app, Qwen/Gemma Apple-silicon, and llama.cpp/MLX/XGrammar stay
unmapped.

Decoder specimen remains `ollama-native-v0.14.0-v0.32.1`.

## Decision

Compatible extension of `ollama.native-text-v1`. Raise
`OLLAMA_LATEST_QUALIFIED_VERSION` to `0.34.4`. Qualify published hops
`0.34.3` and `0.34.4`. Keep baseline `0.14.0`, claim id
`ollama.native-runtime-window-2`, AllowUnverified, and exclusions
`0.32.2` and `0.32.10`. Synthetic later-stable UnverifiedNewer is
`0.34.5`.

No new public selected operation. No flattening onto Cloud, generate, or
llama.cpp. Production claims stay at `0.34.2` until the claim edit.

## Sources

- [GitHub `v0.34.4`](https://github.com/ollama/ollama/releases/tag/v0.34.4)
- [GitHub `v0.34.3`](https://github.com/ollama/ollama/releases/tag/v0.34.3)
- [GitHub `v0.34.2`](https://github.com/ollama/ollama/releases/tag/v0.34.2)
- [GitHub `v0.32.10` still prerelease](https://github.com/ollama/ollama/releases/tag/v0.32.10)
- [GitHub `v0.32.2` still prerelease](https://github.com/ollama/ollama/releases/tag/v0.32.2)
- Tagged `api/types.go` and `server/routes.go` at each hop
- frozen `crates/swallowtail-adapter-ollama/tests/fixtures/ollama-0.34.2/`
- Research 342
