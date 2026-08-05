# Ollama Attached Prepared Integration

Use the prepared facade to attach to an operator-managed Ollama native
runtime. Swallowtail observes and invokes the selected deployment. It does not
install Ollama, acquire models, own the server, or administer residency.

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

One `start_run` call is one native `/api/chat` attempt. Invocation may load the
selected model, refresh its timer, or evict another model. That accepted side
effect grants no unload, restoration, duration, exclusive-capacity, process,
or serving authority.

Cancellation and deadline stop and join Swallowtail-owned network work without
claiming remote compute cancellation. Closing the handle does not stop the
server or unload a model. Retry, endpoint fallback, model substitution,
compatible-facade fallback, tools, thinking, vision, and model administration
remain absent.

The prepared interactive profile exposes `prepare_working_state_restoration`.
It opens an empty replacement session against the same selected attached
runtime and returns the interrupted consumer turn id. The lost private
transcript is not serialized or replayed, and the external runtime remains
operator-owned.

`plan`, `request`, `evidence`, `runtime`, `low_level_driver`, and `into_parts`
remain available for diagnostics and advanced low-level use.

See the compile-tested
[`prepared_attached` example](../../crates/swallowtail-adapter-ollama/examples/prepared_attached.rs).
