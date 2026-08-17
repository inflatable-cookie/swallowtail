# DeepSeek Harness JSON-RPC Prepared Integration

Use `swallowtail-adapter-deepseek-harness` for the installed DeepSeek Harness
JSON-RPC runtime. The route is `deepseek-harness.jsonrpc`; the driver ID is
`swallowtail.deepseek-harness.jsonrpc`. It owns one bounded structured run over
JSON-RPC 2.0 NDJSON on a host-approved child process.

This is a separate route from [DeepSeek Open Platform continuation](deepseek-prepared-integration.md).
It does not qualify DeepSeek's hosted SSE API, ACP, the headless CLI, or the
local Web `/api` surface.

The package is additive unreleased source after `v0.3.2`. Consumers must pin an
explicitly reviewed commit containing it. No version bump, tag, registry
publication, or harness installation is part of this route.

## Selected Boundary

Preparation requires all of the following:

- exact runtime axis `deepseek-harness.runtime-bin`
- exact `0.1.0rc6` packaged executable basename
  `dsh-jsonrpc-agent-pkg-macos-arm64`
- payload digest
  `ac1c91462518427467bd0a0ca3bf1049df62be0dbe8b0ee8014c6761cb8f80bf`
- host-approved `EnvironmentRef` for the Cordis configuration
- explicit provider, model, read-only working resource, and deadline
- host services for task, process, and time ownership

`serverInfo.version` (`0.0.1`) is wire metadata, not the compatibility axis.
Preparation stream-hashes the host-approved executable before version
qualification, and execution re-checks the same digest. The runtime claim is
qualified-only for `0.1.0rc6`; newer RC points do not inherit this route.

Call `prepare_deepseek_harness_jsonrpc` with
`DeepSeekHarnessPreparationInput` and `DeepSeekHarnessPreparationProbe`. The
probe runs only the bounded executable version check. It does not send a
prompt, acquire credentials, choose a model, or inspect the Cordis file.

The prepared integration binds the execution host, exact target, observation,
host configuration reference, access profile, and preflight evidence. Validate
that binding before reusing it. The access profile is local and unauthenticated:
Swallowtail receives no credential reference and opens no credential lease.

## Structured Run

Create `DeepSeekHarnessModelSelection` with an application-owned route ID and
revision plus explicit provider and model IDs. Create
`DeepSeekHarnessRunProfileInput` with a request ID, prompt, read-only
`WorkingResourceRef`, and deadline. Call `prepare_run`, inspect
`DeepSeekHarnessPreparedRun::evidence`, `plan`, and `request`, then call
`start_run` with host services for the same execution host.

The driver owns one joined process and performs this sequence:

1. send `initialize` with the host cwd, provider, and model
2. send `session/prompt` and treat its `{ messageId }` response as enqueue
3. fold `session.status` until the selected turn is idle
4. send `shutdown` and join process and task cleanup

The JSON-RPC stream is bounded. Unknown namespaced observations may remain
namespaced and correlated; raw envelopes do not become diagnostics. Assistant
text, content-free reasoning progress, harness-owned tool lifecycle, usage, and
terminal completion or failure remain separate projections. Tool argument and
result bodies, reasoning text, prompts, credentials, private paths, and raw
JSONL are not projected.

Cancellation force-stops the owned process. There is no advertised native
JSON-RPC cancel, interactive session continuity, provider-session management,
model catalogue, subagent control, consumer-tool callback, permission exchange,
or typed-question exchange.

## Host Configuration And Live Proof

The host owns the Cordis composition. A live smoke may use the qualified local
Ollama composition through `dsh-llm-pi-ai`; that proves the installed JSON-RPC
runtime and agent loop only. It does not qualify `deepseek-official` or change
the `deepseek.continuation` route.

The ignored live tests require explicit operator inputs:

```sh
export SWALLOWTAIL_DEEPSEEK_HARNESS_EXECUTABLE=/absolute/path/dsh-jsonrpc-agent-pkg-macos-arm64
export SWALLOWTAIL_DEEPSEEK_HARNESS_CORDIS=/absolute/path/to/cordis-config
export SWALLOWTAIL_DEEPSEEK_HARNESS_CWD=/absolute/path/to/read-only-workspace
export SWALLOWTAIL_DEEPSEEK_HARNESS_PROVIDER=local-ollama
export SWALLOWTAIL_DEEPSEEK_HARNESS_MODEL=operator-selected-model
```

The installed probe and configured prompt smoke are separate Effigy selectors.
Neither selector logs the Cordis path, provider credentials, prompt, tool
bodies, or reasoning text.

## Failures And Recovery

Wrong target, wrong version, missing host configuration, unavailable discovery,
malformed or oversized JSON-RPC, mismatched request IDs, provider RPC errors,
deadline, cancellation, process failure, and cleanup failure remain distinct
safe failure observations. A missing provider key or `PI_AI_ERROR` is a
provider/configuration failure, not evidence that the runtime or the
DeepSeek-official route is qualified.

There is no persistent session or provider recovery claim. A failed, cancelled,
timed-out, or mismatched run does not leave a reusable Swallowtail continuation
binding. The operation-owned process is joined or force-stopped and its host
configuration remains host-owned.

## Validation

Deterministic validation:

```sh
effigy validate:focused swallowtail-adapter-deepseek-harness
effigy package:verify-affected swallowtail-adapter-deepseek-harness
effigy qa:guides
effigy qa:routes
```

Optional operator-gated probes:

```sh
SWALLOWTAIL_LIVE_DEEPSEEK_HARNESS=1 \
  effigy probe:deepseek-harness-installed

SWALLOWTAIL_LIVE_DEEPSEEK_HARNESS_PROMPT=1 \
  effigy probe:deepseek-harness-live
```

The normal public shape is shown in
[`prepared_deepseek_harness_jsonrpc`](../../crates/swallowtail-adapter-deepseek-harness/examples/prepared_deepseek_harness_jsonrpc.rs).
