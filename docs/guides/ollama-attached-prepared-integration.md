# Ollama Attached Prepared Integration

Use the prepared facade to attach to an operator-managed Ollama native
runtime. Swallowtail observes and invokes the selected deployment. It does not
install Ollama, acquire models, own the server, or administer residency.
New to the shared vocabulary? Read [Key Concepts](key-concepts.md).

The route is `ollama.attached` in `swallowtail-adapter-ollama`, driver ID
`swallowtail.ollama.native-attached`, over native HTTP/NDJSON. Choose it for an
operator-owned Ollama runtime when the application needs inventory, one
structured attempt, or bounded consumer-owned transcript replay. Reject it
when the application must start/stop the runtime, pull/unload models, inject
tools, or obtain durable provider-session identity.

## Inputs That Stay Explicit

Preparation requires:

- one configured-instance identity and revision
- one execution host and host-approved endpoint target
- one local-unauthenticated access profile and evidence
- one consumer-selected Swallowtail model route
- one operator-selected native model tag and expected manifest digest
- one probe scope, cancellation control, and host-monotonic deadline

The model route, native tag, and manifest digest remain separate identities.
Supplying all three does not claim that Swallowtail owns or can locate the
underlying model artifact.

The local access profile requires no credential. The host supplies the
approved endpoint plus network, task, time, and cancellation services required
by discovery and operations. No endpoint value enters stable evidence.
Resource-free inference grants no filesystem or process authority and makes no
sandbox claim.

## Prepare The Attached Runtime

Call `prepare_ollama_attached` with `OllamaPreparationInput`,
`OllamaPreparationProbe`, and the selected host services. The probe uses only
the host-approved native endpoint. It observes:

- exact `/api/version`
- bounded installed inventory from `/api/tags`
- bounded running inventory from `/api/ps`
- bounded selected-model detail from `/api/show`

Preparation performs no inference, credential acquisition, model mutation,
pull, unload, or server lifecycle action. The result retains the exact runtime
version, qualified or unverified-newer assessment, installed and running
observations, selected detail, route selection, configured instance, access
provenance, and low-level driver escape hatch.

The guaranteed window remains `0.14.0` through `0.32.1`. Exact `0.32.2` stays
excluded and semantic prereleases fail. A later exact stable version may
proceed as visibly unverified through the latest qualified text behavior. It
does not expand guaranteed support, and every operation rechecks exact runtime
version and protocol behavior.

## Observe Inventory

`prepare_inventory` accepts a request identity and optional deadline. The
selected route is already explicit in the prepared integration; catalogue
output cannot replace it.

`observe_inventory` returns an `OllamaInventorySnapshot` with separate
`installed`, `running`, and `selected_detail` views over source-scoped
observations. Installed does not mean resident. Running does not mean owned.
Selected detail does not prove hardware fit, invocation readiness, or artifact
authority.

The native low-level driver validates version, installed inventory, running
inventory, and selected detail in one bounded catalogue operation. The
prepared views preserve those scopes rather than inventing new runtime roles.

## One Inference Attempt

`prepare_inference_attempt` requires request identity, content, one positive
maximum-output-token bound, and an optional deadline. It derives the prepared
route and an offline text request with explicit
`AttachedRuntimeResidency::RuntimeManaged`.

Optional controls are exact reasoning `off`, `low`, `medium`, or `high` when
the selected-model detail advertises thinking, and one inline JSON Schema
2020-12 object with provider-native enforcement. Preparation rejects controls
not supported by the observed selected model; it never infers capability from
the model tag alone.

One `start_run` call is one native `/api/chat` attempt. Invocation may load the
selected model, refresh its timer, or evict another model. That accepted side
effect grants no unload, restoration, duration, exclusive-capacity, process,
or serving authority.

Cancellation and deadline stop and join Swallowtail-owned network work without
claiming remote compute cancellation. Closing the handle does not stop the
server or unload a model. Retry, endpoint fallback, model substitution,
compatible-facade fallback, tools, attachments, vision, and model administration
remain absent.

Take and drain run events and terminal concurrently, then close the run.
Assistant output, reasoning, usage, provider failure, cancellation, remote
compute uncertainty, and local cleanup remain distinct. No result or error
authorizes retry.

## Interactive Transcript Replay

`prepare_session` accepts `OllamaSessionProfileInput` with request identity and
optional deadline. `open_session` creates no provider session. Swallowtail
keeps at most 24 clean terminal turns and one MiB of private transcript in the
live handle, replaying that consumer-owned history on later `/api/chat`
requests. Failed, cancelled, timed-out, or malformed turns do not commit.

For every turn, drain events and terminal concurrently, then close it.
Active-turn interruption stops local network work without claiming remote
compute cancellation. Closing the session destroys the private transcript and
leaves the external runtime and model residency untouched.

The prepared interactive profile exposes `prepare_working_state_restoration`.
It opens an empty replacement session against the same selected attached
runtime and returns the interrupted consumer turn id. The lost private
transcript is not serialized or replayed, and the external runtime remains
operator-owned.

`plan`, `request`, `evidence`, `runtime`, `low_level_driver`, and `into_parts`
remain available for diagnostics and advanced low-level use.

See the compile-tested
[`prepared_attached` example](../../crates/swallowtail-adapter-ollama/examples/prepared_attached.rs).

## Failures, Unsupported Capabilities, And Promotion

Handle failures through portable classification and retain the exact
`swallowtail.ollama.*` diagnostic for support. Never parse NDJSON/HTTP payloads,
runtime logs, model files, endpoint values, or provider prose in consumer code.

The route exposes no attachments, tools, callbacks, working resource,
external search, provider retention authority, public load/resume,
reconciliation, archive/restore/delete, native close, owned runtime lifecycle,
background execution, reattachment, billed cost, or retry.

Promotion requires an exact Ollama release and selected-model manifest
evidence, immutable route/residency binding, bounded native fixtures,
lifecycle tests, and route-matrix coverage. An installed or resident tag alone
is insufficient.

## Deterministic Validation And Optional Probe

```sh
effigy validate:focused swallowtail-adapter-ollama
effigy check:examples
```

The separately gated `effigy probe:ollama-installed` may inspect an
operator-selected local runtime. It is not required for deterministic
acceptance and must not pull, invoke, or unload a model.
