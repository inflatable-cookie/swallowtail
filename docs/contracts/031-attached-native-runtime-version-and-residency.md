# 031 Attached Native Runtime Version And Residency

Status: active
Owner: Tom
Updated: 2026-07-23

## Purpose

Attach to a user-managed native model runtime without taking ownership of its
installation, models, serving process, or resident-state policy. Make exact
runtime version, installed and running model observations, and
inference-caused residency visible before work.

## Identity Boundary

The serving driver, configured runtime instance, runtime executable version,
native API revision, endpoint, model tag, model manifest digest, underlying
artifact, loaded state, protocol facade, and configured model route remain
separate identities.

A model tag is not an artifact digest. Catalogue presence is not route
selection. Running-state observation is not ownership. A native API does not
inherit OpenAI, Anthropic, or another provider's semantics.

## Attach-Only Authority

The configured instance is `ExternalAttached`. The host approves one exact
endpoint and topology. Local unauthenticated access is an explicit access
profile, not missing authorization. A remote or credentialed deployment needs
its own access profile and qualification.

The driver may:

- observe the exact runtime version
- observe bounded installed and running model inventory
- observe bounded details for one explicitly selected model
- invoke one preflight-bound model route through the selected native facade

The driver may not install, update, sign in, pull, push, copy, create, delete,
or unload models; change templates or parameters; stop or restart the service;
or mutate runtime administration. Discovery has no acquisition or deployment
side effect.

## Version Qualification

The runtime's exact observed version is a required configured-instance
binding. Contract 029 governs its maintained compatibility window. Discovery
does not accept `latest`, an ambient executable version, or a server newer than
the latest-qualified boundary.

A closed range needs maintained upstream compatibility evidence plus frozen
corpora at the baseline, latest-qualified boundary, both sides of every
behavior milestone, every exclusion, and representative interior points.
Stable release labels and semantic ordering do not prove compatibility alone.
Prereleases remain excluded unless separately claimed and tested.

Runtime version, native API behavior, model manifest requirements, model
capabilities, and adapter release remain separate version axes. A compatible
runtime cannot make an incompatible model route valid.

## Catalogue Truth

Installed inventory, running inventory, and model-detail observations retain
their source operation, configured instance, runtime version, observation
time, model tag, and safe manifest evidence.

Observation time comes from the execution host catalogue clock under Contract
010. Runtime `modified_at` and residency expiry fields are provider-reported
model state, not substitutes for observation time.

Installed means only that the runtime reported a local model entry. Running
means only that the runtime reported resident state at observation time.
Neither proves invocation readiness, memory capacity, hardware fit, context
length, tool behavior, reasoning behavior, or entitlement. Model-detail
capabilities remain mutable, source-scoped observations under Contracts 007
and 020.

Catalogue bounds reject overflow or structural drift. Raw manifests, host
paths, templates, licenses, and provider payloads stay outside public
diagnostics. A safe digest may identify the observed manifest without exposing
its contents.

## Residency Truth

Inference against an attached runtime may load an installed model, refresh its
retention timer, evict another model, or fail for capacity. This is an
invocation side effect even when the driver does not own serving lifecycle.

Preflight and request policy must agree on one explicit residency posture. The
first posture accepts runtime-managed residency caused by the selected
inference request. It grants no unload, eviction, restoration, duration, or
exclusive-capacity authority.

Close joins Swallowtail-owned network and task work. It does not stop the
runtime, unload the model, restore a prior resident set, or claim that the
runtime released compute. Applications decide whether that posture is
acceptable before execution.

## Native Streaming

Native newline-delimited JSON is its own transport family. HTTP success does
not imply model success. The driver preserves ordered output and terminal
usage, rejects unknown semantic records under its frozen corpus, and maps an
error record after stream start to provider failure.

One direct structured run produces one inference attempt. Cancellation and
deadline stop and join local work without claiming remote compute stop.
Redirect, retry, endpoint fallback, model substitution, compatible-facade
fallback, and automatic repair are prohibited.

## First Ollama Mapping

The first driver may publish this claim only after the required corpus and
conformance pass. Its qualification target binds:

- native Ollama API
- semantic runtime versions `0.14.0` through `0.32.1`, inclusive
- qualification points `0.14.0`, `0.18.0`, `0.30.0`, and `0.32.1`
- one maintained text-only behavior segment
- exact `/api/version` observation
- bounded `/api/tags`, `/api/ps`, and `/api/show`
- one host-approved loopback endpoint and local unauthenticated access profile
- one exact operator-selected installed model tag and observed digest
- one resource-free text structured run through streaming `/api/chat`
- one explicit positive output bound and one provider attempt
- runtime-managed residency acceptance

The driver sends no tool, thinking, vision, format, embedding, generation,
keep-alive administration, or compatible-facade fields. It uses no Ollama
Cloud route, sign-in state, cloud model, API key, model mutation, or owned
server lifecycle.

`0.32.2`, `0.32.3-rc0`, other prereleases, older versions, and unknown newer
versions fail compatibility preflight. A later claim revision may extend the
window only after corpus and conformance qualification.

## Conformance

Deterministic default QA proves:

- exact instance, endpoint, topology, access profile, runtime version, route,
  model tag, digest, output bound, and residency posture
- baseline, latest-qualified, and intermediate qualification points
- rejection below baseline, above latest, prerelease, malformed, substituted,
  and stale-claim versions
- bounded installed, running, and model-detail observations without implicit
  route construction
- one native streaming attempt, ordered output and usage, mid-stream error,
  malformed record, disconnect, cancellation, deadline, and joined cleanup
- no credential, process, artifact, mutation, administration, retry, fallback,
  unload, or restoration effect
- safe diagnostics without endpoint, path, manifest, prompt, output, or raw
  payload leakage

Local and remote-authoritative host identities exercise the same public driver
seam. An optional installed-runtime probe may check exact version and
catalogue shape. Live model inference remains separately gated.

## Acceptance

- attached runtime ownership remains external
- exact observed version and maintained range remain distinct
- the first range spans only tested stable releases and selected behavior
- installed, running, detailed, routed, and artifact identities remain
  separate
- inference-caused residency is explicit and grants no administration
- native NDJSON does not become a generic prompt API
- no container, model acquisition, or Monkey responsibility enters the proof
