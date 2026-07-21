# 2026-07-20 Owned llama.cpp Contract And Runtime Records

## Decision

Advance host-owned ephemeral llama.cpp and defer Kimi production. A later
deployment-owned containment lane is authorized, but its first platform and
mechanism remain unselected.

The first owned proof pins current llama.cpp `b10069` as a separate driver
lifecycle from the existing attached `b9910` proof. It uses one pre-existing
operator-supplied GGUF, loopback port zero, offline mode, an explicit alias,
disabled UI, and disabled agent tools. It owns no download, artifact mutation,
persistent server, or Monkey behavior.

## Evidence And Contract

Research 008 records the current release, executable archive digest, tagged
launch surface, health/catalogue observations, and tagged source evidence that
port zero binds an available port and reports the selected address.

Contract 018 promotes:

- safe artifact identity plus an opaque host reference
- read-only serving-scope artifact leases distinct from attachments
- readiness-before-handle and a redacted host-scoped endpoint handoff
- child stop and join before endpoint invalidation and artifact release
- explicit exclusions for router, tool, UI, public-listener, download, and
  persistent-serving behavior

Roadmap 019 and cards 060-064 compile the evidence, provider-neutral runtime,
local host, production driver, and closeout batches inside g01.

## Runtime Records

Card 061 realizes the provider-neutral boundary:

- artifact id, format, revision, digest, optional quantization, and opaque
  reference remain separate
- preflight can require and bind one exact artifact
- `ModelArtifactService` returns a read-only, scope- and host-bound lease with a
  redacted driver materialization
- `ServingEndpointService` turns a redacted driver observation into one
  host-scoped endpoint lease
- `StartServingRequest` carries scope, artifact, serving id, and one absolute
  monotonic deadline
- `OwnedServingHandle` exposes execution-host identity and the safe endpoint
  binding, not inference behavior
- common validation rejects ownership, host, artifact, or service drift before
  process, network, artifact, or endpoint effects

The synthetic owned profile no longer uses `AttachmentService` as a fake model
artifact. It proves the new artifact and endpoint ports, preflight rejection,
substitution rejection, redaction, and owned stop behavior.

## Validation

- 81 focused core, runtime, and testkit tests pass
- focused clippy passes with warnings denied
- full workspace all-target compile passes
- full repository QA passes with 230 tests; the two installed/live probes remain
  separately gated and ignored
- docs QA, Northstar QA, formatting, whitespace, and diff checks pass
- doctor remains at the inherited 19 oversized-file findings, including 7
  errors; the batch adds no finding

## Current State

Cards 060-061 are complete. Card 062 is ready for exact local-host artifact
approvals, digest checks, loopback-only endpoint publication, and expanded
cleanup-order conformance. Kimi cards 058-059 remain blocked by card 057.
