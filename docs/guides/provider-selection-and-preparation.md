# Provider Selection And Preparation

Use this flow to present configured provider choices without inventing a
provider registry or rebuilding adapter evidence in the application.

## Assemble The Catalogue

The consumer owns the configured-instance list. For each configured route:

1. Prepare the exact adapter integration using its route guide.
2. Keep the `DriverDescriptor`, `ConfiguredInstance`, `AccessProfile`,
   `PreparedAccessEvidence`, and every successfully prepared operation's
   `PreparedOperationEvidence` together.
3. If the route has a model catalogue, run its separately prepared catalogue
   operation. Preserve an unavailable result as
   `ConfiguredProviderModelCatalogueInput::unavailable`; do not omit the
   configured instance.
4. Build `ConfiguredProviderInstanceAdmission::new`, add the exact operation
   evidence with `with_prepared_routes`, and add the model result with
   `with_model_catalogue` where applicable.
5. Call `ConfiguredProviderInstanceRecord::admit`, then assemble the admitted
   records with `ConfiguredProviderInstanceCatalogue::new`.

Admission verifies that driver, instance, access, route, and model-catalogue
source evidence describe the same configured instance. The catalogue is
bounded to 256 instances, 64 prepared routes per instance, and 10,000 models
per instance. Duplicate instance, route, or model identities fail closed.

`ConfiguredProviderInstanceSelectionReadiness::Ready` is derived only when
the admitted credential posture permits selection and the bound model
catalogue is available and non-empty. `NotReady` remains a visible record.
It is not permission to hide, retry, authenticate, probe, or replace it.

## Render And Select

Render stable identity and safe posture from `ConfiguredProviderInstanceRecord`:

- `instance_id` and `instance_revision`
- `driver_identity`, integration family, transport family, and protocol facade
- execution host and ownership
- exact interface-version bindings
- capability profile
- safe credential, entitlement, endpoint, runtime, support, and provenance
  posture through `credential_posture`
- admitted operation routes through `routes`
- the correctly bound `model_catalogue`
- `selection_readiness`

The catalogue contains no credential value or raw target. Keep those in the
already prepared adapter integration and host services.

After the operator selects an instance, select one exact
`ConfiguredProviderInstanceRoute` and, where required, one exact
`ConfiguredProviderModelRoute`. Match route identity and operation shape;
do not choose by provider name, capability similarity, model label, or another
transport from the same provider. Then use the original matching prepared
adapter value to prepare and execute the operation.

The catalogue does not return a generic executable facade. It is evidence for
a consumer-owned selection decision. The route-specific prepared value remains
the only normal execution path.

## Model Catalogue Truth

Model discovery and invocation remain separate:

- standalone catalogue routes may be usable before a session exists
- negotiated session model options are available only from the opened or
  attached `InteractiveSessionHandle`
- an available model entry does not prove entitlement, route compatibility,
  reasoning support, schema support, or request acceptance
- an unavailable catalogue is not an empty successful catalogue
- a route without catalogue support requires an explicit route-supported
  model selection; the consumer must not manufacture catalogue evidence

Use the [provider route matrix](provider-route-matrix.md) for route identity
and the [feature matrix](provider-solution-feature-matrix.csv) for model
catalogue availability. Every route guide names its exact model-selection and
preparation sequence.

## Preparation Boundary

Prepared integrations bind configured-instance revision, execution host,
opaque target, access profile and provenance, protocol facade, and exact
version evidence. Bound operations add consumer intent and expose immutable
`PreparedOperationEvidence`, plan, request, low-level driver, and an
ownership-preserving `into_parts` path where the route supports them.

Preparation may perform the bounded observation named by the route. It does
not log in, select a provider or model, search for executables, start an
operation, apply fallback, or create consumer persistence.

Preparation failures retain `PreparationStage` and a safe diagnostic. Use the
[portable failure guide](portable-failure-handling.md); never parse a probe's
stdout, stderr, target, or provider prose in the consumer.

## Ownership And Refresh

The consumer owns display names, ordering, defaults, refresh timing,
persistence, and the mapping from a selected catalogue record back to its
prepared adapter value. Refresh by preparing and admitting a new immutable
catalogue. Do not mutate readiness or splice newer model entries into an old
record.

Swallowtail owns bounded admission and exact evidence consistency. It does not
own a generic router, default provider, automatic fallback, credential flow,
or provider-selection persistence.

## Examples And Validation

Use the normal-path example linked from the selected row in the
[integration guide map](integration-guide-map.md). Those examples expose the
adapter-specific preparation values from which catalogue admissions are
assembled.

Deterministic acceptance:

```sh
effigy check:examples
effigy qa:docs
effigy qa:routes
```

Live catalogue calls and authentication checks remain separately
operator-gated.
