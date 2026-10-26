# Ollama 0.40.11 Attached Identity and Qualification

**Task:** `ollama.attached` only, `swallowtail-adapter-ollama`  
**Observed:** 2026-10-26  
**Previous qualified ceiling:** `0.40.1` (`AllowUnverified`)  
**Result:** compatible extension through official GitHub stable `0.40.11` on
the existing adapter-private revision
`ollama.native-text-v1.manifest-list-runner`.

## Channel and artifact identity

The selected channel is official GitHub non-prerelease releases for
[`ollama/ollama`](https://github.com/ollama/ollama/releases). The official
[`releases/latest` endpoint](https://api.github.com/repos/ollama/ollama/releases/latest)
returned `v0.40.11` with `draft=false` and `prerelease=false`, published
`2026-10-24T00:30:13Z`, release id `422414238`.

| Tag | GitHub release ID | Published at | Commit | Tree | Source archive SHA-256 |
| --- | ---: | --- | --- | --- | --- |
| `v0.40.1` | 406237079 | 2026-10-07T23:22:59Z | `cf2a313a298066d572c36812e5ad30a21c0db13b` | `6ae219abaf2a32dc26c8dc14d71357b92532dec2` | `675cfca761a964d3b40d1331c006fd8a630bdb0b5a9b605ace3494e7d47c3e88` |
| `v0.40.2` | 407063639 | 2026-10-08T16:53:47Z | `b061384d90ff455462bc32745dd0de479de717a3` | `c3e9a02ff82dfc87feff6b5738a1493ffce3569a` | `f40ca583099dd166359a0807e266b682f4f08ef18911959a55801fe2b4c6d71d` |
| `v0.40.3` | 409273105 | 2026-10-11T01:39:04Z | `eab97e9f92b9a25c2d52d2cc6c1b1c99bd9fae21` | `d9794b48a90fda6d822c41025458108079ea74a9` | `2c0eaba5f82b6eba7eb45338494f7dc3496bde674c8f487b20bd269695a362ba` |
| `v0.40.4` | 410905890 | 2026-10-12T19:00:12Z | `45e6011815c74acde6a346512856b69dd79dd22b` | `1b1e8e3162b0c17fef003b695a3ce38623f855e6` | `a09dcf7be0402f6860cfa6a5ed107be9d6f5ac0baf3a22d1f09be02c19b5fab7` |
| `v0.40.5` | 413093320 | 2026-10-14T19:45:22Z | `d919272e53593d1d55d40e1180eaa06dfbede3ac` | `580493dd6669e1dcbff2bc25e266a679234a5b48` | `3e84e7d750b700fb65256643085a9a121571aa5f98734a608d94d8dc6eb18029` |
| `v0.40.6` | 415399896 | 2026-10-16T22:00:46Z | `2476d5ead197abe04629651028f83d48dc75d2b2` | `135a03c8ab75794cbae99a11d6df4bdb6fae9bb0` | `5adad63f326e5a3911d5248e923ac03def72b6ba5ba80fe1c6089cf26b9291c3` |
| `v0.40.7` | 416126438 | 2026-10-18T00:28:57Z | `2cd63f7a15039ccd790e97b31edc091fa1707c56` | `9aeaf6275b4e506c9deb91ee28d9fe991f94fccd` | `9db79f02285556ace3f724c1609e6fbf3d17a9812049400f22567ced591b2c16` |
| `v0.40.8` | 419777982 | 2026-10-21T16:20:30Z | `f580b5484a58c8370c35d669555bf887783a3f68` | `1b87f9644d38f5ec024569655ac26a102e7d5f7e` | `e4a22341210a7590c84465173fac12e38901e3f1d30e108e9f470737703b7703` |
| `v0.40.9` | 420047436 | 2026-10-22T01:33:10Z | `5e31438d5d2223d8338595b366b7434ef3539708` | `85cabb889ce788da9a0f9c8204580ef7439ca89f` | `9e99fd4c86ebb5ff948a7f22a667b0c0a5d80a14b86aaf232f282c092e0d6ca7` |
| `v0.40.10` | 421968618 | 2026-10-23T15:03:59Z | `f52b9e742d8c9a308f7b23cd1f0ab52c7ddb694d` | `21eaa2eec02288ef2f1631db3f0b97e1d580206a` | `e1ec9d983e35b992f2dc9e01ff0b2a06a064fe7d9d502a52995f7c37a38cb937` |
| `v0.40.11` | 422414238 | 2026-10-24T00:30:13Z | `675150d6ad2747df1e218ef59de9194bc4a7e538` | `1dd0e013eeb9ff6acc9deaa37f965bb7a98e62fc` | `9733d6230d8c8a75ccdf368c53132d90ee49009ce677776d897e96b50cc1c654` |

`v0.40.1` matches Research 418. Release id `407063639` was prerelease in that
record and is now the non-prerelease `v0.40.2` stable. `v0.40.2-rc0` is a
different commit and stays a prerelease. No `v0.40.12` or `v0.41.*` tag was
present. Semantic prereleases stay incompatible.

Complete recursive trees and the ten hop ledgers are in
[dist-inventory.json](../../crates/swallowtail-adapter-ollama/tests/fixtures/ollama-0.40.11/dist-inventory.json).
The classification ledger SHA-256 is
`11e5948c6e040db9d3c75354874b4f782e6c8e32c0bcd5441677c01069c74eff`.

The host had no `ollama` executable on `PATH`. No runtime was reachable. No
local runtime, model, credential, or host state was changed.

## Hop classification

Selected routes stay `GET /api/version`, `GET /api/tags`, `GET /api/ps`,
`POST /api/show`, and `POST /api/chat`. Those registrations are present at
`0.40.1` and `0.40.11`. `ChatRequest`, `ChatResponse`, `Message`,
`ShowRequest`, `ListResponse`, and `ListModelResponse` keep the same struct
text. `compatmigrate/migrate.go` blob `d6a5ce939218e4f6092b30322cb39b1fc9dad35e`
and `manifest/paths.go` blob `cf3bfa2569bf1a3fbc69ef12455566e259515a2d` are
byte-identical at every point.

`0.40.1` → `0.40.2`: 5 changed files. `server/model_list.go` lists only the
locally preferred manifest-list child and skips the llamacpp downgrade-guard
tag. The returned row still carries the parent display name, the child
digest, format, family, and runner. `routes.go` and `api/types.go` are
byte-identical.

`0.40.2` → `0.40.3`: 6 changed files. `scheduleRunnerForModel` comments out
`StartLocalCompatibilityMigration`. The five native handlers stay. The
migration source is not edited and is not called.

`0.40.3` → `0.40.4`: 18 added, 6 removed, 90 changed. Chat resolves the
runner-pinned model once and schedules that model. Message assembly moves to
`chatMessagesForRequest` with the same role, content, system prepend, and
think-tag filter. `parserNameForModel` still selects `harmony` when the model
parser is empty. System One types and `POST /v1/systemone` are unselected.
Store open and digest checks are restructured and do not rename catalogue
fields.

`0.40.4` → `0.40.5`: `routes.go` only wraps unselected `/v1/systemone` with
cloud passthrough.

`0.40.5` → `0.40.6`: no selected `api`, route, list, image, schedule, or
manifest file changes. Download, GGUF file-type, and runner-job files stay
off the native HTTP structs.

`0.40.6` → `0.40.7`: `ShowResponse` gains omitempty `template_source`, and
the show builder copies renderer and parser. The adapter reads capabilities,
details, and remote fields only. Native chat NDJSON is unchanged.

`0.40.7` → `0.40.8`: Flint model-family files only. Selected route files stay
byte-identical.

`0.40.8` → `0.40.9`: speech capability, `POST /v1/audio/speech`, and a ps
context length of 0 for speech models are unselected. The adapter does not
read context length and still rejects an unknown capability string. Native
chat thinking rejection is unchanged apart from a log level.

`0.40.9` → `0.40.10`: safetensors show and Gemma renderer edits stay off the
GGUF text fields the adapter reads.

`0.40.10` → `0.40.11`: `ShowResponse` gains omitempty `voices`, set only when
the model advertises speech.

Cloud, OpenAI, Anthropic, CLI, desktop, MLX, llama.cpp, decision, and build
files are classified in the hop ledger and stay outside the attached native
facade.

## Adapter mapping

Claim id `ollama.native-runtime-window-2`, baseline `0.14.0`, exclusions
`0.32.2` and `0.32.10`, and `AllowUnverified` stay. Deprecated
`0.14.0..=0.34.4` and `0.35.0..=0.35.1` stay on `ollama.native-text-v1`.
Maintained `0.40.0..=0.40.11` stays on
`ollama.native-text-v1.manifest-list-runner`. The adapter still binds the
preflight tag and digest, pins `ggml` or `llamacpp` from the matching row,
observes extra same-tag rows when the server sends them, and fails closed on
`remote_model` / `remote_host` and on identity drift. No public operation was
added. Unpublished `0.34.5` and `0.35.2` through `0.39.x` stay interior gaps.
Synthetic `0.40.12` is the first visible `UnverifiedNewer` point. `0.41.0`
remains above the ceiling.

## Verification boundary

No Ollama server was started. There was no provider prompt, live catalogue or
session, model download, inference, installation, credential access, host
mutation, or model-store write. Proof is source identity plus the existing
provider-free HTTP fixtures.

Re-probe of `releases/latest` at `2026-10-26T10:35:48Z`, immediately before
the identity commit, still returned `v0.40.11` (release `422414238`,
published `2026-10-24T00:30:13Z`, commit
`675150d6ad2747df1e218ef59de9194bc4a7e538`). No In-Run Latest Movement.
Research 448 is unused on canonical GitHub main.
