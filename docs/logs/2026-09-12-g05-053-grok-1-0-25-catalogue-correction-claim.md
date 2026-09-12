# 2026-09-12 g05.053 Grok 1.0.25 Catalogue Correction Claim

## Result

Exact installed Grok Build `1.0.25` (`f7e67d6988e2`, SHA-256
`9ef4a40ad60c6a5178a65caf39c2a148e6a98d0d2d350b10329dee34d9195d9c`) now has a
separately prepared `grok-build.catalogue` model-catalogue route. It runs
exactly `--no-auto-update models` under `HarnessConfigurationPosture::Ambient`
as an authenticated, non-inference metadata operation.

This corrects the g05.052 non-admission and the later suppression attempts.
The route may refresh authentication or bounded catalogue metadata, but it
sends no prompt, opens no model session, invokes no inference or tool, updates
no harness, retries nothing, and retains no provider state beyond the bounded
operation. Grok's auth-refresh watcher and bounded catalogue metadata traffic
are permitted; absence of that traffic is not an acceptance condition.
`ProviderSuppressed` was removed because no consumer or route contract required
provider-configuration suppression, and it was not replaced with an
operating-system network sandbox.

## Claim Boundary

`grok_build_acp_claim`, `grok_build_model_for_version`, released `v0.5.0`, and
every tagged or historical ledger and release note are unchanged. Route 50
(`grok-build.catalogue`) is additive post-`v0.5.0`: current source and the
working `0.5.0` baselines validate at 50 production routes while tagged
`v0.5.0` and Research 281 stay frozen at 49, and the route-matrix and
front-door gates fail on collapse.

## Evidence

The shipped exact-`1.0.25` output grammar was recovered statically from the
installed binary: an authentication preamble followed by
`  * <id> (default)` and `  - <id>` bullet rows. The parser requires that
grammar and tolerates the preamble; a bare two-space row fails closed.

The accepted final observation is recorded in
`crates/swallowtail-adapter-grok/tests/fixtures/grok-1.0.25-model-catalogue/live-capsule.json`:
exact argv `["--no-auto-update", "models"]`, `Ambient` posture, stdout 113
bytes with zero stderr, bullet-grammar parse to ordered `grok-4.6` default then
`grok-4.5`, no prompt, no model session, no inference, and joined cleanup. Its
`runtime_proof` fields are descriptive metadata-path observations, not
acceptance gates. Both earlier observations are preserved as evidence and were
not retried. Research 306 freezes the boundary and the capsule.

## Claim Surface

`GrokCatalogueProfileInput`, `GrokPreparedCatalogue` (`list_models`,
`list_models_recorded`), `GrokCatalogueDriver`, `GrokCatalogueListing`, and
`GrokCatalogueOutputEvidence` are the public surface. `GrokCatalogueOutputEvidence`
retains only stdout/stderr byte counts and SHA-256 digests; raw provider output
and raw account data stay host-private and never enter records or diagnostics.

## Validation

`cargo fmt -p swallowtail-adapter-grok -- --check`,
`effigy validate:focused swallowtail-adapter-grok`,
`effigy package:verify-affected swallowtail-adapter-grok`,
`effigy check:examples`, `effigy package:api`, `effigy qa:routes`,
`effigy qa:consumer-docs`, `effigy qa:guides`, `effigy qa:northstar`,
`effigy qa:docs`, the testkit activity corpora, and `git diff --check`.

## Boundary

This corrects the catalogue seam only. It does not widen Grok ACP execution,
change `grok_build_model_for_version`, infer entitlement from a listing,
hard-code models downstream, create a generic router, install or update Grok,
edit host, fleet, requirements, MDM, or user configuration, create a release
or tag, publish, or mutate a consumer. Credentials and raw host configuration
stay private. A listing is not a model prompt and grants no inference
authority. Desktop may adopt only the exact accepted source SHA under separate
consumer authority, or wait for a separately authorized future source tag.
