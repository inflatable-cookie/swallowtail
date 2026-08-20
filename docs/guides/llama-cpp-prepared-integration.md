# llama.cpp Prepared Integration

`swallowtail-adapter-llama-cpp` exposes two deliberately separate routes:
New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

| Route | Driver ID and transport | Choose it for | Reject it when |
| --- | --- | --- | --- |
| `llama-cpp.attached` | `swallowtail.llama-cpp.attached-openai-chat`; OpenAI-compatible HTTP/SSE | catalogue and one inference attempt against an operator-owned server | the application must own process startup, artifact leasing, or server shutdown |
| `llama-cpp.owned` | `swallowtail.llama-cpp.owned-b10069-openai-chat`; owned process plus HTTP readiness | starting and stopping one loopback `llama-server` from an exact approved GGUF binding | the application expects the same facade to perform inference or persist serving |

Neither route uses credentials or billing. Local unauthenticated access is not
permission to discover endpoints, executables, artifacts, or models.

## Add The Attached Connection

Only `llama-cpp.attached` currently exports an addable descriptor.
`llama-cpp.owned` stays on the prepared facade path below. Topology is
**local-runtime**. It is not `ExecutionLayer`. Follow
[connection lifecycle](connection-lifecycle.md) before
`prepare_llama_cpp_attached`. Swallowtail does not start or stop the
operator-owned server.

1. Assemble `AddableRouteCatalog` from
   `llama_cpp_attached_addable_route_descriptor`. The row is `Available`
   when the host exposes the Network service; otherwise
   `Unavailable(HostService)`. Runtime reachability stays preparation, not
   the addable row. The addable row does not probe `/health`.
2. `admit_instance` writes the configured instance with an opaque
   `endpoint` `ApiEndpoint` `ConfigFieldRef`. Admission does not prepare.
3. There is no credential field and no sign-in loop. Reuse
   `llama_cpp_attached_access_profile`. Do not use
   `llama_cpp_owned_access_profile` on this row.
4. `refresh_readiness` writes host-supplied `AccessStatus`. Enablement stays
   independent of 047 `Ready` / `NotReady`. There is no credential
   dimension.
5. `observe_authenticated_subject` is `Absent`.
6. `observe_instance_update` reuses `llama_cpp_attached_runtime_claim`.
   Contract 032 stays unobserved unless an executable is supplied. Exact
   opaque b9910/f5525f7e7 binding stays prepare-time. No unverified-newer.
7. llama.cpp catalogue rows omit `provider_id`. Overlay cannot mark them.
   Do not invent a catalogue provider id.
8. Build `LlamaCppAttachedPreparationInput::from_admitted` from the admitted
   record, then call `prepare_llama_cpp_attached`. The constructor selects the
   stored `endpoint` ref; the host resolves it for preparation. Exact opaque
   b9910/f5525f7e7 binding stays prepare-time.

The compile-tested
[`connection_lifecycle` example](../../crates/swallowtail-adapter-llama-cpp/examples/connection_lifecycle.rs)
shows catalog through prepare. The canonical route-map example remains
[`prepared_llama_cpp_attached`](../../crates/swallowtail-adapter-llama-cpp/examples/prepared_llama_cpp_attached.rs).

## Attached Runtime

`LlamaCppAttachedPreparationInput::from_admitted` selects the admitted route's
opaque endpoint ref. Preparation then requires authoritative execution-host
identity, local attached access profile and evidence. The host resolves the
endpoint and supplies network, task, and time services for health, catalogue,
streaming, deadline, and cleanup work. Preparation makes no inference and
never starts or configures the external server.

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
