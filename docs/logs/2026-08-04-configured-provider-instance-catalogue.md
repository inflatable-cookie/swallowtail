# 2026-08-04 Configured Provider Instance Catalogue

## Outcome

Nucleus g05.073 exposed a portable boundary gap. Swallowtail had exact
configured instances, prepared routes, access evidence, and model catalogues,
but no admitted projection binding them for consumer selection.

Contract 047 and g03.024 now define and realize that projection in
`swallowtail-runtime`.

The runtime surface adds:

- `ConfiguredProviderInstanceAdmission`
- `ConfiguredProviderInstanceRecord`
- `ConfiguredProviderInstanceCatalogue`
- `ConfiguredProviderCredentialPosture`
- `ConfiguredProviderInstanceRoute`
- `ConfiguredProviderModelCatalogueInput`
- `ConfiguredProviderModelCatalogue`
- `ConfiguredProviderInstanceSelectionReadiness`
- `ConfiguredProviderInstanceCatalogueFailure`

Admission verifies exact driver, transport, configured instance, revision,
host, target, facade, policy, access profile, access status, and prepared-route
agreement. The published record drops the target and credential reference. It
retains the full safe driver descriptor, configured capabilities, interface
versions, provider-agent and harness posture, exact route evidence, model and
provider ids, model metadata, and safe access provenance.

Selection readiness is strict. Only positive credential, entitlement,
endpoint, runtime, support, and non-empty model-catalogue evidence produces
`Ready`. Unknown, degraded, unauthenticated, failed, unsupported, absent, and
empty states remain in the catalogue as `NotReady`.

The catalogue is bounded to 256 instances, 64 routes per instance, and 10,000
models per instance. Duplicate instance, route, and provider/model identities
fail safely.

## Nucleus Assembly Path

For the existing Codex app-server route, Nucleus can:

1. call `prepare_codex` as it does now
2. call `CodexPreparedIntegration::prepare_catalogue`
3. clone `CodexPreparedCatalogue::evidence().operation()` as the exact model-
   catalogue source
4. call `CodexPreparedCatalogue::list_models`
5. wrap success with `ConfiguredProviderModelCatalogueInput::available`, or
   retain a safe failure with `ConfiguredProviderModelCatalogueInput::unavailable`
6. construct `ConfiguredProviderInstanceAdmission` from
   `codex_app_server_descriptor()`, `prepared.instance()`,
   `prepared.access_profile()`, and `prepared.access_evidence()`
7. add the source through `with_prepared_routes`, add the model outcome through
   `with_model_catalogue`, then call `ConfiguredProviderInstanceRecord::admit`
8. assemble all admitted records with `ConfiguredProviderInstanceCatalogue::new`

Nucleus may project this immutable snapshot into its API and UI. Instance,
model, and reasoning choice plus session recreation remain Nucleus policy. It
must retain the selected Swallowtail instance, facade, provider, model, and
route identities rather than reconstructing them from labels.

An unavailable model result still produces a visible non-ready record. An
instance with no bound model result also remains visible and non-ready. No
provider router, default, fallback, refresh loop, persistence, or generic
operation entered Swallowtail.

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime` — 176 tests
  passed
- `effigy package:verify-affected swallowtail-core swallowtail-runtime` — both
  extracted packages compiled
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`
- no authenticated provider work or live provider operation

## Current State

Cards 064-065 and roadmap g03.024 are complete. Nucleus may update its local
path dependency and resume g05.073. Swallowtail returns to the g03 evidence
gate.
