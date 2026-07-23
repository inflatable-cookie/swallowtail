# 108 Ollama Native Attached Driver

Status: completed
Owner: Tom
Updated: 2026-07-23
Milestone: `../038-ollama-native-attached-runtime-proof.md`

## Objective

Implement one production attach-only Ollama native driver against
host-approved endpoint services.

## Scope

- separately registered model-catalogue and structured-run roles
- exact version observation before immutable plan use
- local unauthenticated access profile and host-approved loopback endpoint
- installed, running, and selected-model detail observation
- one exact selected model tag, digest, output bound, and residency posture
- one text-only streaming native chat attempt
- ordered output and usage, distinct provider/transport/protocol failures
- cancellation, deadline, joined connection cleanup
- no credential, process, artifact, lifecycle, model mutation, unload, retry,
  fallback, compatible facade, or live default test

## Acceptance Criteria

- [x] effects begin only after exact version and plan agreement
- [x] all catalogue observations retain source scope
- [x] one run makes one inference attempt
- [x] close joins owned work without stopping the runtime or changing residency
- [x] endpoint and raw provider data stay out of diagnostics
- [x] unsupported capability and version drift fail before inference

## Validation

- focused production-driver tests
- focused warnings-denied clippy
- `effigy doctor` delta review
- `git diff --check`

## Auto-Continuation

Yes. Continue to card 109 when the production driver passes focused validation.

## Evidence

- `OllamaNativeAttachedDriver` registers separate catalogue and structured-run
  roles over host-approved loopback HTTP and native NDJSON.
- The host time boundary now exposes a distinct catalogue observation clock;
  provider timestamps and monotonic deadlines are not substituted.
- Catalogue results retain installed, running, and selected-detail observations
  without creating a provider identity or route.
- Run startup revalidates exact runtime version, model tag, and manifest digest
  before one `/api/chat` attempt.
- Twenty focused Ollama tests pass. One installed-runtime probe remains ignored
  and separately gated.
- Focused core, runtime, local-host, and Ollama warnings-denied clippy passes.
