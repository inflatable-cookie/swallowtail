# Ollama Native Attached Driver

Date: 2026-07-23
Card: 108

## Changed

- Added `OllamaNativeAttachedDriver` with separately registered model-catalogue
  and structured-run roles.
- Added strict loopback HTTP and native NDJSON transport with proxy, redirect,
  credential, retry, and endpoint fallback disabled.
- Added host-owned catalogue observation time beside monotonic deadline time.
- Added attached observations to mutable model catalogue metadata without
  creating a route or provider identity.
- Added exact runtime, installed, running, selected-detail, tag, and digest
  revalidation before one text-only `/api/chat` attempt.
- Added deterministic production fixtures for success, version drift,
  provider failure, cancellation, deadline, redaction, joined close, and both
  execution-host identities.

## Boundary

- The runtime and its resident models remain externally managed.
- Close joins Swallowtail-owned task and network work. It does not stop Ollama,
  unload a model, restore residency, or claim remote compute stopped.
- Provider lifecycle timestamps do not become observation time.
- No credential, process, artifact, model mutation, cloud, container, retry,
  compatible facade, or Monkey authority was added.
- Independent runs remain allowed. Swallowtail does not serialize them without
  an explicit provider or contract requirement.

## Evidence

- Ollama tests: 20 pass; one installed-runtime probe is ignored by default.
- Core tests: 38 pass.
- Local-host tests: 21 pass.
- Focused warnings-denied clippy passes for core, runtime, local host, and
  Ollama.

## Next

Card 109 adds portability and failure conformance, runs full repository QA, and
closes roadmap 038.
