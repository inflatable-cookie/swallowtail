# 2026-07-27 Pi RPC Model Catalogue

## Changed

- Revalidated exact Pi `0.80.10` and current `0.82.1`
  `get_available_models` evidence.
- Added Pi's `ModelCatalogDriver` role and typed
  `PiPreparedIntegration::prepare_catalogue` facade.
- Added one route-free, workspace-free, tool-free, provider-suppressed
  ephemeral RPC operation using the prepared executable and delegated harness
  access.
- Added bounded provider/model identity, display name, input modality,
  reasoning-support, context-window, and output-limit projection. Raw endpoint,
  API, cost, and provider payload fields remain private.
- Corrected the solution feature matrix from `No` to `Yes` for Pi catalogue
  coverage.
- Corrected Contract 020: `ModelCatalogRequest` has a deadline but no
  independent cancellation control. Future drop does not claim joined
  cancellation.

## Evidence

- Exact request/response, malformed, rejected, overflow, correlation drift,
  disconnect, deadline, and cleanup-failure fixtures.
- Local and remote-authoritative prepared catalogue execution.
- Existing Pi discovery, session, scheduling, callback, cancellation,
  deadline, failure, redaction, version, and cleanup regression coverage.
- `cargo test -p swallowtail-core -p swallowtail-adapter-pi`: 75 passed.
- `effigy check:rust`, `effigy format:check`, `effigy qa:docs`,
  `effigy qa:routes`, and `git diff --check`: passed.
- `effigy package:api`: expected hash drift. It includes the intentional new
  Pi/core API and unrelated already-present adapter changes. The retained
  release baseline was not rewritten from the dirty tree.

## Current State

Card 067 is complete. Pi reports the configured, auth-aware model catalogue;
Swallowtail does not maintain a duplicate model list or select a model from the
result.

## Next

Card 068 is ready: add Kimi local-server catalogue coverage, then expose only
already-negotiated Gemini and Kimi ACP model options without hidden session
creation.
