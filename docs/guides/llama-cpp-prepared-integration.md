# llama.cpp Prepared Integration

Swallowtail exposes two llama.cpp preparation paths. They are deliberately
different types.

## Attached Runtime

Use `prepare_llama_cpp_attached` when a host-approved llama.cpp server already
exists.

Preparation requires:

- one configured-instance identity and revision
- the authoritative execution host
- one host-approved endpoint target
- `llama_cpp_attached_access_profile()` and matching access evidence

The prepared integration exposes `prepare_catalogue` and
`prepare_inference_attempt`. Each operation checks `/health`, `/props`, the
exact b9910/f5525f7e7 runtime identity, and the selected single-model route
before provider effects continue.

The attached path has no serving-lifecycle method. Closing a run releases only
the operation-owned stream and tasks. It cannot stop the external server.

See
[`prepared_llama_cpp_attached.rs`](../../crates/swallowtail-adapter-llama-cpp/examples/prepared_llama_cpp_attached.rs).

## Owned Ephemeral Serving

Use `prepare_llama_cpp_owned` when the execution host may start one approved
`llama-server` executable from one already-approved GGUF artifact binding.

Preparation requires:

- one configured-instance identity and revision
- the authoritative execution host
- one approved executable target
- `llama_cpp_owned_access_profile()` and matching access evidence
- one exact `ModelArtifactBinding`
- one exact model route and alias

`prepare_serving_start` derives the lifecycle preflight plan and repeats the
selected artifact in `StartServingRequest`. `start` acquires the artifact,
launches an offline loopback-only process, observes its stderr endpoint,
publishes host endpoint authority, and checks health, b10069/178a6c449
properties, and catalogue identity. It returns an `OwnedServingHandle` only
after readiness succeeds.

Stopping the handle joins the child, invalidates endpoint authority, then
releases the artifact lease. There is no attached-server constructor on this
path.

See
[`prepared_llama_cpp_owned.rs`](../../crates/swallowtail-adapter-llama-cpp/examples/prepared_llama_cpp_owned.rs).

## Deliberate Omissions

Neither path discovers or downloads models, searches for an executable,
selects a default route, creates a persistent server, or takes over Monkey's
serving responsibilities. Consumers and execution hosts retain those choices
and authorities.
