# 034 Consumer Integration Usability Audit

Status: promoted
Owner: Tom
Date: 2026-07-24

## Question

Can applications use Swallowtail's exact version, preflight, access, host, and
lifecycle contracts without rebuilding adapter-internal configuration by hand?

## Evidence

### Nucleus

The first Codex integration reached five failures:

- missing exact executable version
- installed discovery produced no compatible observation
- version probe failed
- missing explicit ambient harness-configuration agreement
- session access policy differed from the immutable plan

The first three are one version-preparation gap with repeated diagnostics.
The fourth is an adapter-fixed app-server requirement copied into consumer
configuration. The fifth comes from an ambient default in
`OpenSessionRequest::new` overriding a read-only preflight choice unless the
consumer repeats the policy.

The current Nucleus worktree repairs these failures by adding:

- direct executable selection and canonicalization
- installed-version discovery
- exact version promotion into instance and requirements
- ambient configuration posture in instance and requirements
- the same read-only policy in preflight and session open

The repair needs 729 lines across Nucleus's Codex host, discovery, and
preflight helpers. It works, but the consumer owns too much assembly.

### Soundcheck

Soundcheck carries another 389 lines of Codex host and preflight construction.
Its current application compiles against Swallowtail, but its catalogue and
structured-run plans do not bind the exact executable version or migrated
harness-configuration posture. Those omissions are runtime failures, not Rust
type errors.

This disproves compile-only consumer validation as sufficient release
evidence.

### Local Runtime

The approved local Codex target resolves to a direct `0.145.0` executable.
Nucleus's gated installed-executable observation passes. Focused Nucleus
preflight tests and Swallowtail version, request-policy, and parser tests pass.

The installed harness is healthy. The failure is integration construction and
diagnostic quality.

## Findings

1. Exact version observation, immutable preflight agreement, explicit access,
   and explicit configuration posture remain required.
2. Consumers should choose policy and authority once, not copy adapter-fixed
   facts through several records.
3. Runtime requests should derive immutable plan echoes from the plan or
   require them explicitly. An unrelated default must not create a guaranteed
   mismatch.
4. Installed discovery needs safe typed failure stages. `Failed` cannot hide
   whether target resolution, spawn, output, exit, parsing, classification, or
   cleanup failed.
5. Access status must come from observation or an explicit caller assertion.
   Consumer helpers must not manufacture `Ready`, `Available`, and `Allowed`.
6. The local host crate lacks a reusable scoped task and service bundle, so
   both consumers implement one.
7. The low-level core remains useful for advanced integrations and
   conformance. It should not be the only application-facing path.
8. The frozen `0.1.0` candidate is not publication-ready because its consumer
   proof checks compilation but not deterministic runtime preparation.

## Recommendation

Add a layered integration surface inside the existing package graph:

- provider-neutral plan-derived request construction in
  `swallowtail-runtime`
- safe typed preparation and discovery diagnostics
- a joined per-host service bundle in `swallowtail-host-local`
- provider-specific prepared Codex exec and app-server facades in
  `swallowtail-adapter-codex`
- named inspectable read-only, bounded-workspace, catalogue, and structured-run
  profiles

Preparation performs exact discovery, classification, configuration,
requirements construction, preflight, and request-policy derivation. Consumers
still choose driver, host target, access evidence, model, working resource,
tools, reasoning, network, search, and writable authority.

Do not add an umbrella crate, generic `send_prompt`, global executor, implicit
provider selection, credential discovery, model fallback, or topology
fallback.

## Promotion

- Contract 037 carries the durable preparation, plan-derivation, access-
  provenance, diagnostic, host-composition, and low-level escape-hatch rules.
- Spec 005 is archived as the promoted provisional realization plan.
- Roadmap g02.002 implements the shared boundary.
- Roadmap g02.003 realizes the Codex prepared facade.
- Roadmaps g02.004 and g02.005 migrate Nucleus and Soundcheck under their own
  repository authority.
- Roadmap g02.006 replaces and requalifies the unpublished release candidate.
