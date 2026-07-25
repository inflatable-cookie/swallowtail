# Prepared Integration Contract And Architecture

Date: 2026-07-24

## Outcome

Card 005 is complete. Contract 037 promotes the prepared consumer-integration
boundary before implementation.

Swallowtail now has a durable two-layer model:

- low-level provider-neutral records, host services, preflight, and operation
  roles remain public
- provider-specific prepared integrations bind adapter-owned facts and derive
  immutable plan echoes

The first implementation remains inside the existing runtime, local-host, and
Codex adapter crates. No umbrella crate or generic prompt surface was added.

## Durable Rules

- preparation receives one explicit driver, host, approved target, access
  evidence, and named operation profile
- provider, model, credential, endpoint, billing, topology, writable access,
  network, search, tools, prompts, and workflows never become implicit
- low-level requests carry plan echoes explicitly; prepared requests derive
  them from the immutable plan
- access state is observed or visibly caller-asserted and is never promoted to
  ready, available, or allowed by convenience code
- target, spawn, bounded-output, exit, parse, compatibility, access, preflight,
  and cleanup failures remain machine-distinct and safely formatted
- local host composition stays per-host and joined, with no global executor or
  detached task
- Codex exec, app-server catalogue, and app-server session lifecycles remain
  separate
- sandboxing remains an opt-in provider or host capability

Contracts 008-010, 032-033, and 036 now carry the matching narrow rules.
System architecture records the contracted dependency direction and labels the
layer as not yet realized.

## Promotion And Candidate State

Research 034 is promoted. Spec 005 is archived with its promotion record.

The first non-published `0.1.0` candidate and its consumer handoffs are marked
superseded. Frozen source, package, checksum, handoff, and validation evidence
remain intact. No candidate artifact or external release state changed.

Replacement evidence must execute deterministic, credential-free preparation
through the packaged normal path. Compile-only consumer checks are
insufficient.

## Validation

- `effigy qa:docs` passes
- `effigy qa:northstar` passes
- `git diff --check` passes
- dependency-direction and authority audit finds no new crate, provider-
  specific shared type, hidden route, or consumer ownership
- `effigy doctor` remains at the inherited 19 oversized-file findings: 12
  warnings and seven errors

## Continuation

Card 006 is ready. It owns plan-derived runtime requests, typed preparation
diagnostics, provider-neutral fixtures, and the clean pre-release removal of
the implicit request-policy footgun.

Cards 007-010 remain in bounds behind that surface. Consumer repositories and
release mutations remain out of scope.
