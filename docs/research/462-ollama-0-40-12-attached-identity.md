# Ollama 0.40.12 Attached Identity and Stop

**Task:** `ollama.attached` only, `swallowtail-adapter-ollama`
**Observed:** 2026-10-28
**Previous qualified ceiling:** `0.40.11` (`AllowUnverified`)
**Result:** stop. The maintained ceiling remains `0.40.11`; official stable
`0.40.12` is `UnverifiedNewer`. Selected dependency and renderer effects need
the finite adaptation below before qualification.

## Channel and artifact identity

The selected channel is official GitHub non-prerelease releases for
[`ollama/ollama`](https://github.com/ollama/ollama/releases). The official
[`releases/latest` endpoint](https://api.github.com/repos/ollama/ollama/releases/latest)
returned `v0.40.12` with `draft=false` and `prerelease=false`, published
`2026-10-27T00:07:12Z`, release id `424898500`.

| Tag | GitHub release ID | Published at | Commit | Tree | Source archive SHA-256 |
| --- | ---: | --- | --- | --- | --- |
| `v0.40.11` | 422414238 | 2026-10-24T00:30:13Z | `675150d6ad2747df1e218ef59de9194bc4a7e538` | `1dd0e013eeb9ff6acc9deaa37f965bb7a98e62fc` | `9733d6230d8c8a75ccdf368c53132d90ee49009ce677776d897e96b50cc1c654` |
| `v0.40.12` | 424898500 | 2026-10-27T00:07:12Z | `f067033f9126993b6172bdfffa71a74ad693fda4` | `5b73640f43d4a4b0a5a03be5752693e654c0f4b8` | `b625d6dd6b4650366872b2d0263167993411b734838d9b773a22076a8cf7f3fc` |

`v0.40.11` matches Research 448, including the re-downloaded tag tarball.
`v0.40.12-rc0` is commit `6c15b26ba0d053eecbf63b5fc5a6cbf262b1014c` and stays
a prerelease. `v0.40.12-rc1` is the same commit as the stable and stays an
unqualified prerelease version string. No `v0.40.13` or `v0.41.*` tag was
present. Semantic prereleases stay incompatible.

The hop ledger is in
[dist-inventory.json](../../crates/swallowtail-adapter-ollama/tests/fixtures/ollama-0.40.12/dist-inventory.json).
`v0.40.11` has 1513 files and 1708 tree entries. `v0.40.12` has 1577 files
and 1776 tree entries. The hop adds 66 files, removes 2, and changes 70.
The classification ledger SHA-256 is
`452ad16d14e2be87e814d996579437aaddfcb5f7710a001516c49ef7b5fa31aa` (reclassified after selected-path review).

The host had no `ollama` executable on `PATH`. No runtime was reachable. No
local runtime, model, credential, or host state was changed.

## Hop classification

Selected routes stay `GET /api/version`, `GET /api/tags`, `GET /api/ps`,
`POST /api/show`, and `POST /api/chat`. Those registrations are present at
both points. `api/types.go` blob `1aa71342a372fc2a0902cc55f43afe91c3a55d06`
is byte-identical, so `ChatRequest`, `ChatResponse`, `Message`,
`ShowRequest`, `ShowResponse`, `ListResponse`, and `ListModelResponse` keep
the same struct text. `server/model_list.go` blob
`6be2862816f5ba2d6101812a1a03ee98fc079b8d`, `server/sched.go`,
`compatmigrate/migrate.go` blob `d6a5ce939218e4f6092b30322cb39b1fc9dad35e`,
and `manifest/paths.go` blob `cf3bfa2569bf1a3fbc69ef12455566e259515a2d` are
byte-identical. The migration call stays commented out. `/api/tags` still
lists only the locally preferred manifest-list child.

`server/routes.go` does not change `ChatHandler`. Show still sets omitempty
`voices` only when the model advertises speech, and now also appends config
voices. The adapter does not read `voices`. Safetensors show copies context
and embedding lengths into model info. The adapter does not read model info
and still rejects a non-gguf selected row. Ps sets context length to 0 only
for speech-only models. The adapter does not read context length.

`server/images.go` adds `audio` to public capabilities only for decision
models. Those models still omit `completion`, so the adapter rejects them.
Non-decision capability lists are unchanged. Unknown capability strings still
fail closed.

OpenAI `/v1/audio/transcriptions` no longer wraps `ChatHandler`, and
`/v1/audio/translations` is new. Both stay off the native facade. The empty
tool-call content fix is in the OpenAI converter. `ollama launch` stays off the native facade. No public Swallowtail operation
was added. Renderer and model internals are not bounded by identical HTTP
structs; their selected reachability remains an open qualification gap.

## Selected-path stop and adaptation

`server/sched.go` selects `llm.NewLlamaServer`; `llm/server.go` sends GGUF
models to `NewLlamaServerRunner`. `llama/server/CMakeLists.txt` reads
`LLAMA_CPP_VERSION` and fetches that tag by default; an explicit source override
is also possible. The frozen pins are `b11845` at 0.40.11 and `b11901` at
0.40.12. The dependency tag identities, intervening selected source changes,
and their native text effects have not been traced. The pin is selected, not
an unselected build detail.

`server/prompt.go` dispatches a nonempty `Config.Renderer` independently of
`Config.Parser`. `server/routes.go` separately considers renderer, parser,
Harmony and Go-template predicates. `OLLAMA_GO_TEMPLATE` defaults true when
applicable; an attached runtime controls its environment. Registration includes
`lfm3`, `lfm3-omni`, and `lfm3-audio`, and the Gemma renderer changed. The
parser capability rejection does not establish that each renderer is unreachable
for an admitted GGUF completion model. Text effects and the exact admitted-model
and environment boundaries remain open.

The planner-owned adaptation must resolve both llama.cpp tags to immutable
commits, inventory and classify their selected GGUF text-path delta and effects,
then trace each changed renderer and parser through model capability, config,
template and approved environment boundaries. Freeze mutation-sensitive source
identities and effects; qualify 0.40.12 only after those gates close. This is
static source work unless a separate authority grants live evidence. Existing
identity and the previous 0.40.11 claim remain useful and intact.

## Adapter mapping

Claim id `ollama.native-runtime-window-2`, baseline `0.14.0`, exclusions
`0.32.2` and `0.32.10`, and `AllowUnverified` stay. Deprecated
`0.14.0..=0.34.4` and `0.35.0..=0.35.1` stay on `ollama.native-text-v1`.
Maintained `0.40.0..=0.40.11` stays on
`ollama.native-text-v1.manifest-list-runner`. The adapter still binds the
preflight tag and digest, pins `ggml` or `llamacpp` from the matching row,
observes extra same-tag rows when the server sends them, and fails closed on
`remote_model` / `remote_host` and on identity drift. Unpublished `0.34.5`
and `0.35.2` through `0.39.x` stay interior gaps. Official stable `0.40.12` is `UnverifiedNewer`. `0.41.0` remains above the ceiling.

## Verification boundary

No Ollama server was started. There was no provider prompt, live catalogue or
session, model download, inference, installation, credential access, host
mutation, or model-store write. Proof is source identity plus the existing
provider-free HTTP fixtures.

Re-probe of `releases/latest` at `2026-10-28T09:38:37Z`, immediately before
the identity commit, still returned `v0.40.12` (release `424898500`,
published `2026-10-27T00:07:12Z`, commit
`f067033f9126993b6172bdfffa71a74ad693fda4`). No In-Run Latest Movement.
Research 462 is the pre-assigned unused number for this family.
