# Ollama Selected llama.cpp Pin and Rendered-Chat Qualification

**Task:** `ollama.attached` only, `swallowtail-adapter-ollama`
**Observed:** 2026-11-02
**Previous qualified ceiling:** `0.40.11` (`AllowUnverified`)
**Result:** adaptation complete. The selected GGUF text-runner dependency and
rendered-chat renderer gates that Research 462 named are closed at source
level. The maintained ceiling stays `0.40.11`; official stable `0.40.12`
remains `UnverifiedNewer` and its claim extension is a separate Contract 029
upgrade-workflow task. No claim changed and no vendor ran.

This record performs the finite adaptation Research 462 required; it is not
completed qualification of `0.40.12`.

## Pin identity

Research 462 froze the Ollama hop and identified the selected dependency pin
(`LLAMA_CPP_VERSION`, blob `80291d7f1423525ef37228948f78fb69f5407aa2` at
`0.40.11`, blob `98bc1f1660e3264d1df83952b9e8a452e22a6088` at `0.40.12`).
This record resolves both pins to immutable identities in upstream
`ggml-org/llama.cpp`. Both tags are lightweight commit tags; the refs were
cross-checked against the GitHub `git/ref` API after a shallow fetch of the
exact tags.

| Pin | Selected at | Commit | Tree |
| --- | --- | --- | --- |
| `b11845` | Ollama `0.40.11` | `9ff0d7e90f0faaa7c16017e6dcf9cf91c50a100e` | `60bf42018da47404b7260ee2bebd108b6c1b511f` |
| `b11901` | Ollama `0.40.12` | `5320c87ff0d994845b44ce7976f10055ec697447` | `3e71dcf38c6806ff99c75d3b1879f6501cb0834f` |

The pins span 56 upstream commits and a 263-file delta (20 added, 1 removed,
242 modified). The frozen corpus is
[ollama-llama-cpp-pin](../../crates/swallowtail-adapter-ollama/tests/fixtures/ollama-llama-cpp-pin/README.md):
`identity.json` freezes the pin identities, the delta ledger SHA-256 is
`36ca2887be600043645811d063a0e0a62e13409bb2b84b6f605cd2f0daf89764`, and
`llama-server-cmake.txt` freezes the pin consumer.

## Selected runner path

`server/sched.go` selects `llm.NewLlamaServer`, which sends every GGUF model
to `NewLlamaServerRunner`. The Go runner spawns the upstream `llama-server`
binary from `lib/ollama/` and speaks to it over loopback HTTP:
`/health`, `/completion`, `/v1/chat/completions`, `/apply-template`,
`/v1/embeddings`, `/tokenize`, and `/detokenize`. `llama/server/CMakeLists.txt`
— byte-identical across the Ollama hop (blob
`9153c9bf019548f7015305b773a9fed2a61f3fcc`) — reads `../../LLAMA_CPP_VERSION`
and fetches that tag by default with `FetchContent` from
`https://github.com/ggml-org/llama.cpp.git`. The whole selected delta on the
dependency side is therefore the pin movement itself: `llama`,
`llama/compat`, and `llama/server` trees are identical across the Ollama hop,
the compat hook patch blob `6610052094be088cc8f2452b4dc7960be7e1f42f` is
unchanged, and the Go selected surface (`server/sched.go`, `llm/server.go`,
`llm/llama_server.go`, `server/prompt.go`, `server/renderer_resolution.go`,
`server/model_thinking.go`, `server/model_show_cache.go`,
`envconfig/config.go`) is byte-identical.

The compat shim applies `001-llama-cpp-hooks.patch` at build time; all ten
hunks keep their distinctive context lines in the `b11901` blobs
(`src/llama-model-loader.cpp` moved two lines between the pins). Applicability
stays a build-time concern for source builds; it is not a claim element.

## Pin delta classification

The full ledger is
[delta-inventory.json](../../crates/swallowtail-adapter-ollama/tests/fixtures/ollama-llama-cpp-pin/delta-inventory.json).
Selected classes total 160 files; unselected classes total 103.

