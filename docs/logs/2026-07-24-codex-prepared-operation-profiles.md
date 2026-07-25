# Codex Prepared Operation Profiles

Date: 2026-07-24

## Outcome

Card 009 is complete. Card 010 is ready.

`CodexPreparedIntegration` now exposes four named preparation paths:

- app-server catalogue discovery
- app-server read-only session
- app-server bounded-workspace session
- exec structured run

They do not converge into a generic prompt surface. Every prepared value
retains safe exact-version and access-provenance evidence, an inspectable
immutable preflight plan, and the runtime request derived from that plan.

## Authority

The consumer still chooses the exact model route, model, working resource,
content, writable authority, network, search, reasoning, schema, attachment,
tools, and supported deadline. The adapter derives only Codex-owned facts and
plan echoes.

Read-only sessions use provider-enforced read-only isolation, approval never,
and denied network and search. Bounded workspace is a separate version-gated
path. Structured exec keeps exact-version provider-retention and ambient-
configuration behavior private.

Unsupported exec tools and app-server session deadlines fail during
preparation. Dynamic session tools are bounded, name-unique, and validated as
inline JSON Schema. Resume derives the original agreement and cannot redeclare
those tools.

## Evidence

- prepared catalogue and read-only sessions execute through the existing app-
  server driver
- prepared bounded-workspace sessions preserve their writable sandbox and
  reject versions before workspace-root support
- prepared structured runs execute through the existing exec driver across
  legacy, current, and unverified-newer versions
- driver substitution, target drift, access mismatch, unsupported inputs, and
  missing host services fail before provider effects
- reasoning, one image attachment, JSON Schema output, host-approved search,
  and deadlines add only their exact capabilities and host requirements
- local and remote-authoritative host identities remain exact
- the public example compiles
- 86 Codex tests pass, including 13 prepared-facade tests
- Codex all-target check and warnings-denied clippy pass
- workspace check, lint, and all 23 public-package API gates pass
- Effigy doctor remains at the inherited 19 findings: 12 warnings and 7 errors

## API Classification

This is additive pre-1.0 public API. Existing low-level discovery, structured-
run, catalogue, and interactive-session roles remain usable. The public API
baseline refresh belongs to the superseded unreleased `0.1.0` candidate.

## Continuation

Card 010 owns the complete deterministic conformance matrix, public getting-
started and escape-hatch guidance, and exact Nucleus and Soundcheck migration
inputs. It must not edit either consumer repository.

Publication, tagging, pushing, releases, workflows, registries, and consumer
mutation remain out of scope.
