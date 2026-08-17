# 124 DeepSeek Harness JSON-RPC Route Qualification

Status: promoted
Owner: Tom
Date: 2026-08-17

## Question

Does DeepSeek AI's new DeepSeek Harness (`dsh`) expose a stable enough machine
boundary for a Swallowtail adapter family, which surface should be first, and
how much of Swallowtail's portable feature set can that surface honestly
claim?

## Method

Sources:

- public repository `deepseek-ai/deepseek-harness` at clone
  `47f943859bef60e4160492346772ded9b24f765a` (2026-08-13 npm-public merge)
- published docs: README, Python SDK, JSON-RPC protocol, ACP, CLI, Web `/api`
  proxy, `dsh-llm-pi-ai`, `dsh-llm-deepseek`
- npm `@deepseek-ai/dsh@0.1.0-rc.6` metadata
- PyPI `deepseek-harness-sdk==0.1.0rc6` plus platform wheel
  `deepseek-harness-runtime-bin==0.1.0rc6` (`py3-none-macosx_14_0_arm64`)
- isolated probe checkout outside the Swallowtail tree
- JSON-RPC handshake without a provider key
- two local Ollama live runs through a throwaway Cordis composition (no
  DeepSeek account)

The live model was host-local `gemma4:12b` via Ollama's OpenAI-compatible
`/v1` endpoint. That qualifies the JSON-RPC runtime and agent loop. It does
not qualify `dsh-llm-deepseek` / `deepseek-official`.

Prompts, tool argument and result bodies, reasoning text, session ids, and
raw JSONL transcripts remain private capture data and are not copied here.

## Installed Artifact

Developer preview. The README warns of compatibility-breaking changes. Pin an
exact wheel and payload digest. Do not use `serverInfo.version` as the
compatibility axis.

| Fact | Value |
| --- | --- |
| Product | DeepSeek Harness (`dsh`) |
| License | MIT |
| Published identity | `0.1.0rc6` |
| Probe SDK | `deepseek-harness-sdk==0.1.0rc6` |
| Probe runtime wheel | `deepseek-harness-runtime-bin==0.1.0rc6` |
| Executable | `dsh-jsonrpc-agent-pkg-macos-arm64` |
| Executable size | 200,248,448 bytes |
| Executable SHA-256 | `ac1c91462518427467bd0a0ca3bf1049df62be0dbe8b0ee8014c6761cb8f80bf` |
| Spawn helper SHA-256 | `21c589109bca43e287df884f3c34ab888033a83927ea7d273949ac5030583f26` |
| Source clone | `47f943859bef60e4160492346772ded9b24f765a` |

Handshake (no provider key):

- `serverInfo.name`: `deepseek-harness-sdk-runtime`
- `serverInfo.version`: `0.0.1` (documented as unvalidated)

The Python SDK is a probe client only. A Swallowtail adapter must spawn the
bundled executable with a host-approved Cordis config. It must not wrap
Python.

## Surfaces

Do not flatten these into one Swallowtail route.

| Surface | Machine boundary | Swallowtail role |
| --- | --- | --- |
| JSON-RPC stdio (`dsh-jsonrpc-agent`) | Owned process; NDJSON JSON-RPC 2.0 | Primary installed-harness route |
| ACP stdio (`@deepseek-ai/dsh-acp`) | Thin ACP: new/prompt/cancel/one-shot permission; no load, resume, catalogue, reasoning, or tool activity on the wire | Later second route if cancel plus permission matter |
| Headless CLI (`dsh --profile headless`) | One fresh session, last assistant text on stdout, exit 0/1, no follow-up | Weaker one-shot; defer |
| Web UI (`dsh web`, `:3080`) | Browser GUI | Not a driver |
| Web `/api` RPC + WebSocket mux | GUI BFF: session list/search/create/history/models/fork/prompt/cancel, subagent interrupt, workspace archive, `llm.models`, attachments | Later Kimi-local-server analogue; not first |
| Direct DeepSeek Open Platform | Existing `deepseek.continuation` | Separate identity; do not merge |

Web `/api` is a real local HTTP+WS API, not static files. It is richer than
JSON-RPC for catalogue, paged history, cancel, fork, and archive. It is also
a GUI host: no OpenAPI, no protocol version, no bearer auth (Host/Origin
fence only), and credentials/settings/directory-picker share `/api`. Sequence
it like Kimi: JSON-RPC first, web local-server later with a method allowlist
that never touches credentials or settings.

JSON-RPC wire (documented and handshake-proven):

- client→server: `initialize`, `session/prompt` (enqueue receipt only),
  `shutdown`
- server→client: `session.event`, `session.status`, `subagent.started`,
  `subagent.finished`
- no protocol cancel or per-session close; abandon is process kill
- no model-catalogue method; caller supplies `provider` and `model`
- `session/prompt` returns `{ messageId }`; clients own the idle interval

## Live Probe Evidence

Both live runs used provider `local-ollama`, model `gemma4:12b`, disposable
workspace, isolated session-root, and a dummy `OLLAMA_API_KEY`. Catalog route
id `ollama` fails with `PI_AI_ERROR` / `No API key for provider: ollama`. A
hand-declared OpenAI-compat route still requires a non-empty dummy bearer.

### Text-only structured run

Composition omitted bash and editor. 4.7s. `finish_reason=completed`.
Assistant text was the requested token. Durable JSONL event types:

