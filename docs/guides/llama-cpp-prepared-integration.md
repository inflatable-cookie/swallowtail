# llama.cpp Prepared Integration

`swallowtail-adapter-llama-cpp` exposes two deliberately separate routes:
New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

| Route | Driver ID and transport | Choose it for | Reject it when |
| --- | --- | --- | --- |
| `llama-cpp.attached` | `swallowtail.llama-cpp.attached-openai-chat`; OpenAI-compatible HTTP/SSE | catalogue and one inference attempt against an operator-owned server | the application must own process startup, artifact leasing, or server shutdown |
| `llama-cpp.owned` | `swallowtail.llama-cpp.owned-b10069-openai-chat`; owned process plus HTTP readiness | starting and stopping one loopback `llama-server` from an exact approved GGUF binding | the application expects the same facade to perform inference or persist serving |

Neither route uses credentials or billing. Local unauthenticated access is not
permission to discover endpoints, executables, artifacts, or models.

## Attached Runtime

`prepare_llama_cpp_attached` requires configured-instance and authoritative
execution-host identity, one host-approved endpoint target, local attached
access profile and evidence. The host supplies network, task, and time services
for health, catalogue, streaming, deadline, and cleanup work. Preparation makes
no inference and never starts or configures the external server.

The route binds exact opaque b9910/f5525f7e7 behavior on
`llama.cpp.attached-runtime`; another build does not receive unverified-newer
admission.

The prepared integration exposes two typed operations:

- `prepare_catalogue` then `list_models` checks `/health`, `/props`, the exact
  runtime identity, and the selected single-model source without selecting a
  new route
- `prepare_inference_attempt` accepts exact `LlamaCppModelSelection`, text,
  positive maximum-output-token bound, and optional deadline; `start_run`
  performs one HTTP/SSE attempt

Take and drain run events and terminal concurrently, then close the run.
Output, usage, provider failure, cancellation, deadline, network cleanup, and
external server state remain separate. Cancellation closes local request work
without claiming remote compute interruption. No result or error authorizes
retry.

Closing catalogue or inference work preserves the external server. Catalogue
presence proves neither hardware fit, model ownership, context availability,
nor successful invocation.

See the compile-tested
[`prepared_llama_cpp_attached`](../../crates/swallowtail-adapter-llama-cpp/examples/prepared_llama_cpp_attached.rs).

## Owned Ephemeral Serving

Use `prepare_llama_cpp_owned` when the execution host may start one approved
`llama-server` executable from one exact `ModelArtifactBinding` and model
route/alias. `LlamaCppOwnedPreparationInput` also binds configured-instance,
host, local access evidence, and executable target. The host supplies process,
artifact, endpoint-authority, network, task, and time services required by the
lifecycle plan.

The route binds exact opaque b10069/178a6c449 behavior on
`llama.cpp.owned-runtime`; it has no unverified-newer posture.

Call `prepare_serving_start`, inspect the lifecycle plan and
`StartServingRequest`, then `start`. The operation acquires the artifact,
launches one offline loopback-only process, observes its bounded stderr
endpoint, publishes host endpoint authority, and verifies health, properties,
and catalogue identity. Only then does it return `OwnedServingHandle`.

Call `stop` exactly once during shutdown. It stops and joins the child,
invalidates endpoint authority, then releases the artifact lease. Failure or
cancellation during startup runs the same bounded cleanup. The handle owns no
model file, provider state, persistent server, or inference transcript.

Inference and catalogue use the returned endpoint only through a separately
prepared attached route. Owned serving itself has no structured-run or model-
catalogue role.

See the compile-tested
[`prepared_llama_cpp_owned`](../../crates/swallowtail-adapter-llama-cpp/examples/prepared_llama_cpp_owned.rs).

## Failures, Unsupported Capabilities, And Promotion

Handle failures through portable classification and retain the exact
`swallowtail.llama_cpp.*` diagnostic for support. Keep readiness, inference
terminal, process stop, endpoint invalidation, artifact release, and cleanup
truth separate. Never parse raw HTTP/SSE payloads, model bytes, executable
paths, stderr beyond the qualified bounded endpoint observation, or endpoint
values in consumer code.

Neither route downloads or mutates models, searches `PATH`, selects a default,
uses working resources, tools, callbacks, attachments, reasoning, structured
output, external search, sessions, reconciliation, management, retry, or
fallback. Attached has no runtime lifecycle; owned has no inference role.

Promotion requires exact build/artifact/protocol evidence, immutable topology
and authority binding, bounded fixtures, startup/readiness/stop or inference
lifecycle tests, and route-matrix coverage.

## Deterministic Validation

```sh
effigy validate:focused swallowtail-adapter-llama-cpp
effigy check:examples
```

No external server, model execution, artifact download, or process launch is
required.
