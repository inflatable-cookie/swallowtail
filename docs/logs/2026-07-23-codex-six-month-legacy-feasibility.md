# 2026-07-23 Codex Six-Month Legacy Feasibility

## Changed

- Corrected the assumption that January Codex app-server is v1-only.
- Confirmed the selected v2 methods in exact `0.80.0` tagged source.
- Identified default stdio through `0.99.0` and explicit listener invocation
  from `0.100.0`.
- Split legacy exec behavior around search configuration, ephemeral state, and
  config or rules suppression.
- Kept unpublished stable-version gaps as exact exclusions.
- Selected explicit deprecated segments instead of a container or copied
  credential home.
- Compiled cards 116-119 for contract, corpus, production dispatch, and
  six-month conformance work.

## Recommendation

Keep the existing driver identities and dispatch privately by exact version.
Legacy app-server exposes only its proven stable read-only subset. Legacy exec
uses the user's existing Codex home and login, but its ambient configuration
and pre-`0.99.0` durable retention must be explicit. Current suppressed exec
behavior remains separate.

This keeps use seamless without hiding risk. Swallowtail does not select a
legacy route, fall back to it, or decide whether a consuming application warns,
blocks, or prefers an upgrade.

## Evidence

- official npm history places `0.80.0` on 2026-01-09 and `0.145.0` on
  2026-07-21
- `0.80.0` source contains the selected app-server v2 method and notification
  map
- generated v2 schemas enter tagged source at `0.94.0`
- app-server listener selection enters at `0.100.0`
- exec ephemeral mode enters at `0.99.0`
- exec config and rules suppression enter at `0.122.0`
- `0.82.0`, `0.83.0`, `0.108.0`, and `0.109.0` have source tags but no stable
  npm publication and remain excluded

## Next

Card 116 promotes the shared harness-configuration posture before any legacy
provider code changes.