`session`, `agent/inbox/spliced`, `turn/start`, `step/start`, `user/message`,
`session/title`, `request/header`, `request/context`, `assistant/chunk`,
`reasoning-chunks`, `text-chunks`, `assistant/message`, `step/end`,
`turn/end`.

`assistant/chunk` kinds: `block-start`, `reasoning-delta`, `block-end`,
`usage`, `finish`. Usage was non-zero. Finish kind `stop`. Header bound the
selected provider, model, and `maxTokens`.

### Tool-enabled structured run

Same runtime with persistent bash, PTY, and `str_replace_editor` under
`danger-full-access`. 165s. One turn, eleven steps, `finish_reason=completed`.
The requested workspace marker file was written. Final assistant text was the
requested token. Workspace otherwise unchanged.

Durable JSONL (668 records):

| Type | Count |
| --- | --- |
| `reasoning-chunks` | 516 |
| `assistant/chunk` | 87 |
| `step/start`, `step/end`, `assistant/message` | 11 each |
| `tool/call`, `tool/result` | 11 each |
| `turn/start`, `turn/end`, `user/message` | 1 each |

Eleven tool calls: `bash` ×7, `str_replace_editor` ×4. Each `tool/call`
carried `callId`, `name`, `arguments` (string blob), `turn`, and `step`.
Matching `tool/result` used `message` plus optional `error`. One editor
result failed with `FS_NOT_FOUND` (`code`, `name` only) and the loop
continued. Ten steps finished `tool-calls`; the last finished `stop`.

Per-step usage was present and increased across the turn (first step 786/448
tokens; last 6055/85).

The Python SDK's live `RunResult.events` count (4626) is much larger than
durable JSONL cardinality. The notification stream unpacks deltas that
persistence packs. Swallowtail must bound the live stream, not assume JSONL
counts.

Subagent notifications were not observed. Session-id reuse was not probed.

## Route Decision

DeepSeek Harness qualifies for a dedicated installed-harness package and
route, distinct from `deepseek.continuation`:

- package: `swallowtail-adapter-deepseek-harness`
- family: `deepseek-harness`
- route: `deepseek-harness.jsonrpc`
- driver: `swallowtail.deepseek-harness.jsonrpc`
- transport: owned process; NDJSON JSON-RPC 2.0 over stdio
- version axis: `deepseek-harness.runtime-bin`
- first qualified point: exact `0.1.0rc6` with the payload digest above
- unverified-newer: no, while the product is an RC preview

Do not start with ACP, headless CLI, Web `/api`, or an extension of
`swallowtail-adapter-deepseek`.

## First Production Subset

One bounded structured run:

- host-approved `dsh-jsonrpc-agent` at exact `0.1.0rc6`
- host-approved Cordis config path (`DSH_CORDIS_CONFIG`); stdout stays
  protocol-only
- explicit `provider` and `model` on `initialize`
- Swallowtail-owned idle interval from prompt enqueue through
  `session.status` idle
- project turn/step lifecycle, assistant text, content-free reasoning
  progress, tool `callId`/name/lifecycle, usage, terminal `completed` /
  `error`
- ignore unknown event types
- never project tool argument or result bodies, reasoning text, or raw
  JSONL into diagnostics
- cancellation is force-stop of the owned process; do not advertise a
  native cancel method
- joined process cleanup on `shutdown` or kill

The probe used a local OpenAI-compat model because no DeepSeek key was
available. Production access remains host-admitted: DeepSeek official key,
another `dsh-llm-pi-ai` catalog route, or a custom endpoint. The route must
not imply DeepSeek-official SSE behavior from the Ollama smoke.

Do not default `danger-full-access`. Write and shell authority belong to the
approved composition, recorded as prepared evidence.

## Deferred

- ACP stdio (native cancel and one-shot permission; thinner activity)
- Web `/api` local-server (catalogue, history, cancel, fork, archive)
- headless CLI one-shot
- session-id interactive continuity (documented; not live-probed)
- subagent topology (protocol notifications exist; not observed)
- model catalogue, load/import, per-session close
- DeepSeek-official adapter live qualification
- consumer tools, questions, and permission exchange
- protocol-version negotiation

## Contract Fit

No new provider-neutral contract is required for the first structured-run
subset. Contracts 005-006, 009-010, 023, 029, 032-033, 037, 039-041,
044-045, 051, and 052 already govern owned-process harness runs, prepared
evidence, activity, usage, and fail-closed diagnostics.

The JSON-RPC codec can live in the new adapter, as Command Code NDJSON did,
until a second consumer needs a shared crate.

Contract 036 still requires architecture/package review before the package
enters the workspace release set. Immutable `v0.3.2` must not be described
as containing this route.

## Recommendation

Promote DeepSeek Harness into g03 as an installed-harness foundation:
`deepseek-harness.jsonrpc` at exact `0.1.0rc6`. Evidence is sufficient for a
bounded structured-run driver with reasoning progress, text, harness-owned
tool activity, usage, and typed step errors. Keep ACP and Web `/api` as
later distinct routes. Keep `deepseek.continuation` unchanged.

Next planning move: spec and contract the selected JSON-RPC subset, then
compile a g03 tranche after the operator confirms sequencing against the
current evidence gate. Freeze redacted fixtures from the success captures
during that tranche; do not commit private transcripts into research.
