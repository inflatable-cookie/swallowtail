# Research 313: Ollama 0.34.0 Identity

Status: complete; identity evidence only. Production claim changes land in
the g05.063 claim batch after this record.

Observed 2026-09-14 on the `ollama.runtime` axis:

- Host `ollama --version`: client `0.33.3` from a 50-byte launcher shim at
  `/usr/local/bin/ollama` (SHA-256
  `75b6b83ab9c06712c7097807c91d2bb86a3ef3dd143d59359a5fbb3314970c0a`).
  `Warning: could not connect to a running Ollama instance`. No runtime was
  reachable, no server was started, and the host was not mutated.
- Official GitHub latest: `v0.34.0`, published 2026-09-05T23:49:00Z,
  `prerelease: false`. Rechecked immediately before the identity commit;
  no newer stable exists.
- Published non-prerelease stables after the prior `0.32.15` ceiling are
  exactly `0.33.0`, `0.33.1`, `0.33.2`, `0.33.3`, and `0.34.0`, each
  corroborated by its tag, commit, and tree. GitHub still marks plain
  `v0.32.2` and `v0.32.10` as prereleases; those stay named exclusions.

## Official hop identities

| Version | Tag commit | Tag tree | Selected-file delta vs `0.32.15` |
| --- | --- | --- | --- |
| `0.33.0` | `ebf200f9` | `c537fd75` | none: `types.go` and `routes.go` byte-identical |
| `0.33.1` | `13f2fb8c` | `b3e63b82` | `types.go` gains unselected experimental recommendation mappings; `routes.go` identical |
| `0.33.2` | `f96e7aa0` | `c35ae275` | none vs `0.33.1` |
| `0.33.3` | `b79067b0` | `69ea0f1f` | STOP: `Metrics.prompt_eval_cached_count` reaches selected `/api/chat` |
| `0.34.0` | `d8ab4b4f` | `61d8627a` | STOP inherits; rest unselected |

Tagged sources were retrieved as official tag tarballs into `/tmp` and
never executed. Full commits, trees, file hashes, and the per-hop ledger
are frozen in the
[0.34.0 identity](../../crates/swallowtail-adapter-ollama/tests/fixtures/ollama-0.34.0/identity.json)
and
[protocol](../../crates/swallowtail-adapter-ollama/tests/fixtures/ollama-0.34.0/protocol.json)
fixtures.

## Selected-surface classification

All eight selected structs (`ChatRequest`, `ChatResponse`, `Message`,
`ListResponse`, `ProcessResponse`, `ShowRequest`, `ShowResponse`,
`Options`) are byte-identical from `v0.32.15` through `v0.34.0` under the
same struct-block extraction that reproduces every Research 174 digest.
The five selected routes stay registered at every hop.
`server/routes.go` is byte-identical from `v0.32.15` through `v0.33.2`.

Hops `0.33.0` through `0.33.2` are a compatible extension. The only
`api/types.go` delta in that range adds `Mappings` to the experimental
`ModelRecommendationsResponse` plus two app-route-preference types; the
`/api/experimental/model-recommendations` handler predates this window and
stays unmapped. Release-note items in range (Claude Desktop gateway
proxy, prefill restore-point caching, DeepSeek harness `npx` fallback,
MLX/Qwen3.8 structured output, macOS single-instance handoff) feed no
selected operation.

`0.33.3` is the stop. `Metrics` gains optional
`prompt_eval_cached_count`, plumbed from llama-server `cache_n` into the
selected `POST /api/chat` (and generate) NDJSON records. The strict
`ollama.native-text-v1` chat decoder fail-closes on unmapped keys, so a
valid `0.33.3` terminal record carrying that key is rejected while the
identical record without it decodes to `Finished(stop)` plus usage; both
directions are proved by the committed identity test. The `0.33.3`
generation-defaults change fills gaps only (explicit request options still
win) and the intermediate-metrics gating only fires when `format` is
requested, which the adapter never sends; both are bounded unmapped. The
`0.34.0` remainder (Codex desktop proxy route, `/v1/responses` compaction
and middleware, recommendation thinking advertisement) is likewise
unselected.

## Contract 029 decision

`compatible-extension-with-stop` on `ollama.native-text-v1`. Keep the
baseline `0.14.0`, claim id `ollama.native-runtime-window-2`, posture
`AllowUnverified`, decoder specimen `ollama-native-v0.14.0-v0.32.1`, and
exclusions `0.32.2` and `0.32.10`. Qualify hops `0.33.0`, `0.33.1`, and
`0.33.2` and raise the ceiling to `0.33.2`. Leave `0.33.3` and `0.34.0`
`UnverifiedNewer` with the named decoder reason above. Concrete follow-up:
a decoder-tolerance task that accepts and ignores the additive cached-count
key (or maps it), then re-qualifies `0.33.3` through current official. No
prompt, live session, login, installation, host update, download execution,
or credential was used.