| Class | Files | Effect on the selected native text path |
| --- | ---: | --- |
| `selected-ggml-backend` | 85 | Compute kernels and core ggml ops (CUDA, Metal, Vulkan, HIP, SYCL, OpenCL, WebGPU, hexagon; `MUL_MAT_ID` skip, fused rope/GLU): numeric and performance effects on every generated token; no wire-shape change |
| `selected-llama-core` | 25 | Tokenizer (`llama-vocab`), sampler (dry sequence breaker fix), KV-cache `no_alloc` handling, graph, context, model load, arch registry, and the new DeepSeek V4.1 graph builder |
| `selected-mtmd-media` | 18 | Projector path only, reachable when a projector is loaded; the driver text path never triggers it for text-only GGUF rows |
| `selected-server-surface` | 9 | llama-server entrypoint refactor (#31113) and shared helpers behind `/completion`, `/v1/chat/completions`, `/tokenize`, `/detokenize`, and the schema endpoint; `/models` and decision endpoints stay uncalled by the driver |
| `selected-common-jinja` | 6 | Minja engine: parser recursion limit, `select`/`selectattr` argument fix, value handling — applied to GGUF chat templates in native chat mode |
| `selected-common-grammar` | 5 | JSON-schema and PEG/grammar recursion depth limits behind provider-native JSON Schema structured runs |
| `selected-common-chat` | 3 | OpenAI-compatible message parsing on the native chat path: extra-key pass-through, a 128-level JSON nesting guard, DeepSeek V4.1 render handling |
| `selected-common` | 3 | Startup argument parsing for the driver's flag set, common init |
| `selected-build-graph` | 3 | CMake wiring only, for targets llama-server links (`common`, `mtmd`, `ggml-opencl`); `common/CMakeLists.txt` adds the file-lock sources to `llama-common`, while those sources stay on the unreachable remote-download path |
| `selected-server-docs` | 2 | Server documentation inside `tools/server`; no serving behavior |
| `selected-server-test` | 1 | Python server test harness; not part of the shipped runner |
| `unselected-*` | 103 | Web UI (`--no-webui`, disabled build), llama.cpp tests, Python conversion, CI/contrib, HF download cache (the driver passes local blob paths and launches `--offline`), quantize tool path, tools that Ollama's install scope never installs |

Native text effects are real but bounded: tokenization, sampling, cache
behavior, native-template application, and schema-constrained decoding all
changed inside the runner, while every wire shape the driver observes — the
`/completion` and `/v1/chat/completions` request/response contracts as mapped
by the byte-identical Go runner — stayed the same. Those provider-internal
text effects are exactly what makes `0.40.12` a qualification question
instead of a byte-identity extension.

## Rendered-chat trace

`chatModeForModel` picks `chatExecutionModeRendered` when
`usesOllamaRenderedChat(m)` — `Config.Renderer != ""`, `Config.Parser != ""`,
Harmony, or the Go-template predicate — and `chatExecutionModeNative`
otherwise. Both predicate inputs are unchanged across the hop.

- Rendered mode: `renderPrompt` dispatches on `Config.Renderer` through
  `resolveRendererName` independently of `Config.Parser`; the rendered string
  goes to `/completion`, and the response text is parsed by
  `ParserForName(parserNameForModel(m))` into `Content`, `Thinking`, and
  `ToolCalls` before the native NDJSON reaches the adapter. llama-server
  starts with `--no-jinja --chat-template chatml`, a startup-only placeholder.
- Native mode: messages go to `/v1/chat/completions` with `cache_prompt`;
  llama-server applies the GGUF chat template through minja on the C++ side
  and returns `content`/`reasoning_content`/`tool_calls` deltas that the Go
  runner maps directly.

Changed renderer and parser effects, all from the Ollama hop, not the pin:

- `lfm3`, `lfm3-omni`, `lfm3-audio` renderer and parser names are new at
  `0.40.12`. They are reachable only for models whose manifest config names
  them; at `0.40.11` such a manifest fails chat with an unknown-renderer
  error. Model families are a separate axis from the runtime version
  (Contract 031), so this gates model support, not the runtime claim.
- `gemma4-large` gains `transcribeThenTranslate`, used only by
  `AudioPrompterForRenderer` on the new OpenAI `/v1/audio/transcriptions` and
  `/v1/audio/translations` endpoints — unselected for the native facade.
  `gemma4_audio.go` and `audio.go` serve only those endpoints.
- `lfm3-omni` maps to the existing `Qwen3CoderParser` on the response side.
- `server/images.go` adds an `audio` public capability only for decision
  models, which still omit `completion`, so the adapter's rejection is
  unchanged. Unknown capability strings still fail closed.
- `server/routes.go` leaves `ChatHandler` unchanged; ps zeroes context length
  only for speech-only models now, and show appends config voices — the
  adapter reads neither field.
- Thinking resolution (`server/model_thinking.go`, byte-identical) dispatches
  on the resolved renderer before Go-template and Harmony fallbacks; the
  adapter's think controls keep failing closed on unsupported models.

Template and environment boundaries: `shouldUseGoTemplate` reads
`envconfig.GoTemplate`, a `BoolWithDefault("OLLAMA_GO_TEMPLATE")` that
defaults true; a model with a Go template and no renderer/parser/harmony
renders Go-side unless `PreferChatTemplate` (set when the GGUF chat template
should win and the env is unset). An attached runtime controls its own
environment; the adapter neither sets nor inspects these variables. Native
think controls reach llama-server as `chat_template_kwargs`
(`enable_thinking`, `reasoning_effort`).

## Mutation and cache clarification

Driver-issued mutation stays what Contract 031 rules: inference may load the
selected model, refresh its timer, or evict another model; the commented-out
compatibility-migration call stays commented out at both points. The compat
shim's `translate_metadata` mutates the runner's in-memory GGUF view at load
only; serving issues no model-store write. Separately,
`server/model_show_cache.go` (blob
`f829e477497da4d23e2dd8496951ea61b96a00ed`, byte-identical across the
hop) keeps a process-local `/api/show` response cache. Local entries are
keyed by canonical model name, runner, and verbose flag, and each entry
records the manifest digest as its freshness boundary: a content change
replaces the previous response instead of accumulating under an old
digest. Cloud entries are keyed by normalized cloud base model name and
verbose flag with stale-while-revalidate. A show observation can
therefore be preexisting runtime-derived metadata assembled at an
earlier time, not a fresh computation per request; its freshness
boundary is the recorded manifest digest, entries are cloned at read
and write boundaries, and nothing is written to `~/.ollama/cache/show`.
The adapter's show observations remain source-scoped observations under
Contracts 007 and 020; this cache is provider-internal state, not a
Swallowtail effect and not a host mutation.

## Override axes

The pinned default build path is FetchContent at the version-file tag plus
the compat patch and compat sources. A source-built deployment can depart
from it: `OLLAMA_LLAMA_CPP_SOURCE` (or `FETCHCONTENT_SOURCE_DIR_LLAMA_CPP`)
substitutes llama.cpp source — the compat patch still applies to a
`FETCHCONTENT_SOURCE_DIR_LLAMA_CPP` override and is deliberately skipped under
`OLLAMA_LLAMA_CPP_SOURCE` — `OLLAMA_LLAMA_CPP_SKIP_COMPAT_PATCH` skips the
patch, and `OLLAMA_LLAMA_CPP_COMPAT=0` disables the hook bodies at load time.
Under Contracts 029 and 031 the claim binds the exact runtime version on the
`ollama.runtime` axis, not a binary inventory: official platform builds
compile the same pinned sources with different toolchains and backends, and
this record draws no inference from shipped binary lists. Deployments that
override the dependency source or disable the compat layer leave the default
build path the claim describes.

## Research 448 reassessment

Research 448 classified llama.cpp and build files outside the attached native
facade and described its 0.40.9 to 0.40.10 Gemma renderer edits as outside
the GGUF text fields the adapter reads. Both classifications held for the
HTTP struct surface they examined and do not establish that the runner and
renderer effects are unselected: the `LLAMA_CPP_VERSION` pin is a selected
dependency, and the Gemma renderer edits — like this hop's renderer changes —
sit on the rendered-chat path, upstream of the fields the adapter reads.
Research 462 recorded that correction; this record completes it by resolving
and classifying the pin delta. Nothing here revokes or requalifies the
`0.40.11` claim: that claim was made on the selected-surface evidence of its
time and stays intact, and the rendered-chat effects it did not bound belong
to the `0.40.12` qualification question.

## Adapter mapping

Claim id `ollama.native-runtime-window-2`, baseline `0.14.0`, exclusions
`0.32.2` and `0.32.10`, segments, and the prepared tag, digest, and runner
binding all stay unchanged. Deprecated `0.14.0..=0.34.4` and
`0.35.0..=0.35.1` stay on `ollama.native-text-v1`. Maintained
`0.40.0..=0.40.11` stays on `ollama.native-text-v1.manifest-list-runner`.
Official stable `0.40.12` remains the visible `UnverifiedNewer` point, and
`0.41.0` remains above the ceiling. The real prepared-driver fakes cover the
mapping unchanged under the extended `validate:current-ollama-attached`
selector.

## Verification boundary

No Ollama or llama.cpp runtime was started and no vendor process ran. There
was no provider prompt, live catalogue or session, model download, inference,
installation, credential access, host mutation, or model-store write. Proof
is source identity — pin commits and trees, Ollama blob IDs, and the frozen
corpora — plus the existing provider-free HTTP fixtures.

## Where 0.40.12 stands

Research 462's stop required exactly this record's work: resolve both llama.cpp
tags to immutable commits, inventory and classify the selected GGUF text-path
delta and effects, and trace each changed renderer and parser through
capability, config, template, and environment boundaries. Those gates are now
closed at source level. Extending the maintained window to `0.40.12` is the
next Contract 029 upgrade-workflow task: a reviewed claim edit binding this
corpus, with the runner's provider-internal text effects classified rather
than erased. This lead grants no execution, release, or authority change.
